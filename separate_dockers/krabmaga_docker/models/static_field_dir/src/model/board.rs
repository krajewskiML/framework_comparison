use krabmaga::engine::fields::field::Field;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use std::any::Any;
use krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;

use std::collections::VecDeque;
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;

use crate::model::constants::{ESCAPE_DOOR, MAX_VALUE, PERSON, WALL, DIRECTIONS};


use crate::model::agent::Person;

pub struct Board {
    pub step: u64,
    pub rows: i32,
    pub cols: i32,
    pub field: DenseNumberGrid2D<i16>,
    pub agents: SparseGrid2D<Person>,
    pub initial_state: Vec<Vec<i8>>,
    pub escaped: i32,
    pub initial_person_count: i32,
}

impl Board {
    pub fn new(initial_map: Vec<Vec<i8>>) -> Board{
        let rows = initial_map.len() as i32;
        let cols = initial_map[0].len() as i32;
        let field = Board::compute_static_field(initial_map.clone());
        let agents = SparseGrid2D::new(rows, cols);
        Board{
            step: 0,
            rows,
            cols,
            field,
            agents,
            initial_state: initial_map,
            escaped: 0,
            initial_person_count: 0,
        }
    }
    fn compute_static_field(map: Vec<Vec<i8>>) -> DenseNumberGrid2D<i16> {
        let rows = map.len();
        let cols = map[0].len();

        let mut static_field = vec![vec![MAX_VALUE; cols]; rows];
        let mut queue = VecDeque::new();

        // Initialize the static field with escape doors and add them to the queue
        for i in 0..rows {
            for j in 0..cols {
                if map[i][j] == ESCAPE_DOOR {
                    //println!("Escape door at {} {}", i, j);
                    static_field[i][j] = 0;
                    queue.push_back((i, j));
                }
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            for &(dx, dy) in &DIRECTIONS {
                let new_x = (x as isize + dx as isize) as usize;
                let new_y = (y as isize + dy as isize) as usize;

                if new_x < rows && new_y < cols && map[new_x][new_y] != WALL && static_field[new_x][new_y] == MAX_VALUE {
                    static_field[new_x][new_y] = static_field[x][y] + 1;
                    queue.push_back((new_x, new_y));
                }
            }
        }

        // put static field into krabmaga field
        let mut krabmaga_static_field = DenseNumberGrid2D::new(rows as i32, cols as i32);
        for i in 0..rows {
            for j in 0..cols {
                krabmaga_static_field.set_value_location( static_field[i][j], &Int2D {x: i as i32, y: j as i32},);
            }
        }
        krabmaga_static_field.update();
        krabmaga_static_field
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
        // initialize the board by filling the agents field with agents based on initial_state
        let mut id = 0;
        for x in 0..self.rows {
            for y in 0..self.cols {
                let cell = self.initial_state[x as usize][y as usize];
                if cell == PERSON {
                    let person = Person { id , loc: Int2D {x, y}, escaped: false};
                    self.agents.set_object_location(person, &Int2D {x, y});
                    schedule.schedule_repeating(Box::new(person), 0., 0);
                    id += 1;
                }
            }
        }
        self.initial_person_count = id as i32;
        println!("{} agents", id);
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
    }

    fn update(&mut self, step: u64) {
        // iterate over all fields and check if on any of them there are more than one agent
        // for x in 0..self.rows {
        //     for y in 0..self.cols {
        //         let agents = self.agents.get_objects(&Int2D {x, y});
        //         if agents != None {
        //             let agents = agents.unwrap();
        //             if agents.len() > 1 {
        //                 println!("Collision at {} {}", x, y);
        //             }
        //         }
        //     }
        // }
        // println!("Step {}", step);
        self.agents.update();
        self.step = step;
        // print how many agents are still on the board
        // println!("{} agents left", self.initial_person_count - self.escaped);
        // iterate over the agents and print its location
        //self.agents.iter_objects(|_, agent|{println!("Person is still at field {} at {} {}", agent.id, agent.loc.x, agent.loc.y)});
    }

    fn end_condition(&mut self, schedule: &mut Schedule) -> bool {
        // if agents field is empty then end the simulation
        if self.escaped == self.initial_person_count {
            // print how many steps it took to evacuate
            println!("Evacuation took {} steps", self.step);
            return true;
        }
        return false;
    }
}