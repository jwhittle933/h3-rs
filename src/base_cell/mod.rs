pub(crate) mod consts;

use crate::face::Face;
pub use consts::*;

use derive_new::new;

#[derive(Clone, Debug, PartialEq, PartialOrd, new)]
pub struct BaseCellData {
    pub home_face: Face,
    pub is_pentagon: bool,
    pub cw_offset_pentagon: [isize; 2],
}

#[derive(Clone, Debug, PartialEq, PartialOrd, new)]
pub struct BaseCellRotation {
    /// base cell number
    pub base_cell: isize,
    /// number of counter-clockwise 60 degree rotations
    /// relative to the current face.
    pub ccw_rotation_60: isize,
}

/// Returns whether or not the indicated base cell is a pentagon.
pub fn is_base_cell_pentagon(base_cell: usize) -> bool {
    if base_cell >= NUM_BASE_CELLS {
        return false;
    }

    BASE_CELL_DATA[base_cell].is_pentagon
}

/// Returns whether the indicated base cell is a pentagon where all
/// neighbors are oriented toward it.
pub fn is_base_cell_polar_pentagon(base_cell: usize) -> bool {
    base_cell == 4 || base_cell == 117
}
