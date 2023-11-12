use krabmaga::engine::location::Int2D;
use std::ops::Range;

pub const FOOD_RADIUS: Range<i32> = 3..5;
pub const FOOD_VALUE: i8 = 2;
pub const NEW_FOOD_FREQUENCY: u64 = 500;
pub const EVAPORATION_RATE: f32 = 0.01;
pub const DIFFUSION_RATE: f32 = 0.15;
pub const PHEROMONE_LOW_THRESHOLD: f32 = 0.4;
pub const PHEROMONE_INITIAL_VALUE: f32 = 100.;
pub const PHEROMONE_DROP_DECREASE: f32 = 0.95;
pub const WALL_PHEROMONE_VALUE: f32 = 100000.0;
