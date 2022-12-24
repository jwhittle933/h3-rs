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
    pub fn init(resolution: usize, base_cell: usize, direction: Direction) -> Self {
        let mut h3 = Self(H3_INIT)
            .set_mode(H3_CELL_MODE)
            .set_resolution(resolution)
            .set_base_cell(base_cell);

        for r in 1..=resolution {
            h3 = h3.set_index_digit(r, direction);
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
    pub(crate) fn set_index_digit(self, resolution: usize, direction: Direction) -> Self {
        let digit: u64 = direction.into();

        Self(
            (self & !(H3_DIGIT_MASK << ((MAX_H3_RES - (resolution as u64)) * H3_PER_DIGIT_OFFSET)))
                | (digit << ((MAX_H3_RES - (resolution as u64)) * H3_PER_DIGIT_OFFSET)),
        )
    }

    pub fn valid_cell(&self) {
        //
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
