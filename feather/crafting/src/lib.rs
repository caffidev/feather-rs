use libcraft_items::Item;

mod model;
mod recipe;
mod solver;

pub const TABLE_WIDTH: usize = 3;
pub const TABLE_SIZE: usize = TABLE_WIDTH * TABLE_WIDTH;

pub use recipe::convert;
pub use solver::{transpose, Solver};
/// A main crafting grid. Origin is UL to DR.
/// Stored in column-major, indexed by [x][y]
/// will yield the item `x` slots from the left
/// and `y` slots from the top.
pub type Grid = [[Option<Item>; TABLE_WIDTH]; TABLE_WIDTH];
