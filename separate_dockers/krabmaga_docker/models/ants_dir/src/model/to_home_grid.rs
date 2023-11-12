use crate::model::constants::{EVAPORATION_RATE, WALL_PHEROMONE_VALUE};
use crate::model::state::ModelState;
use array2d::Array2D;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::grid_option::GridOption;
use krabmaga::engine::fields::sparse_number_grid_2d::SparseNumberGrid2D;
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::engine::location::Int2D;
use std::cmp::min;

// Represents home pheromones. Higher f32 means more concentrated pheromone.
pub struct ToHomeGrid {
    pub grid: SparseNumberGrid2D<f32>,
    pub anthill_position: Int2D,
}

impl ToHomeGrid {
    pub fn new(width: i32, height: i32, anthill_position: Int2D) -> ToHomeGrid {
        // calculate the distance from the anthill and set the pheromone concentration accordingly
        ToHomeGrid {
            grid: SparseNumberGrid2D::new(width, height),
            anthill_position,
        }
    }

    // function that will calculate static field taking walls into consideration
    pub fn calculate_static_field(&mut self, state: &ModelState) {}

    pub fn update(&mut self) {
        self.grid.update();
        self.grid
            .apply_to_all_values(|val| val * (1. - EVAPORATION_RATE), GridOption::READ)
    }
}

pub struct ToHomeGridRed {
    pub grid: ToHomeGrid,
}

impl ToHomeGridRed {
    pub fn new(width: i32, height: i32, anthill_position: Int2D) -> ToHomeGridRed {
        ToHomeGridRed {
            grid: ToHomeGrid::new(width, height, anthill_position),
        }
    }

    pub fn calculate_static_field(&mut self, state: &ModelState) {
        self.grid.calculate_static_field(state);
    }

    pub fn update(&mut self) {
        //self.grid.update();
    }
}

pub struct ToHomeGridBlack {
    pub grid: ToHomeGrid,
}

impl ToHomeGridBlack {
    pub fn new(width: i32, height: i32, anthill_position: Int2D) -> ToHomeGridBlack {
        ToHomeGridBlack {
            grid: ToHomeGrid::new(width, height, anthill_position),
        }
    }

    pub fn update(&mut self) {
        //self.grid.update();
    }

    pub fn calculate_static_field(&mut self, state: &ModelState) {
        self.grid.calculate_static_field(state);
    }
}
