use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::state::State;
use std::hash::{Hash, Hasher};
use crate::model::board::Board;
use crate::model::constants::DIRECTIONS;

#[derive(Clone, Copy)]
pub struct Person {
    pub id: u32,
    pub loc: Int2D,
    pub escaped: bool,
}

impl Hash for Person {
    fn hash<H>(&self, state: &mut H)
        where
            H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Eq for Person {}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.id == other.id
    }
}

impl Agent for Person {
    fn step(&mut self, state: &mut dyn State) {
        if self.escaped {
            return;
        }
        //println!("Person {} at {} {}", self.id, self.loc.x, self.loc.y);
        // agent will take a look at its neighbors and move to the first empty cell which value is lower than its own
        // if there is no empty cell with lower value, it will stay in place
        let board = state.as_any_mut().downcast_mut::<Board>().unwrap();
        let current_loc = self.loc;
        let current_value = board.field.get_value(&current_loc).unwrap();

        if current_value == 0 {
            // if the new value is 0, then the agent has reached the goal
            //println!("Person {} has reached the goal", self.id);
            board.agents.remove_object_location(*self, &current_loc);
            self.escaped = true;
            board.escaped += 1;
            return;
        }

        let mut new_loc = current_loc;
        let mut new_value = current_value;
        let mut found = false;
        //print current value
        //println!("Person {} current value {}", self.id, current_value);

        for (dx, dy) in DIRECTIONS.iter() {
            let neighbor_loc = Int2D { x: current_loc.x + dx, y: current_loc.y + dy };
            // print neighbor location
            // println!("Person {} neighbor location {} {}", self.id, neighbor_loc.x, neighbor_loc.y);
            // check if neighbor is in bounds
            if neighbor_loc.x >= 0 && neighbor_loc.x < board.rows && neighbor_loc.y >= 0 && neighbor_loc.y < board.cols {
                // check if there is an agent in the neighbor cell if None then the cell is empty
                match board.agents.get_objects_unbuffered(&neighbor_loc) {
                    None => {
                        let neighbor_value = board.field.get_value(&neighbor_loc).unwrap();
                        // check if the value of the neighbor cell is lower than the current value
                        if neighbor_value < new_value {
                            //println!("Person {} new value {}", self.id, neighbor_value);
                            // if so, update the new location and value
                            new_loc = neighbor_loc;
                            new_value = neighbor_value;
                            found = true;
                        }
                    },
                    Some(agents) => {
                        if agents.len() == 0 {
                            let neighbor_value = board.field.get_value(&neighbor_loc).unwrap();
                            // check if the value of the neighbor cell is lower than the current value
                            if neighbor_value < new_value {
                                //println!("Person {} new value {}", self.id, neighbor_value);
                                // if so, update the new location and value
                                new_loc = neighbor_loc;
                                new_value = neighbor_value;
                                found = true;
                            }
                        }
                    }
                }
            }
        }

        if found {
            self.loc = new_loc;
            board.agents.set_object_location(*self, &new_loc);
        }
        //board.agents.update();
    }
}