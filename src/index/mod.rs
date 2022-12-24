pub(crate) mod consts;

use core::ops::{BitAnd, Shl, Shr};

use derive_new::new;

use crate::{consts::H3_CELL_MODE, direction::Direction, result::H3ErrorCode, MAX_H3_RES};
pub use consts::*;

/// Identifier for an object (cell, edge, etc) in the H3System.
/// The H3Index fits within a 64-bit unsigned integer.
#[derive(Debug, new)]
pub struct H3Index(u64);

impl H3Index {
    /// Initializes an index with `resolution`, `base_cell`, and `direction` (0-7).
    pub fn init(resolution: usize, base_cell: usize, direction: Direction) -> Self {
        let mut h3 = Self(H3_INIT)
            .set_mode(H3_CELL_MODE)
            .set_resolution(resolution)
            .set_base_cell(base_cell);

        for r in 1..=resolution {
            h3 = h3.set_index_digit(r, direction.clone());
        }

        h3
    }

    pub fn high_bit(&self) -> usize {
        ((self & H3_HIGH_BIT_MASK) >> H3_MAX_OFFSET) as usize
    }

    /// Sets the highest bit of the h3 to `bit`. Consumes `self`.
    pub(crate) fn set_high_bit(self, bit: u64) -> Self {
        Self((self & H3_HIGH_BIT_MASK_NEGATIVE) | (bit << H3_MAX_OFFSET))
    }

    /// Returns the mode of the index.
    pub fn mode(&self) -> usize {
        ((self & H3_MODE_MASK) >> H3_MODE_OFFSET) as usize
    }

    /// Sets the mode of the index to `mode`. Consumes `self`.
    pub(crate) fn set_mode(self, mode: usize) -> Self {
        Self((self & H3_MODE_MASK_NEGATIVE) | ((mode as u64) << H3_MODE_OFFSET))
    }

    /// Returns the integer base cell of `self`.
    pub fn base_cell(&self) -> usize {
        ((self & H3_BC_MASK) >> H3_BC_OFFSET) as usize
    }

    /// Sets the base cell of the index to `base_cell`. Consumes `self`.
    pub(crate) fn set_base_cell(self, base_cell: usize) -> Self {
        Self((self & H3_BC_MASK_NEGATIVE) | ((base_cell as u64) << H3_BC_OFFSET))
    }

    /// Returns the resolution of the index.
    pub fn resolution(&self) -> usize {
        ((self & H3_RES_MASK) >> H3_RES_OFFSET) as usize
    }

    /// Sets the resolution of the index to `resolution`. Consumes `self`.
    pub(crate) fn set_resolution(self, resolution: usize) -> Self {
        Self((self & H3_RES_MASK_NEGATIVE) | ((resolution as u64) << H3_RES_OFFSET))
    }

    /// Gets the resolution digit (0-7)
    pub fn index_digit(&self, resolution: usize) -> Direction {
        (((self >> (MAX_H3_RES - (resolution as u64))) * H3_PER_DIGIT_OFFSET) & H3_DIGIT_MASK)
            .into()
    }

    /// Sets the resolution of the index.
    pub(crate) fn set_index_digit<D>(self, resolution: usize, direction: D) -> Self
    where
        D: Into<u64>,
    {
        let res = resolution as u64;
        Self(
            ((self) & !(H3_DIGIT_MASK << ((MAX_H3_RES - res) * H3_PER_DIGIT_OFFSET)))
                | ((direction.into()) << ((MAX_H3_RES - res) * H3_PER_DIGIT_OFFSET)),
        )
    }

    /// Returns the value in the reserved space. Should always be 0 for valid indices.
    pub fn reserved(&self) -> usize {
        ((self & H3_RESERVED_MASK) >> H3_RESERVED_OFFSET) as usize
    }

    /// Sets a value in the reserved space. Setting to non-zero may
    /// produce invalid indices.
    pub(crate) fn set_reserved(self, val: usize) -> Self {
        Self((self & H3_RESERVED_MASK_NEGATIVE) | ((val as u64) << H3_RESERVED_OFFSET))
    }

    /// Returns whether or not an index is a valid cell (hexagon or pentagon).
    pub fn valid_cell(&self) -> bool {
        if self.high_bit() != 0 {
            return false;
        }

        if self.mode() != H3_CELL_MODE {
            return false;
        }

        let base_cell = self.base_cell();
        // NEVER()?

        true
    }
}

impl core::fmt::Display for H3Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl TryFrom<String> for H3Index {
    type Error = H3ErrorCode;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for H3Index {
    type Error = H3ErrorCode;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .parse::<u64>()
            .map(H3Index)
            .map_err(|_| H3ErrorCode::Failed)
    }
}

impl From<H3Index> for String {
    fn from(h3: H3Index) -> Self {
        format!("{}", h3.0)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn debug_index(h3: H3Index) -> H3Index {
        println!("H3: {}", h3);
        h3
    }

    #[test]
    fn h3_init() {
        let h3 = debug_index(H3Index::init(3, 4, Direction::Center));
        assert_eq!(3, h3.resolution());
        assert_eq!(4, h3.base_cell());
        // TODO: figure this out
        // assert_eq!(
        //     Into::<usize>::into(Direction::Center),
        //     h3.index_digit(3).into()
        // );
    }

    #[test]
    fn h3_high_bit() {
        let h3 = debug_index(H3Index::init(15, 4, Direction::KAxes));
        assert_eq!(0, h3.high_bit());
        assert_eq!(1, debug_index(h3.set_high_bit(1)).high_bit(),);
    }

    #[test]
    fn h3_mode() {
        let h3 = debug_index(H3Index::init(12, 4, Direction::KAxes));
        assert_eq!(1, h3.mode());
        assert_eq!(0, debug_index(h3.set_mode(0)).mode(),);
    }
}
