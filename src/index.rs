use core::ops::{BitAnd, Shl, Shr};

use crate::{
    base_cell::BaseCell,
    constants::{MAX_H3_RES, NUM_BASE_CELLS, NUM_PENT_VERTS},
    coordijk::Direction,
    error::H3ErrorCode,
    faceijk::FaceIJK,
    CellBoundary, LatLng,
};

pub const H3_BC_OFFSET: u64 = 45;
pub const H3_BC_MASK: u64 = 127 << H3_BC_OFFSET;
pub const H3_RES_OFFSET: u64 = 52;
pub const H3_RES_MASK: u64 = 15 << H3_RES_OFFSET;
pub const H3_PER_DIGIT_OFFSET: u64 = 3;
pub const H3_DIGIT_MASK: u64 = 7;

/// Identifier for an object (cell, edge, etc) in the H3System.
/// The H3Index fits within a 64-bit unsigned integer.
pub struct H3Index(u64);

impl H3Index {
    pub fn new(i: u64) -> Self {
        Self(i)
    }

    /// Returns the integer base cell of `self`.
    pub fn base_cell(&self) -> u64 {
        self & H3_BC_MASK >> H3_BC_OFFSET
    }

    pub fn resolution(&self) -> usize {
        (self & H3_RES_MASK >> H3_RES_OFFSET) as usize
    }

    /// Determines if index is a pentagon.
    pub fn is_pentagon(&self) -> bool {
        if let Direction::CenterDigit = self.into() {
            false
        } else {
            self.base_cell().is_base_cell_pentagon()
        }
    }

    /// Returns the resolution res integer digit (0-7).
    pub fn index_digit(&self, resolution: usize) -> Direction {
        ((self >> (MAX_H3_RES - resolution as u64) * H3_PER_DIGIT_OFFSET) & H3_DIGIT_MASK).into()
    }

    /// Rotate an index 60 degrees clockwise.
    pub fn rotate_60cw(&mut self) {
        let res = self.resolution() as u64;
        for r in 1..=res {
            self.set_index_digit(r, self.index_digit(r as usize).rotate_60cw().into())
        }
    }

    /// Rotate an index 60 degrees counter-clockwise.
    pub fn rotate_60ccw(&mut self) {
        let res = self.resolution() as u64;
        for r in 1..=res {
            self.set_index_digit(r, self.index_digit(r as usize).rotate_60ccw().into())
        }
    }

    fn set_index_digit(&mut self, res: u64, digit: u64) {
        self.0 = ((self) & !(H3_DIGIT_MASK << ((MAX_H3_RES - (res)) * H3_PER_DIGIT_OFFSET)))
            | ((digit) << ((MAX_H3_RES - (res)) * H3_PER_DIGIT_OFFSET));
    }
}

/// Determines the spherical coordinates of the center point of an H3 index.
impl TryInto<LatLng> for H3Index {
    type Error = H3ErrorCode;

    fn try_into(self) -> Result<LatLng, Self::Error> {
        (&self).try_into()
    }
}

/// Determines the spherical coordinates of the center point of an H3 index.
impl TryInto<LatLng> for &H3Index {
    type Error = H3ErrorCode;

    fn try_into(self) -> Result<LatLng, Self::Error> {
        TryInto::<FaceIJK>::try_into(self)?.try_into()
    }
}

impl TryInto<CellBoundary> for H3Index {
    type Error = H3ErrorCode;

    fn try_into(self) -> Result<CellBoundary, Self::Error> {
        (&self).try_into()
    }
}

impl TryInto<CellBoundary> for &H3Index {
    type Error = H3ErrorCode;

    fn try_into(self) -> Result<CellBoundary, Self::Error> {
        let face: FaceIJK = self.try_into()?;
        if self.is_pentagon() {
            Ok(face.pentagon_to_cell_boundary(self.resolution(), 0, NUM_PENT_VERTS))
        } else {
            Ok(face)
        }
    }
}

impl TryInto<FaceIJK> for H3Index {
    type Error = H3ErrorCode;

    fn try_into(self) -> Result<FaceIJK, Self::Error> {
        (&self).try_into()
    }
}

impl TryInto<FaceIJK> for &H3Index {
    type Error = H3ErrorCode;

    /// Determines the cell boundary in spherical coordinates for an index.
    fn try_into(self) -> Result<FaceIJK, Self::Error> {
        let out: FaceIJK = Default::default();

        let base_cell = self.base_cell();
        if base_cell as usize >= NUM_BASE_CELLS {
            return Err(H3ErrorCode::CellInvalid);
        }

        let dir: Direction = self.into();
        if base_cell.is_base_cell_pentagon() && dir == Direction::IKAxesDigit {
            self.rotate_60cw();
        }

        Ok(out)
    }
}

impl Into<Direction> for H3Index {
    fn into(self) -> Direction {
        (&self).into()
    }
}

impl Into<Direction> for &H3Index {
    fn into(self) -> Direction {
        let resolution = self.resolution();
        for i in 1..=resolution {
            let digit = self.index_digit(resolution);
            if digit != Direction::CenterDigit {
                return digit;
            }
        }

        Direction::CenterDigit
    }
}

impl BaseCell for H3Index {
    fn is_base_cell_pentagon(&self) -> bool {
        (&self).is_base_cell_pentagon()
    }

    fn is_base_cell_polar_pentagon(&self) -> bool {
        (&self).is_base_cell_polar_pentagon()
    }
}

impl BaseCell for &H3Index {
    fn is_base_cell_pentagon(&self) -> bool {
        self.0.is_base_cell_pentagon()
    }

    fn is_base_cell_polar_pentagon(&self) -> bool {
        self.0.is_base_cell_polar_pentagon()
    }
}

impl BitAnd<u64> for H3Index {
    type Output = u64;

    fn bitand(self, rhs: u64) -> Self::Output {
        (&self).bitand(rhs)
    }
}

impl BitAnd<u64> for &H3Index {
    type Output = u64;

    fn bitand(self, rhs: u64) -> Self::Output {
        self.0 & rhs
    }
}

impl BitAnd<u64> for &mut H3Index {
    type Output = u64;

    fn bitand(self, rhs: u64) -> Self::Output {
        self.0 & rhs
    }
}

impl Shl<u64> for H3Index {
    type Output = u64;

    fn shl(self, rhs: u64) -> Self::Output {
        (&self).0 << rhs
    }
}

impl Shl<u64> for &H3Index {
    type Output = u64;

    fn shl(self, rhs: u64) -> Self::Output {
        self.0 << rhs
    }
}

impl Shr<u64> for H3Index {
    type Output = u64;

    fn shr(self, rhs: u64) -> Self::Output {
        (&self).0 >> rhs
    }
}

impl Shr<u64> for &H3Index {
    type Output = u64;

    fn shr(self, rhs: u64) -> Self::Output {
        self.0 >> rhs
    }
}
