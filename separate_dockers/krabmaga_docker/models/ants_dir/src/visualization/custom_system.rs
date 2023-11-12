use crate::model::state::ModelState;
use crate::model::state::*;
use crate::model::to_food_grid::{ToFoodGrid, ToFoodGridBlack, ToFoodGridRed};
use crate::model::to_home_grid::{ToHomeGrid, ToHomeGridBlack, ToHomeGridRed};
use krabmaga::bevy::prelude::Image;
use krabmaga::engine::location::Int2D;
use krabmaga::visualization::fields::number_grid_2d::BatchRender;

use crate::model::ant::{Ant, AntType};
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::visualization::fields::object_grid_2d::RenderObjectGrid2D;

impl BatchRender<ModelState> for ToHomeGridBlack {
    fn get_pixel(&self, loc: &Int2D) -> [u8; 4] {
        match self.grid.grid.get_value(loc) {
            Some(val) => {
                let scaled_val = val / 210.0;
                let scaled_blue = (255.0 * scaled_val) as u8;
                [10u8, 10u8, scaled_blue, 255u8]
            }
            None => [0u8, 255u8, 0u8, 0u8],
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.grid.grid.width as u32, self.grid.grid.height as u32)
    }

    fn get_layer(&self) -> f32 {
        -1.
    }

    fn get_texture_from_state(state: &ModelState) -> Image {
        state.to_home_grid_black.texture()
    }
}

impl BatchRender<ModelState> for ToHomeGridRed {
    fn get_pixel(&self, loc: &Int2D) -> [u8; 4] {
        match self.grid.grid.get_value(loc) {
            Some(val) => {
                let scaled_val = val / 210.0;
                let scaled_red = (255.0 * scaled_val) as u8;
                [scaled_red, 10u8, 10u8, 255u8]
            }
            None => [0u8, 255u8, 0u8, 0u8],
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.grid.grid.width as u32, self.grid.grid.height as u32)
    }

    fn get_layer(&self) -> f32 {
        0.
    }

    fn get_texture_from_state(state: &ModelState) -> Image {
        state.to_home_grid_red.texture()
    }
}

impl BatchRender<ModelState> for ToFoodGridBlack {
    fn get_pixel(&self, loc: &Int2D) -> [u8; 4] {
        match self.grid.grid.get_value(loc) {
            Some(val) => {
                let cell = val;

                let alpha = (cell * 10.) as u8;
                [0u8, 0u8, 255u8, alpha]
            }
            None => [0u8, 0u8, 255u8, 0u8],
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.grid.grid.width as u32, self.grid.grid.height as u32)
    }

    fn get_layer(&self) -> f32 {
        1.
    }

    fn get_texture_from_state(state: &ModelState) -> Image {
        state.to_food_grid_black.texture()
    }
}

impl BatchRender<ModelState> for ToFoodGridRed {
    fn get_pixel(&self, loc: &Int2D) -> [u8; 4] {
        match self.grid.grid.get_value(loc) {
            Some(val) => {
                let cell = val;
                let alpha = (cell * 10.) as u8;
                [0u8, 255u8, 0u8, alpha]
            }
            None => [0u8, 0u8, 255u8, 0u8],
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.grid.grid.width as u32, self.grid.grid.height as u32)
    }

    fn get_layer(&self) -> f32 {
        2.
    }

    fn get_texture_from_state(state: &ModelState) -> Image {
        state.to_food_grid_red.texture()
    }
}

impl BatchRender<ModelState> for SparseGrid2D<Item> {
    fn get_pixel(&self, loc: &Int2D) -> [u8; 4] {
        match self.get_objects(loc) {
            Some(obj) => {
                let inside = obj.first().unwrap().value;
                match inside {
                    ItemType::Food(_) => [100u8, 100u8, 0u8, 255u8],
                    ItemType::Home => [100u8, 0u8, 100u8, 255u8],
                    ItemType::Obstacle => [0u8, 100u8, 100u8, 255u8],
                }
            }
            None => [0u8, 0u8, 0u8, 0u8],
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    fn get_layer(&self) -> f32 {
        9.
    }

    fn get_texture_from_state(state: &ModelState) -> Image {
        state.obstacles_grid.texture()
    }
}

impl BatchRender<ModelState> for SparseGrid2D<Ant> {
    fn get_pixel(&self, loc: &Int2D) -> [u8; 4] {
        match self.get_objects(loc) {
            Some(ants) => {
                // get first ant
                let firs_ant = ants.first().unwrap();
                // match color to displayed pixel
                match firs_ant.ant_type {
                    AntType::RED => [255u8, 0u8, 0u8, 255u8],
                    AntType::BLACK => [0u8, 0u8, 0u8, 255u8],
                }
            }
            None => [255u8, 255u8, 255u8, 0u8],
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    fn get_layer(&self) -> f32 {
        10.
    }

    fn get_texture_from_state(state: &ModelState) -> Image {
        state.ants_grid.texture()
    }
}

// impl RenderObjectGrid2D<ModelState, Item> for SparseGrid2D<Item> {
//     fn fetch_sparse_grid(state: &ModelState) -> Option<&SparseGrid2D<Item>> {
//         Some(&state.obstacles_grid)
//     }
//
//     fn fetch_dense_grid(_state: &ModelState) -> Option<&DenseGrid2D<Item>> {
//         None
//     }
//
//     fn fetch_emoji(_state: &ModelState, obj: &Item) -> String {
//         match obj.value {
//             ItemType::Home => "house".to_string(),
//             ItemType::Food(_) => {println!("candy"); "candy".to_string()},
//             ItemType::Obstacle => "no_entry_sign".to_string(),
//         }
//     }
//
//     fn fetch_loc(state: &ModelState, obj: &Item) -> Option<Int2D> {
//         state.obstacles_grid.get_location(*obj)
//     }
//
//     fn fetch_rotation(_state: &ModelState, _obj: &Item) -> f32 {
//         0.0
//     }
//
//     fn scale(obj: &Item) -> (f32, f32) {
//         match obj.value {
//             ItemType::Home => (0.1, 0.1),
//             ItemType::Food(_) => (0.1, 0.1),
//             ItemType::Obstacle => (0.05, 0.05),
//         }
//     }
// }
