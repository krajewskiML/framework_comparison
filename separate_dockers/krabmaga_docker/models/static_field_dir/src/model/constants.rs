// define all 8 directions
pub(crate) const DIRECTIONS: [(i32, i32); 8] = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, -1), (-1, 1), (1, -1)];
const FREE_SPACE: i8 = 0;
pub(crate) const WALL: i8 = 1;
pub(crate) const ESCAPE_DOOR: i8 = 2;
pub(crate) const PERSON: i8 = 3;
pub(crate) const MAX_VALUE: i16 = i16::MAX;