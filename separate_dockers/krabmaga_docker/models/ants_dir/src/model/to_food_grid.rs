use crate::model::constants::{DIFFUSION_RATE, EVAPORATION_RATE, PHEROMONE_LOW_THRESHOLD};
use array2d::{Array2D, Error};
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::grid_option::GridOption;
use krabmaga::engine::fields::sparse_number_grid_2d::SparseNumberGrid2D;
use krabmaga::engine::location::Int2D;

// Represents food pheromones. Higher f32 value means more concentrated pheromone.
pub struct ToFoodGrid {
    pub grid: SparseNumberGrid2D<f32>,
}

impl ToFoodGrid {
    pub fn new(width: i32, height: i32) -> ToFoodGrid {
        let mut grid = SparseNumberGrid2D::new(width, height);
        // fill the grid with 0.0
        for x in 0..width {
            for y in 0..height {
                grid.set_value_location(0.0, &Int2D { x, y });
            }
        }
        grid.update();
        ToFoodGrid { grid }
    }

    pub fn update(&mut self) {
        let mut new_grid =
            Array2D::filled_with(0., self.grid.width as usize, self.grid.height as usize);

        //self.grid.update();
        // Diffusion
        // calculate average of the 8 neighbors of each cell
        for x in 0..self.grid.width {
            for y in 0..self.grid.height {
                let mut sum = 0.;
                let mut count = 0;
                for dx in -1..2 {
                    for dy in -1..2 {
                        let neighbor = self.grid.get_value(&Int2D {
                            x: x + dx,
                            y: y + dy,
                        });
                        if neighbor.is_some() {
                            sum += neighbor.unwrap();
                            count += 1;
                        }
                    }
                }
                let average = sum / (count + 1) as f32;
                new_grid.set(x as usize, y as usize, average).unwrap();
            }
        }
        // subtract value of each cell from the average of its neighbors
        for x in 0..self.grid.width {
            for y in 0..self.grid.height {
                let value = self.grid.get_value(&Int2D { x, y }).unwrap();
                let average = new_grid.get(x as usize, y as usize).unwrap();
                let diff = average - value;
                new_grid.set(x as usize, y as usize, diff).unwrap();
            }
        }
        // multiply each cell by the diffusion rate
        for x in 0..self.grid.width {
            for y in 0..self.grid.height {
                let value = new_grid.get(x as usize, y as usize).unwrap();
                new_grid
                    .set(x as usize, y as usize, value * DIFFUSION_RATE)
                    .unwrap();
            }
        }
        // add the result to the original grid
        for x in 0..self.grid.width {
            for y in 0..self.grid.height {
                let value = new_grid.get(x as usize, y as usize).unwrap();
                let original_value = self.grid.get_value(&Int2D { x, y }).unwrap();
                self.grid
                    .set_value_location(original_value + value, &Int2D { x, y });
            }
        }
        self.grid.update();
        // evaporate pheromones
        self.grid.apply_to_all_values(
            |val| {
                let new_val = val * (1. - EVAPORATION_RATE);
                if new_val < PHEROMONE_LOW_THRESHOLD {
                    return 0.;
                }
                new_val
            },
            GridOption::READWRITE,
        );
        self.grid.update();
        // check if grid is filled by grabbing the value of the middle cell
        let middle = self.grid.get_value(&Int2D {x: self.grid.width / 2, y: self.grid.height / 2});
        if !middle.is_some() {
            //raise error
            panic!("Grid is not filled");
        }
    }

    pub fn update2(&mut self) {
        self.grid.update();
        // difuse food pheromones
        // first we need to create a 2d array of the same size as the grid filled with zeros
        let mut new_grid =
            Array2D::filled_with(0., self.grid.width as usize, self.grid.height as usize);
        // then we need to iterate over the grid and add the value of the current cell to the 8 surrounding cells
        for x in 0..self.grid.width {
            for y in 0..self.grid.height {
                let cur_value = self.grid.get_value(&Int2D { x, y }).unwrap_or(0.);
                // change value of current cell to value * (1 - diffusion rate)
                //new_grid[(x as usize, y as usize)] =  cur_value * (1. - DIFFUSION_RATE);
                let value = cur_value / 8.;
                // if value is to low we don't need to add it to the surrounding cells
                if value < PHEROMONE_LOW_THRESHOLD {
                    continue;
                }

                let x_index = x as usize;
                let y_index = y as usize;
                // add the value to the current cell if the cell is not out of bounds
                if x_index > 0 && y_index > 0 {
                    new_grid[(x_index - 1, y_index - 1)] += value;
                }
                if y_index > 0 {
                    new_grid[(x_index, y_index - 1)] += value;
                }
                if x_index < self.grid.width as usize - 1 && y_index > 0 {
                    new_grid[(x_index + 1, y_index - 1)] += value;
                }
                if x_index > 0 {
                    new_grid[(x_index - 1, y_index)] += value;
                }
                if x_index < self.grid.width as usize - 1 {
                    new_grid[(x_index + 1, y_index)] += value;
                }
                if x_index > 0 && y_index < self.grid.height as usize - 1 {
                    new_grid[(x_index - 1, y_index + 1)] += value;
                }
                if y_index < self.grid.height as usize - 1 {
                    new_grid[(x_index, y_index + 1)] += value;
                }
                if x_index < self.grid.width as usize - 1 && y_index < self.grid.height as usize - 1
                {
                    new_grid[(x_index + 1, y_index + 1)] += value;
                }
            }
        }
        //println!("new grid: {:?}", new_grid);

        // then we need to iterate over the new grid and set the values of the original grid
        for x in 0..self.grid.width {
            for y in 0..self.grid.height {
                let x_index = x as usize;
                let y_index = y as usize;
                self.grid
                    .set_value_location(new_grid[(x_index, y_index)], &Int2D { x, y });
            }
        }
        self.grid.update();

        // evaporate pheromones
        self.grid.apply_to_all_values(
            |val| {
                let new_val = val * (1. - EVAPORATION_RATE);
                if new_val < PHEROMONE_LOW_THRESHOLD {
                    return 0.;
                }
                new_val
            },
            GridOption::READ,
        );

        self.grid.update();
    }
}

pub struct ToFoodGridRed {
    pub grid: ToFoodGrid,
}

impl ToFoodGridRed {
    pub fn new(width: i32, height: i32) -> ToFoodGridRed {
        ToFoodGridRed {
            grid: ToFoodGrid::new(width, height),
        }
    }

    pub fn update(&mut self) {
        self.grid.update();
    }
}

pub struct ToFoodGridBlack {
    pub grid: ToFoodGrid,
}

impl ToFoodGridBlack {
    pub fn new(width: i32, height: i32) -> ToFoodGridBlack {
        ToFoodGridBlack {
            grid: ToFoodGrid::new(width, height),
        }
    }

    pub fn update(&mut self) {
        self.grid.update();
    }
}
