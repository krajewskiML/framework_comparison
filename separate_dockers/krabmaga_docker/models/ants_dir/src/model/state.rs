use crate::model::ant::{Ant, AntType};
use crate::model::constants::{
    FOOD_RADIUS, FOOD_VALUE, NEW_FOOD_FREQUENCY, WALL_PHEROMONE_VALUE
};
use crate::model::to_food_grid::{ToFoodGrid, ToFoodGridBlack, ToFoodGridRed};
use crate::model::to_home_grid::{ToHomeGrid, ToHomeGridBlack, ToHomeGridRed};
use array2d::Array2D;
use core::fmt;
use core::hash::{Hash, Hasher};
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::any::Any;
use std::sync::RwLock;

// Objects within the field
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ItemType {
    Food(i8),
    Home,
    Obstacle,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ItemType::Food(_) => write!(f, "Food"),
            ItemType::Home => write!(f, "Home"),
            ItemType::Obstacle => write!(f, "Obstacle"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Item {
    pub id: u32,
    pub value: ItemType,
}

impl Hash for Item {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Eq for Item {}

impl PartialEq for Item {
    fn eq(&self, other: &Item) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} value {}", self.id, self.value)
    }
}

// The global simulation state. This holds the various grids used for movement, exposing setter methods
// so that the state itself will worry about ownership rules by mutating its own fields.
pub struct ModelState {
    pub size: i32,
    pub ants_grid: SparseGrid2D<Ant>,
    pub obstacles_grid: SparseGrid2D<Item>,
    pub to_food_grid_black: ToFoodGridBlack,
    pub to_home_grid_black: ToHomeGridBlack,
    pub to_food_grid_red: ToFoodGridRed,
    pub to_home_grid_red: ToHomeGridRed,
    pub step: u64,
    pub item_counter: u32,
    pub black_anthill_position: Int2D,
    pub red_anthill_position: Int2D,
    pub ants_per_type: u32
}

impl ModelState {
    pub(crate) fn new(size: i32) -> ModelState {
        let black_anthill_position = Int2D {
            x: ((size as f32) * 0.1) as i32,
            y: ((size as f32) * 0.1) as i32,
        };
        let red_anthill_position = Int2D {
            x: ((size as f32) * 0.9) as i32,
            y: ((size as f32) * 0.9) as i32,
        };
        let ants_per_type = (size / 2) as u32;
        ModelState {
            size,
            ants_grid: SparseGrid2D::new(size, size),
            obstacles_grid: SparseGrid2D::new(size, size),
            to_food_grid_black: ToFoodGridBlack::new(size, size),
            to_home_grid_black: ToHomeGridBlack::new(size, size, black_anthill_position),
            to_food_grid_red: ToFoodGridRed::new(size, size),
            to_home_grid_red: ToHomeGridRed::new(size, size, red_anthill_position),
            step: 0,
            item_counter: 0,
            black_anthill_position,
            red_anthill_position,
            ants_per_type
        }
    }

    fn check_if_value(&self) {
        // grab value from all grids and check if they are not 0
        let mid_x = self.size / 2;
        let mid_y = self.size / 2;
        let mut val;
        //to_food_grid_black
        val = self.to_food_grid_black.grid.grid.get_value(&Int2D { x: mid_x, y: mid_y });
        match val {
            Some(value) => {
                if value == 0. {
                    println!("to_food_grid_black is 0");
                }
            }
            None => panic!("to_food_grid_black is None"),
        }
        //to_home_grid_black
        val = self.to_home_grid_black.grid.grid.get_value(&Int2D { x: mid_x, y: mid_y });
        match val {
            Some(value) => {
                if value == 0. {
                    println!("to_home_grid_black is 0");
                }
            }
            None => panic!("to_home_grid_black is None"),
        }
        //to_food_grid_red
        val = self.to_food_grid_red.grid.grid.get_value(&Int2D { x: mid_x, y: mid_y });
        match val {
            Some(value) => {
                if value == 0. {
                    println!("to_food_grid_red is 0");
                }
            }
            None => panic!("to_food_grid_red is None"),
        }
        //to_home_grid_red
        val = self.to_home_grid_red.grid.grid.get_value(&Int2D { x: mid_x, y: mid_y });
        match val {
            Some(value) => {
                if value == 0. {
                    println!("to_home_grid_red is 0");
                }
            }
            None => panic!("to_home_grid_red is None"),
        }
    }

    pub fn calculate_static_field_red(&mut self) {
        // to home red
        let mut new_grid = Array2D::filled_with(
            WALL_PHEROMONE_VALUE,
            self.to_home_grid_red.grid.grid.width as usize,
            self.to_home_grid_red.grid.grid.height as usize,
        );
        let mut visited = Array2D::filled_with(
            false,
            self.to_home_grid_red.grid.grid.width as usize,
            self.to_home_grid_red.grid.grid.height as usize,
        );
        let mut queue = Vec::new();
        let mut current = self.to_home_grid_red.grid.anthill_position.clone();
        queue.push(current.clone());
        new_grid
            .set(current.x as usize, current.y as usize, 0.)
            .unwrap();
        visited
            .set(current.x as usize, current.y as usize, true)
            .unwrap();
        while !queue.is_empty() {
            current = queue.remove(0);
            let mut neighbours = Vec::new();
            neighbours.push(Int2D {
                x: current.x + 1,
                y: current.y,
            });
            //neighbours.push(Int2D { x: current.x + 1, y: current.y +1 });
            //neighbours.push(Int2D { x: current.x - 1, y: current.y -1 });
            neighbours.push(Int2D {
                x: current.x - 1,
                y: current.y,
            });
            //neighbours.push(Int2D { x: current.x - 1, y: current.y + 1 });
            neighbours.push(Int2D {
                x: current.x,
                y: current.y + 1,
            });
            //neighbours.push(Int2D { x: current.x + 1, y: current.y - 1 });
            neighbours.push(Int2D {
                x: current.x,
                y: current.y - 1,
            });
            for neighbour in neighbours {
                if !self.check_wall(&neighbour) {
                    if !visited
                        .get(neighbour.x as usize, neighbour.y as usize)
                        .unwrap()
                    {
                        let value = new_grid
                            .get(current.x as usize, current.y as usize)
                            .unwrap();
                        new_grid
                            .set(neighbour.x as usize, neighbour.y as usize, value + 1.)
                            .unwrap();
                        visited
                            .set(neighbour.x as usize, neighbour.y as usize, true)
                            .unwrap();
                        queue.push(neighbour.clone());
                    }
                }
            }
        }
        for x in 0..self.to_home_grid_red.grid.grid.width {
            for y in 0..self.to_home_grid_red.grid.grid.height {
                let value = new_grid.get(x as usize, y as usize).unwrap();
                self.to_home_grid_red
                    .grid
                    .grid
                    .set_value_location(*value, &Int2D { x, y });
            }
        }

        self.to_home_grid_red.grid.grid.update();
    }

    pub fn calculate_static_field_black(&mut self) {
        // to home red
        let mut new_grid = Array2D::filled_with(
            WALL_PHEROMONE_VALUE,
            self.to_home_grid_black.grid.grid.width as usize,
            self.to_home_grid_black.grid.grid.height as usize,
        );
        let mut visited = Array2D::filled_with(
            false,
            self.to_home_grid_black.grid.grid.width as usize,
            self.to_home_grid_black.grid.grid.height as usize,
        );
        let mut queue = Vec::new();
        let mut current = self.to_home_grid_black.grid.anthill_position.clone();
        queue.push(current.clone());
        new_grid
            .set(current.x as usize, current.y as usize, 0.)
            .unwrap();
        visited
            .set(current.x as usize, current.y as usize, true)
            .unwrap();
        while !queue.is_empty() {
            current = queue.remove(0);
            let mut neighbours = Vec::new();
            neighbours.push(Int2D {
                x: current.x + 1,
                y: current.y,
            });
            //neighbours.push(Int2D { x: current.x + 1, y: current.y +1 });
            //neighbours.push(Int2D { x: current.x - 1, y: current.y -1 });
            neighbours.push(Int2D {
                x: current.x - 1,
                y: current.y,
            });
            //neighbours.push(Int2D { x: current.x - 1, y: current.y + 1 });
            neighbours.push(Int2D {
                x: current.x,
                y: current.y + 1,
            });
            //neighbours.push(Int2D { x: current.x + 1, y: current.y - 1 });
            neighbours.push(Int2D {
                x: current.x,
                y: current.y - 1,
            });
            for neighbour in neighbours {
                if !self.check_wall(&neighbour) {
                    if !visited
                        .get(neighbour.x as usize, neighbour.y as usize)
                        .unwrap()
                    {
                        let value = new_grid
                            .get(current.x as usize, current.y as usize)
                            .unwrap();
                        new_grid
                            .set(neighbour.x as usize, neighbour.y as usize, value + 1.)
                            .unwrap();
                        visited
                            .set(neighbour.x as usize, neighbour.y as usize, true)
                            .unwrap();
                        queue.push(neighbour.clone());
                    }
                }
            }
        }
        for x in 0..self.to_home_grid_black.grid.grid.width {
            for y in 0..self.to_home_grid_black.grid.grid.height {
                let value = new_grid.get(x as usize, y as usize).unwrap();
                self.to_home_grid_black
                    .grid
                    .grid
                    .set_value_location(*value, &Int2D { x, y });
            }
        }

        self.to_home_grid_black.grid.grid.update();
    }

    // Check if a particular grid cell has an obstacle or not. Will return None if the grid cell holds no obstacle.
    pub fn check_wall(&self, loc: &Int2D) -> bool {
        match self.obstacles_grid.get_objects(loc) {
            Some(vec) => {
                if vec.first().unwrap().value == ItemType::Obstacle {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
    // Check if a particular grid cell has a food or not. Will return None if the grid cell holds no obstacle.
    pub fn get_food(&self, loc: &Int2D) -> Option<Item> {
        match self.obstacles_grid.get_objects(loc) {
            Some(vec) => match vec.first().unwrap().value {
                ItemType::Food(_) => Some(vec.first().unwrap().clone()),
                _ => None,
            },
            None => None,
        }
    }

    // function that creates a wall of obstacles given x, y, width and height
    pub fn create_obstacle(&mut self, x: i32, y: i32, width: i32, height: i32) {
        for i in x..x + width {
            for j in y..y + height {
                self.add_obstacle(i, j);
            }
        }
    }

    // function that creates am obstacle at a given x, y
    pub fn add_obstacle(&mut self, x: i32, y: i32) {
        let loc = Int2D { x, y };
        let item = Item {
            id: self.item_counter,
            value: ItemType::Obstacle,
        };
        self.item_counter += 1;
        self.obstacles_grid.set_object_location(item, &loc);
    }

    // decreses food value in a cell if it is equal to 0 then it is removed from the grid
    pub fn decrease_food(&mut self, loc: &Int2D) {
        let obstacles = self.obstacles_grid.get_objects(loc).unwrap();
        // get food from the cell
        let food = obstacles.first().unwrap();
        // get food value
        let food_value = match food.value {
            ItemType::Food(value) => value,
            _ => panic!("Food value is not correct"),
        };
        // remove food from the cell
        self.obstacles_grid.remove_object_location(*food, loc);
        if food_value > 1 {
            // decrease food value
            let new_food = Item {
                id: food.id,
                value: ItemType::Food(food_value - 1),
            };
            // add new food to the cell
            self.obstacles_grid.set_object_location(new_food, loc);
        }
        self.obstacles_grid.update();
    }

    pub fn generate_food(&mut self) {
        // Food generation
        let mut rng = rand::thread_rng();
        let radius: i32 = rng.gen_range(FOOD_RADIUS);
        let x = rng.gen_range(0..self.size);
        let y = rng.gen_range(0..self.size);

        let food_location = Int2D { x, y };

        for i in -radius..=radius {
            for j in -radius..=radius {
                // check if coordinates will create a circle
                if !(i * i + j * j <= radius * radius) {
                    continue;
                }
                let loc = Int2D {
                    x: food_location.x + i,
                    y: food_location.y + j,
                };
                if loc.x >= 0 && loc.x < self.size && loc.y >= 0 && loc.y < self.size {
                    self.obstacles_grid.set_object_location(
                        Item {
                            id: self.item_counter,
                            value: ItemType::Food(FOOD_VALUE),
                        },
                        &loc,
                    );
                    self.item_counter += 1;
                }
            }
        }
    }
    // function that removes pheromon from the grid where wall is located
    fn remove_pheromon_from_walls(&mut self) {
        for x in 0..self.size {
            for y in 0..self.size {
                let loc = Int2D { x, y };
                if self.check_wall(&loc) {
                    self.to_food_grid_black
                        .grid
                        .grid
                        .set_value_location(0., &loc);
                    self.to_food_grid_red.grid.grid.set_value_location(0., &loc);
                }
            }
        }
    }
}

impl State for ModelState {
    fn init(&mut self, schedule: &mut Schedule) {
        self.step = 0;
        self.ants_grid = SparseGrid2D::new(self.size, self.size);
        self.obstacles_grid = SparseGrid2D::new(self.size, self.size);
        self.to_food_grid_black = ToFoodGridBlack::new(self.size, self.size);
        self.to_home_grid_black = ToHomeGridBlack::new(self.size, self.size, self.black_anthill_position);
        self.to_food_grid_red = ToFoodGridRed::new(self.size, self.size);
        self.to_home_grid_red = ToHomeGridRed::new(self.size, self.size, self.red_anthill_position);


        // generate red ants and black ants that starts from corresponding anthill
        for i in 0..self.ants_per_type {
            let position = Int2D {
                x: self.red_anthill_position.x,
                y: self.red_anthill_position.y,
            };
            let ant = Ant::new(i, position, AntType::RED, false);
            self.ants_grid.set_object_location(ant, &position);
            schedule.schedule_repeating(Box::new(ant), 0., 0);
        }

        for i in 0..self.ants_per_type {
            let position = Int2D {
                x: self.black_anthill_position.x,
                y: self.black_anthill_position.y,
            };
            let ant = Ant::new(i + self.ants_per_type, position, AntType::BLACK, false);
            self.ants_grid.set_object_location(ant, &position);
            schedule.schedule_repeating(Box::new(ant), 0., 0);
        }

        // generate obstacles
        self.create_obstacle(0, 0, 1, self.size);
        self.create_obstacle(0, 0, self.size, 1);
        self.create_obstacle(self.size - 1, 0, 1, self.size);
        self.create_obstacle(0, self.size - 1, self.size, 1);

        self.create_obstacle(40, 30, 40, 10);

        // create anthills in positions of anthills and add them to the grid as item type home
        self.obstacles_grid.set_object_location(
            Item {
                id: self.item_counter,
                value: ItemType::Home,
            },
            &self.red_anthill_position,
        );
        self.item_counter += 1;
        self.obstacles_grid.set_object_location(
            Item {
                id: self.item_counter,
                value: ItemType::Home,
            },
            &self.black_anthill_position,
        );
        self.item_counter += 1;

        self.obstacles_grid.update();

        // calculate static fields
        self.calculate_static_field_red();
        self.calculate_static_field_black();

        //self.check_if_value();

        // remove pheromon from the grid where wall is located
        self.remove_pheromon_from_walls();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
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
        self.item_counter = 0;
        self.obstacles_grid = SparseGrid2D::new(self.size, self.size);
        self.ants_grid = SparseGrid2D::new(self.size, self.size);
        self.to_food_grid_black = ToFoodGridBlack::new(self.size, self.size);
        self.to_home_grid_black = ToHomeGridBlack::new(self.size, self.size, self.black_anthill_position);
        self.to_food_grid_red = ToFoodGridRed::new(self.size, self.size);
        self.to_home_grid_red = ToHomeGridRed::new(self.size, self.size, self.red_anthill_position);
    }

    fn update(&mut self, step: u64) {
        if step % NEW_FOOD_FREQUENCY == 0 {
            self.generate_food();
        }
        self.ants_grid.update();
        self.to_food_grid_black.update();
        self.to_home_grid_black.update();
        self.to_food_grid_red.update();
        self.to_home_grid_red.update();
        self.step = step;
        self.obstacles_grid.update();
        //self.check_if_value();
    }
}
