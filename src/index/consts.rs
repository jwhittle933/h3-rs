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
