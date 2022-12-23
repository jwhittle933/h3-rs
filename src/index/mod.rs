use core::ops::{BitAnd, Shl, Shr};

use crate::error::H3ErrorCode;

use derive_new::new;

/// The number of bits in an H3 index.
pub const H3_NUM_BITS: u64 = 64;
/// The bit offset of the max resolution digit in an H3 index.
pub const H3_MAX_OFFSET: u64 = 63;
/// The bit offset of the mode in an H3 index.
pub const H3_MODE_OFFSET: u64 = 59;
/// The bit offset of the base cell in an H3 index.
pub const H3_BC_OFFSET: u64 = 45;
/// The bit offset of the resolution in an H3 index.
pub const H3_RES_OFFSET: u64 = 52;
/// The bit offset of the reserved bits in an H3 index.
pub const H3_RESERVED_OFFSET: u64 = 56;
/// The bit offset of the reserved bits in an H3 index.
pub const H3_PER_DIGIT_OFFSET: u64 = 3;
/// 1 in the highest bit, 0's everywhere else.
pub const H3_HIGH_BIT_MASK: u64 = 1 << H3_MAX_OFFSET;
/// 0 in the highest bit, 1's everywhere else.
pub const H3_HIGH_BIT_MASK_NEGATIVE: u64 = !H3_HIGH_BIT_MASK;
/// 1 in the 4 mode bits, 0's everywhere else.
pub const H3_MODE_MASK: u64 = 15 << H3_MODE_OFFSET;
/// 0 in the 4 mode bits, 1's everywhere else.
pub const H3_MODE_MASK_NEGATIVE: u64 = !H3_MODE_MASK;
/// 1's in the 7 base cell bits, 0's everywhere else;
pub const H3_BC_MASK: u64 = 127 << H3_BC_OFFSET;
/// 0's in the 7 base cell bits, 1's everywhere else;
pub const H3_BC_MASK_NEGATIVE: u64 = !H3_BC_MASK;
/// 1's in the 4 resolution bits, 0's everwhere else;
pub const H3_RES_MASK: u64 = 15 << H3_RES_OFFSET;
/// 0's in the 4 resolution bits, 1's everwhere else;
pub const H3_RES_MASK_NEGATIVE: u64 = !H3_RES_MASK;
/// 1's in the 3 reserved bits, 0's everwhere else;
pub const H3_RESERVED_MASK: u64 = 7 << H3_RESERVED_OFFSET;
/// 0's in the 3 reserved bits, 1's everwhere else;
pub const H3_RESERVED_MASK_NEGATIVE: u64 = !H3_RESERVED_MASK;
/// 1's in the 3 bits of res 15 digit bits, 0's everwhere else;
pub const H3_DIGIT_MASK: u64 = 7;
/// 0's in the 7 base cell bits, 1's everwhere else;
pub const H3_DIGIT_MASK_NEGATIVE: u64 = !H3_DIGIT_MASK;

/// H3 index with mode 0, res 0, base cell 0, and 7 for all index digits.
/// Typically used to initialize the creation of an H3 cell index, which
/// expects all direction digits to be 7 beyond the cell's resolution.
pub const H3_INIT: u64 = 35184372088831;

/// Identifier for an object (cell, edge, etc) in the H3System.
/// The H3Index fits within a 64-bit unsigned integer.
#[derive(Debug, new)]
pub struct H3Index(u64);

impl H3Index {
    pub fn high_bit(&self) -> u64 {
        (self & H3_HIGH_BIT_MASK) >> H3_MAX_OFFSET
    }

    /// Sets the highest bit of the h3 to `bit`. Consumes `self`.
    pub(crate) fn with_high_bit(self, bit: u64) -> Self {
        Self((self & H3_HIGH_BIT_MASK_NEGATIVE) | (bit << H3_MAX_OFFSET))
    }

    /// Returns the mode of the index.
    pub fn mode(&self) -> u64 {
        (self & H3_MODE_MASK) >> H3_MODE_OFFSET
    }

    /// Sets the mode of the index to `mode`. Consumes `self`.
    pub(crate) fn with_mode(self, mode: u64) -> Self {
        Self((self & H3_MODE_MASK_NEGATIVE) | (mode << H3_MODE_OFFSET))
    }

    /// Returns the integer base cell of `self`.
    pub fn base_cell(&self) -> u64 {
        (self & H3_BC_MASK) >> H3_BC_OFFSET
    }

    /// Sets the base cell of the index to `base_cell`. Consumes `self`.
    pub(crate) fn with_base_cell(self, base_cell: u64) -> Self {
        Self((self & H3_BC_MASK_NEGATIVE) | (base_cell << H3_BC_OFFSET))
    }

    /// Returns the resolution of the index.
    pub fn resolution(&self) -> u64 {
        (self & H3_RES_MASK) >> H3_RES_OFFSET
    }

    /// Sets the resolution of the index to `resolution`. Consumes `self`.
    pub(crate) fn with_resolution(self, resolution: u64) -> Self {
        Self((self & H3_RES_MASK_NEGATIVE) | (resolution << H3_RES_OFFSET))
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
