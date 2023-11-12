extern crate core;

use clap::{Arg, Command};
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
// Global imports, required in all cases
use crate::model::state::{Item, ModelState};

pub mod model;

pub const EVAPORATION: f32 = 0.99;
pub const STEP: u64 = 1000;

// Visualization specific imports
// #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
// use {
//     crate::model::to_food_grid::ToFoodGrid, crate::model::to_home_grid::ToHomeGrid,
//     crate::visualization::vis_state::VisState, krabmaga::bevy::prelude::Color,
//     krabmaga::bevy::prelude::IntoSystem,
//     krabmaga::visualization::fields::number_grid_2d::BatchRender,
//     krabmaga::visualization::visualization::Visualization,
// };

use crate::model::ant::Ant;
use crate::model::to_food_grid::{ToFoodGridBlack, ToFoodGridRed};
use crate::model::to_home_grid::{ToHomeGridBlack, ToHomeGridRed};
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use {
    krabmaga::engine::schedule::Schedule, krabmaga::engine::state::State, krabmaga::simulate,
    krabmaga::Info, krabmaga::ProgressBar, krabmaga::*, std::time::Duration,
};

// #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
// pub mod visualization;
//
// // Main used when a visualization feature is applied
// #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
// fn main() {
//     let state = ModelState::new(100);
//     let mut app = Visualization::default()
//         .with_background_color(Color::rgb(255., 255., 255.))
//         .with_simulation_dimensions(100 as f32, 100 as f32)
//         .with_window_dimensions(1280., 720.)
//         .with_name("Ants foraging")
//         .setup::<VisState, ModelState>(VisState, state);
//     app //.add_system(ToHomeGridRed::batch_render.system())
//         //.add_system(ToHomeGridBlack::batch_render.system())
//         .add_system(ToFoodGridBlack::batch_render.system())
//         .add_system(ToFoodGridRed::batch_render.system())
//         .add_system(SparseGrid2D::<Item>::batch_render.system())
//         .add_system(SparseGrid2D::<Ant>::batch_render.system())
//         .add_system(ToHomeGridRed::batch_render.system())
//         .add_system(ToHomeGridBlack::batch_render.system());
//     app.run()
// }

// #[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
// use {krabmaga::rand, krabmaga::rand::Rng};

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let matches = Command::new("Game of Life")
        .version("0.1.0")
        .author("Maciej Krajewski")
        .about("Approach to Ants")
        .arg(Arg::new("size")
            .short('s')
            .long("size")
            .help("Size describing initial state"))
        .arg(Arg::new("steps")
            .short('t')
            .long("steps")
            .help("Steps to run simulation for"))
        .get_matches();

    let default_size = "100".to_string();
    let size: &String = matches.get_one::<String>("size").unwrap_or(&default_size);

    let default_steps = "1000".to_string();
    let steps: &String = matches.get_one::<String>("steps").unwrap_or(&default_steps);

    // parse size and steps
    let size = size.parse::<i32>().unwrap();
    let steps = steps.parse::<i32>().unwrap();

    // print size and steps
    println!("Size: {}", size);
    println!("Steps: {}", steps);


    let state = ModelState::new(size);

    let _ = simulate!(state, steps as u64, 1, false);
}
