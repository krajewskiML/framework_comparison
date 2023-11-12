use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::state::State;
use krabmaga::rand::Rng;
use krabmaga::{rand, thread_rng};
use std::hash::{Hash, Hasher};

use crate::model::constants::{
    PHEROMONE_DROP_DECREASE,
    PHEROMONE_INITIAL_VALUE
};
use crate::model::state::*;

#[derive(Copy, Clone)]
pub enum AntType {
    RED,
    BLACK,
}

pub fn direction_to_int2d(direction: i8) -> Int2D {
    match direction {
        0 => Int2D { x: 0, y: 1 },
        1 => Int2D { x: 1, y: 1 },
        2 => Int2D { x: 1, y: 0 },
        3 => Int2D { x: 1, y: -1 },
        4 => Int2D { x: 0, y: -1 },
        5 => Int2D { x: -1, y: -1 },
        6 => Int2D { x: -1, y: 0 },
        7 => Int2D { x: -1, y: 1 },
        _ => panic!("no such direction"),
    }
}

// function that gets direction index from current position and target position
pub fn get_direction_index(current_position: &Int2D, target_position: &Int2D) -> i8 {
    let mut direction_index = 0;
    let x_diff = target_position.x - current_position.x;
    let y_diff = target_position.y - current_position.y;

    if x_diff == 0 && y_diff == 0 {
        return direction_index;
    }

    if x_diff == 0 {
        if y_diff > 0 {
            direction_index = 0;
        } else {
            direction_index = 4;
        }
    } else if y_diff == 0 {
        if x_diff > 0 {
            direction_index = 2;
        } else {
            direction_index = 6;
        }
    } else {
        if x_diff > 0 {
            if y_diff > 0 {
                direction_index = 1;
            } else {
                direction_index = 3;
            }
        } else {
            if y_diff > 0 {
                direction_index = 7;
            } else {
                direction_index = 5;
            }
        }
    }

    direction_index
}

// function that gets coordinates of the ants field of view
pub fn get_field_of_view(position: Int2D, direction: i8, state_size: i32) -> Vec<Int2D> {
    let mut field_of_view = Vec::new();
    let direction = direction;
    let mut diretion_int_2_d;
    for change in -1..2 {
        let change_direction = match direction + change {
            -2 => 6,
            -1 => 7,
            8 => 0,
            9 => 1,
            _ => direction + change,
        };
        diretion_int_2_d = direction_to_int2d(change_direction);
        let field_of_view_position = Int2D {
            x: position.x + diretion_int_2_d.x,
            y: position.y + diretion_int_2_d.y,
        };
        // check if field of view is in the field
        if field_of_view_position.x >= 0
            && field_of_view_position.x < state_size
            && field_of_view_position.y >= 0
            && field_of_view_position.y < state_size
        {
            field_of_view.push(field_of_view_position);
        }
    }
    field_of_view
}

// A struct representing an ant, with an id, a location, whether it's holding food or not and the
// current reward, used to increase the pheromone on the location of the ant if a site is reached.
#[derive(Copy, Clone)]
pub struct Ant {
    // An unique id.
    pub id: u32,
    // The location of the agent.
    pub loc: Int2D,
    // type of ant
    pub ant_type: AntType,
    // Direction of the ant.
    pub direction: i8,
    // False means the agent will try to find food by following food pheromones if possible, or by
    // flooding the grid until it is found. True means the agent will try to return home by using the
    // previously deposited pheromones.
    pub has_food: bool,
    // Value used to increase the pheromones in the nest and in the food source.
    // This will let the agents spread pheromones in the surrounding areas from point of interests
    // so that other agents will know which path to take to do their job.
    pub pheromone_dropped: f32,
}

impl Ant {
    pub fn new(id: u32, loc: Int2D, ant_type: AntType, has_food: bool) -> Ant {
        let mut rng = thread_rng();
        Ant {
            id,
            loc,
            ant_type,
            direction: rng.gen_range(0..8),
            has_food,
            pheromone_dropped: PHEROMONE_INITIAL_VALUE,
        }
    }

    // Deposit a home pheromone if self is not holding food, else deposit a food pheromone,
    // so that other agents will take in account the pheromone value when choosing the next step's
    // direction.
    pub fn deposit_pheromone(&mut self, state: &ModelState) {}

    // Step to the next cell by taking into account pheromones. If no pheromones of the right type
    // are found in a 3x3 grid centered on us, try to step in the same direction of the last frame
    // with a probability of MOMENTUM_PROBABILITY. Otherwise, step in a random direction with a
    // probability of RANDOM_ACTION_PROBABILITY.
    pub fn act(&mut self, state: &mut ModelState) {
        // println!("Ant {} is at x: {}, y: {}", self.id, self.loc.x, self.loc.y);
        // first we check whether the ant is carrying food or not
        if self.has_food {
            //print to console that it has food and is going to the anthill
            // if the ant is carrying food, it will try to return home
            // we check whether the ant is in the nest or not
            if self.loc
                == match self.ant_type {
                    AntType::RED => state.red_anthill_position,
                    AntType::BLACK => state.black_anthill_position,
                }
            {
                // if the ant is in the nest, it will drop the food and reset its state
                self.has_food = false;
                // print that the ant has dropped the food to the console
                //println!("Ant {} has dropped the food", self.id);
                self.pheromone_dropped = PHEROMONE_INITIAL_VALUE;
                return;
            }
            // if the ant is not in the nest, it will try to find the nest
            // we are going to check the pheromones in the field of view of the ant
            let positions_to_check = get_field_of_view(self.loc, self.direction, state.size);
            // we are going to check the pheromones in the field of view of the ant
            // print the direction of the ant
            //println!("Ant at x: {} y: {} is facing direction {}", self.loc.x, self.loc.y, self.direction);
            let mut min = f32::MAX;
            let mut new_position = self.loc;
            for pos in positions_to_check.iter() {
                // print the checked position
                //println!("Ant at x: {} y: {} is checking position x: {} y: {}", self.loc.x, self.loc.y, pos.x, pos.y);
                // we fetch the pheromone value of the position
                let pheromone = match self.ant_type {
                    AntType::RED => state.to_home_grid_red.grid.grid.get_value(&pos),
                    AntType::BLACK => state.to_home_grid_black.grid.grid.get_value(&pos),
                }
                .unwrap();
                // we check whether the pheromone value is higher than the current maximum
                if pheromone < min {
                    // if the pheromone value is higher than the current maximum, we update the maximum
                    min = pheromone;
                    new_position = *pos;
                }
            }
            // we check whether there is any pheromone in the field of view and if not we randomly walk
            if min == f32::MAX {
                let mut rng = thread_rng();
                let change_in_direction = rng.gen_range(-1..=1);
                let new_direction = self.direction + change_in_direction;
                self.direction = match new_direction {
                    -2 => 6,
                    -1 => 7,
                    8 => 0,
                    9 => 1,
                    _ => new_direction,
                };

                // move ant by 1 using its direction
                let mut direction_2d = direction_to_int2d(self.direction);
                // let's add the direction to current loc
                let mut new_x = (self.loc.x + direction_2d.x);
                let mut new_y = (self.loc.y + direction_2d.y);
                let mut new_loc = Int2D { x: new_x, y: new_y };
                // check if the new location is wall
                if state.check_wall(&new_loc) {
                    // if the new location is wall, we reverse the direction
                    self.direction = match self.direction {
                        0 => 4,
                        1 => 5,
                        2 => 6,
                        3 => 7,
                        4 => 0,
                        5 => 1,
                        6 => 2,
                        7 => 3,
                        _ => self.direction,
                    };
                    direction_2d = direction_to_int2d(self.direction);
                    new_x = (self.loc.x + direction_2d.x);
                    new_y = (self.loc.y + direction_2d.y);
                    new_loc = Int2D { x: new_x, y: new_y };
                }
                self.loc = new_loc;
                state.ants_grid.set_object_location(*self, &new_loc);
            } else {
                // we follow the pheromone trail and move to the new position and change ants direction accordingly
                let mut new_direction = get_direction_index(&self.loc, &new_position);
                self.direction = new_direction;
                // check if the new location is wall
                if state.check_wall(&new_position) {
                    // if the new location is wall, we reverse the direction
                    self.direction = match self.direction {
                        0 => 4,
                        1 => 5,
                        2 => 6,
                        3 => 7,
                        4 => 0,
                        5 => 1,
                        6 => 2,
                        7 => 3,
                        _ => self.direction,
                    };
                    let direction_2d = direction_to_int2d(self.direction);
                    let new_x = (self.loc.x + direction_2d.x);
                    let new_y = (self.loc.y + direction_2d.y);
                    new_position = Int2D { x: new_x, y: new_y };
                }
                self.loc = new_position;
                state.ants_grid.set_object_location(*self, &new_position);
            }
        } else {
            // if the ant is not carrying food, it will try to find food
            // we check if the food is in the field of view of the ant
            let positions_to_check = get_field_of_view(self.loc, self.direction, state.size);
            for pos in positions_to_check.iter() {
                // we check if the food is in the field of view of the ant
                let food = state.get_food(&pos);
                if food == None {
                    continue;
                }
                // if the food is in the field of view of the ant, it will pick up the food and change its state
                self.has_food = true;
                // print that ant has picked up the food to the console
                //println!("Ant {} has picked up the food", self.id);
                state.decrease_food(&pos);
                return;
            }
            // if the food is not in the field of view of the ant, it will try to find the food
            // we are going to check the pheromones in the field of view of the ant
            let mut max = 0.;
            let mut new_position = self.loc;
            for pos in positions_to_check.iter() {
                // we fetch the pheromone value of the position
                let pheromone = match self.ant_type {
                    AntType::RED => state.to_food_grid_red.grid.grid.get_value(&pos),
                    AntType::BLACK => state.to_food_grid_black.grid.grid.get_value(&pos),
                }
                .unwrap_or(0.);
                // if the pheromone value is higher than the visible maximum or lower than the visible minimum, we ignore it
                // we check whether the pheromone value is higher than the current maximum
                if pheromone > max {
                    // if the pheromone value is higher than the current maximum, we update the maximum
                    max = pheromone;
                    new_position = *pos;
                }
            }
            // we check whether there is any pheromone in the field of view and if not we randomly walk
            if max == 0. {
                let mut rng = thread_rng();
                let change_in_direction = rng.gen_range(-1..=1);
                let new_direction = self.direction + change_in_direction;
                self.direction = match new_direction {
                    -2 => 6,
                    -1 => 7,
                    8 => 0,
                    9 => 1,
                    _ => new_direction,
                };
                // move ant by 1 using its direction
                let mut direction_2d = direction_to_int2d(self.direction);
                // let's add the direction to current loc
                let mut new_x = (self.loc.x + direction_2d.x);
                let mut new_y = (self.loc.y + direction_2d.y);
                let mut new_loc = Int2D { x: new_x, y: new_y };
                if state.check_wall(&new_loc) {
                    // if the new location is wall, we reverse the direction
                    self.direction = match self.direction {
                        0 => 4,
                        1 => 5,
                        2 => 6,
                        3 => 7,
                        4 => 0,
                        5 => 1,
                        6 => 2,
                        7 => 3,
                        _ => self.direction,
                    };
                    direction_2d = direction_to_int2d(self.direction);
                    new_x = (self.loc.x + direction_2d.x);
                    new_y = (self.loc.y + direction_2d.y);
                    new_loc = Int2D { x: new_x, y: new_y };
                }
                self.loc = new_loc;
                state.ants_grid.set_object_location(*self, &new_loc);
            } else {
                // we follow the pheromone trail and move to the new position and change ants direction accordingly
                let new_direction = get_direction_index(&self.loc, &new_position);
                if state.check_wall(&new_position) {
                    // if the new location is wall, we reverse the direction
                    self.direction = match self.direction {
                        0 => 4,
                        1 => 5,
                        2 => 6,
                        3 => 7,
                        4 => 0,
                        5 => 1,
                        6 => 2,
                        7 => 3,
                        _ => self.direction,
                    };
                    let direction_2d = direction_to_int2d(self.direction);
                    let new_x = (self.loc.x + direction_2d.x);
                    let new_y = (self.loc.y + direction_2d.y);
                    new_position = Int2D { x: new_x, y: new_y };
                }
                self.direction = new_direction;
                self.loc = new_position;
                state.ants_grid.set_object_location(*self, &new_position);
            }
        }
    }
}

impl Agent for Ant {
    /// Each ant deposits a pheromone in its current location, then it steps in the next grid cell.
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any_mut().downcast_mut::<ModelState>().unwrap();
        // if the ant is carrying food, it will deposit pheromone on the way back to the nest indicating the way to the food
        if self.has_food {
            match self.ant_type {
                AntType::RED => {
                    let pheromone = state
                        .to_food_grid_red
                        .grid
                        .grid
                        .get_value(&self.loc)
                        .unwrap();
                    state
                        .to_food_grid_red
                        .grid
                        .grid
                        .set_value_location(self.pheromone_dropped + pheromone, &self.loc);
                }
                AntType::BLACK => {
                    let pheromone = state
                        .to_food_grid_black
                        .grid
                        .grid
                        .get_value(&self.loc)
                        .unwrap();
                    state
                        .to_food_grid_black
                        .grid
                        .grid
                        .set_value_location(self.pheromone_dropped + pheromone, &self.loc);
                }
            }
            self.pheromone_dropped *= PHEROMONE_DROP_DECREASE;
        }
        self.act(state);
    }
}

impl Eq for Ant {}

impl PartialEq for Ant {
    fn eq(&self, other: &Ant) -> bool {
        self.id == other.id
    }
}

impl Hash for Ant {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Ant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} loc {}", self.id, self.loc)
    }
}
