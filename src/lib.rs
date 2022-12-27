pub mod base_cell;
pub mod bounding_box;
pub(crate) mod consts;
pub mod coordinate;
pub mod direction;
pub mod face;
pub mod index;
pub mod latlng;
pub mod result;

pub use consts::*;
pub use index::H3Index;

pub const MAX_CELL_BOUNDARY_VERTS: usize = 10;

pub fn is_resolution_classIII(res: usize) -> bool {
    res % 2 > 0
}
