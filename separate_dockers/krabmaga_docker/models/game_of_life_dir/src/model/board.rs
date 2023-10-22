use core::fmt;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::any::Any;
use std::hash::Hash;
use std::hash::Hasher;


use crate::model::fake_agent::FakeAgent;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Status {
    Dead,
    Alive,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::Dead => write!(f, "Dead"),
            Status::Alive => write!(f, "Alive"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub id: i32,
    pub status: Status,
}

impl Hash for Cell {
    fn hash<H>(&self, state: &mut H)
        where
            H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Eq for Cell {}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} status {}", self.id, self.status)
    }
}

pub struct Board {
    pub step: u64,
    pub dim: i32,
    pub field: DenseGrid2D<Cell>,
    pub initial_state: Vec<Vec<bool>>,
}

impl Board {
    pub fn new(initial_map: Vec<Vec<bool>>) -> Board{
        let dim = initial_map.len() as i32;
        let mut field = DenseGrid2D::new(dim, dim);
        Board{
            step: 0,
            dim,
            field: field,
            initial_state: initial_map,
        }
    }

    pub fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    #[allow(dead_code)]
    pub fn as_state(&self) -> &dyn State {
        self
    }
}

impl State for Board {
    fn init(&mut self, schedule: &mut Schedule) {
        // initialize the board by filling the field with cells based on initial_state
        let mut id = 0;
        for row in self.initial_state.iter() {
            for cell in row.iter() {
                let status = if *cell {
                    Status::Alive
                } else {
                    Status::Dead
                };
                self.field.set_object_location(Cell {
                    id,
                    status,
                },
                &Int2D {x: id % self.dim, y: id / self.dim});
                id += 1;
            }
        }
        let fake_agent = FakeAgent { id: 0 };
        schedule.schedule_repeating(Box::new(fake_agent), 0., 1);
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }

    fn reset(&mut self) {
        self.step = 0;
        self.field = DenseGrid2D::new(self.dim, self.dim);
    }

    fn update(&mut self, step: u64) {
        self.step = step;
        let mut updates = Vec::new();
        for x in 0..self.field.width {
            let mut row_updates = Vec::new();
            for y in 0..self.field.height {
                // get the cell
                let loc = Int2D { x, y };
                let value = match self.field.get_objects(&loc) {
                    Some(t) => t[0],
                    None => {
                        continue;
                    }
                };
                // if I am DEAD
                if value.status == Status::Dead {
                    let mut alive_neighbors = 0;
                    for i in -1..2 {
                        for j in -1..2 {
                            if i == 0 && j == 0 {
                                continue;
                            }
                            let loc_n = Int2D {
                                x: (x + i + self.dim) % self.dim,
                                y: (y + j + self.dim) % self.dim,
                            };
                            let neighbor = match self.field.get_objects(&loc_n) {
                                Some(t) => t[0],
                                None => {
                                    continue;
                                }
                            };
                            if neighbor.status == Status::Alive {
                                alive_neighbors += 1;
                            }
                        }
                    }
                    if alive_neighbors == 3 {
                        row_updates.push(true);
                    } else {
                        row_updates.push(false);
                    }
                } else {
                    // if I am ALIVE
                    let mut alive_neighbors = 0;
                    for i in -1..2 {
                        for j in -1..2 {
                            if i == 0 && j == 0 {
                                continue;
                            }
                            let loc_n = Int2D {
                                x: (x + i + self.dim) % self.dim,
                                y: (y + j + self.dim) % self.dim,
                            };
                            let neighbor = match self.field.get_objects(&loc_n) {
                                Some(t) => t[0],
                                None => {
                                    continue;
                                }
                            };
                            if neighbor.status == Status::Alive {
                                alive_neighbors += 1;
                            }
                        }
                    }
                    if alive_neighbors < 2 || alive_neighbors > 3 {
                        row_updates.push(false);
                    } else {
                        row_updates.push(true);
                    }
                }
            }
            updates.push(row_updates);
        }

        // apply updates to cells
        let mut id = 0;
        self.field = DenseGrid2D::new(self.dim, self.dim);
        for row in self.initial_state.iter() {
            for cell in row.iter() {
                let status = if *cell {
                    Status::Alive
                } else {
                    Status::Dead
                };
                self.field.set_object_location(Cell {
                    id,
                    status,
                },
                   &Int2D {x: id % self.dim, y: id / self.dim});
                id += 1;
            }
        }
    }
}