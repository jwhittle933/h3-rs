mod constants;

pub use constants::*;

use crate::{constants::NUM_BASE_CELLS, faceijk::FaceIJK};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct BaseCellData {
    pub home_fijk: FaceIJK,
    pub is_pentagon: bool,
    pub cw_offset_pent: [i32; 2],
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct BaseCellRotation {
    pub base_cell: u64,
    pub ccw_rot60: i32,
}

// BaseCell operations.
pub(crate) trait BaseCell {
    /// Return whether or not the indicated base cell is a pentagon.
    fn is_base_cell_pentagon(&self) -> bool {
        false
    }

    /// Return whether the indicated base cell is a pentagon where all
    /// neighbors are oriented towards it.
    fn is_base_cell_polar_pentagon(&self) -> bool {
        false
    }
}

impl BaseCell for u64 {
    fn is_base_cell_pentagon(&self) -> bool {
        let val = *self as usize;
        if val >= NUM_BASE_CELLS {
            return false;
        }

        BASE_CELL_DATA[val].is_pentagon
    }

    fn is_base_cell_polar_pentagon(&self) -> bool {
        *self == 4 || *self == 117
    }
}
