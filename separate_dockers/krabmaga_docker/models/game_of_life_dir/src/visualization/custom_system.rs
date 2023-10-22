// use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
// use krabmaga::visualization::fields::number_grid_2d::BatchRender;
// use krabmaga::engine::location::Int2D;
// use crate::model::board::{Board, Cell, Status};
// use krabmaga::bevy::prelude::Image;
// impl BatchRender<Board> for DenseGrid2D<Cell> {
//     fn get_pixel(&self, loc: &Int2D) -> [u8; 4] {
//        // get cell
//         let cell = self.get_objects(loc).unwrap()[0];
//         match cell.status {
//             Status::Dead => [0u8, 0u8, 0u8, 255u8],
//             Status::Alive => [255u8, 255u8, 255u8, 255u8],
//         }
//     }
//
//     fn get_dimensions(&self) -> (u32, u32) {
//         (self.width as u32, self.height as u32)
//     }
//
//     fn get_layer(&self) -> f32 {
//         -1.
//     }
//
//     fn get_texture_from_state(state: &Board) -> Image {
//         state.field.texture()
//     }
// }