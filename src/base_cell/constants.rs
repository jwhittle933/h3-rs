use crate::{
    constants::{NUM_BASE_CELLS, NUM_ICOSA_FACES},
    faceijk::FaceIJK,
    CoordIJK,
};

use super::{BaseCellData, BaseCellRotation};
pub const INVALID_BASE_CELL: i32 = 127;

/// Resolution 0 base cell data table.
///
/// For each base cell, gives the "home" face and ijk+ coordinates on that face,
/// whether or not the base cell is a pentagon. Additionally, if the base cell
/// is a pentagon, the two cw offset rotation adjacent faces are given (-1
/// indicates that no cw offset rotation faces exist for this base cell).
pub(super) const BASE_CELL_DATA: [BaseCellData; NUM_BASE_CELLS] = [
    BaseCellData {
        home_fijk: FaceIJK {
            face: 1,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [0, 0],
    }, // base cell 0
    BaseCellData {
        home_fijk: FaceIJK {
            face: 2,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [0, 0],
    }, // base cell 1
    BaseCellData {
        home_fijk: FaceIJK {
            face: 1,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [0, 0],
    }, // base cell 2
    BaseCellData {
        home_fijk: FaceIJK {
            face: 2,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [0, 0],
    }, // base cell 3
    BaseCellData {
        home_fijk: FaceIJK {
            face: 0,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [-1, -1],
    }, // base cell 4
    BaseCellData {
        home_fijk: FaceIJK {
            face: 1,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 5
    BaseCellData {
        home_fijk: FaceIJK {
            face: 1,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 6
    BaseCellData {
        home_fijk: FaceIJK {
            face: 2,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 7
    BaseCellData {
        home_fijk: FaceIJK {
            face: 0,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 8
    BaseCellData {
        home_fijk: FaceIJK {
            face: 2,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 9
    BaseCellData {
        home_fijk: FaceIJK {
            face: 1,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 10
    BaseCellData {
        home_fijk: FaceIJK {
            face: 1,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 11
    BaseCellData {
        home_fijk: FaceIJK {
            face: 3,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 12
    BaseCellData {
        home_fijk: FaceIJK {
            face: 3,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 13
    BaseCellData {
        home_fijk: FaceIJK {
            face: 11,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [2, 6],
    }, // base cell 14
    BaseCellData {
        home_fijk: FaceIJK {
            face: 4,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 15
    BaseCellData {
        home_fijk: FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 16
    BaseCellData {
        home_fijk: FaceIJK {
            face: 6,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 17
    BaseCellData {
        home_fijk: FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 18
    BaseCellData {
        home_fijk: FaceIJK {
            face: 2,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 19
    BaseCellData {
        home_fijk: FaceIJK {
            face: 7,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 20
    BaseCellData {
        home_fijk: FaceIJK {
            face: 2,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 21
    BaseCellData {
        home_fijk: FaceIJK {
            face: 0,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 22
    BaseCellData {
        home_fijk: FaceIJK {
            face: 6,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 23
    BaseCellData {
        home_fijk: FaceIJK {
            face: 10,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [1, 5],
    }, // base cell 24
    BaseCellData {
        home_fijk: FaceIJK {
            face: 6,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 25
    BaseCellData {
        home_fijk: FaceIJK {
            face: 3,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 26
    BaseCellData {
        home_fijk: FaceIJK {
            face: 11,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 27
    BaseCellData {
        home_fijk: FaceIJK {
            face: 4,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 28
    BaseCellData {
        home_fijk: FaceIJK {
            face: 3,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 29
    BaseCellData {
        home_fijk: FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 30
    BaseCellData {
        home_fijk: FaceIJK {
            face: 4,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 31
    BaseCellData {
        home_fijk: FaceIJK {
            face: 5,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 32
    BaseCellData {
        home_fijk: FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 33
    BaseCellData {
        home_fijk: FaceIJK {
            face: 7,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 34
    BaseCellData {
        home_fijk: FaceIJK {
            face: 11,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 35
    BaseCellData {
        home_fijk: FaceIJK {
            face: 7,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 36
    BaseCellData {
        home_fijk: FaceIJK {
            face: 10,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 37
    BaseCellData {
        home_fijk: FaceIJK {
            face: 12,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [3, 7],
    }, // base cell 38
    BaseCellData {
        home_fijk: FaceIJK {
            face: 6,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 39
    BaseCellData {
        home_fijk: FaceIJK {
            face: 7,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 40
    BaseCellData {
        home_fijk: FaceIJK {
            face: 4,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 41
    BaseCellData {
        home_fijk: FaceIJK {
            face: 3,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 42
    BaseCellData {
        home_fijk: FaceIJK {
            face: 3,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 43
    BaseCellData {
        home_fijk: FaceIJK {
            face: 4,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 44
    BaseCellData {
        home_fijk: FaceIJK {
            face: 6,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 45
    BaseCellData {
        home_fijk: FaceIJK {
            face: 11,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 46
    BaseCellData {
        home_fijk: FaceIJK {
            face: 8,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 47
    BaseCellData {
        home_fijk: FaceIJK {
            face: 5,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 48
    BaseCellData {
        home_fijk: FaceIJK {
            face: 14,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [0, 9],
    }, // base cell 49
    BaseCellData {
        home_fijk: FaceIJK {
            face: 5,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 50
    BaseCellData {
        home_fijk: FaceIJK {
            face: 12,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 51
    BaseCellData {
        home_fijk: FaceIJK {
            face: 10,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 52
    BaseCellData {
        home_fijk: FaceIJK {
            face: 4,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 53
    BaseCellData {
        home_fijk: FaceIJK {
            face: 12,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 54
    BaseCellData {
        home_fijk: FaceIJK {
            face: 7,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 55
    BaseCellData {
        home_fijk: FaceIJK {
            face: 11,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 56
    BaseCellData {
        home_fijk: FaceIJK {
            face: 10,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 57
    BaseCellData {
        home_fijk: FaceIJK {
            face: 13,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [4, 8],
    }, // base cell 58
    BaseCellData {
        home_fijk: FaceIJK {
            face: 10,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 59
    BaseCellData {
        home_fijk: FaceIJK {
            face: 11,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 60
    BaseCellData {
        home_fijk: FaceIJK {
            face: 9,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 61
    BaseCellData {
        home_fijk: FaceIJK {
            face: 8,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 62
    BaseCellData {
        home_fijk: FaceIJK {
            face: 6,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [11, 15],
    }, // base cell 63
    BaseCellData {
        home_fijk: FaceIJK {
            face: 8,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 64
    BaseCellData {
        home_fijk: FaceIJK {
            face: 9,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 65
    BaseCellData {
        home_fijk: FaceIJK {
            face: 14,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 66
    BaseCellData {
        home_fijk: FaceIJK {
            face: 5,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 67
    BaseCellData {
        home_fijk: FaceIJK {
            face: 16,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 68
    BaseCellData {
        home_fijk: FaceIJK {
            face: 8,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 69
    BaseCellData {
        home_fijk: FaceIJK {
            face: 5,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 70
    BaseCellData {
        home_fijk: FaceIJK {
            face: 12,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 71
    BaseCellData {
        home_fijk: FaceIJK {
            face: 7,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [12, 16],
    }, // base cell 72
    BaseCellData {
        home_fijk: FaceIJK {
            face: 12,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 73
    BaseCellData {
        home_fijk: FaceIJK {
            face: 10,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 74
    BaseCellData {
        home_fijk: FaceIJK {
            face: 9,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 75
    BaseCellData {
        home_fijk: FaceIJK {
            face: 13,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 76
    BaseCellData {
        home_fijk: FaceIJK {
            face: 16,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 77
    BaseCellData {
        home_fijk: FaceIJK {
            face: 15,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 78
    BaseCellData {
        home_fijk: FaceIJK {
            face: 15,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 79
    BaseCellData {
        home_fijk: FaceIJK {
            face: 16,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 80
    BaseCellData {
        home_fijk: FaceIJK {
            face: 14,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 81
    BaseCellData {
        home_fijk: FaceIJK {
            face: 13,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 82
    BaseCellData {
        home_fijk: FaceIJK {
            face: 5,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [10, 19],
    }, // base cell 83
    BaseCellData {
        home_fijk: FaceIJK {
            face: 8,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 84
    BaseCellData {
        home_fijk: FaceIJK {
            face: 14,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 85
    BaseCellData {
        home_fijk: FaceIJK {
            face: 9,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 86
    BaseCellData {
        home_fijk: FaceIJK {
            face: 14,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 87
    BaseCellData {
        home_fijk: FaceIJK {
            face: 17,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 88
    BaseCellData {
        home_fijk: FaceIJK {
            face: 12,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 89
    BaseCellData {
        home_fijk: FaceIJK {
            face: 16,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 90
    BaseCellData {
        home_fijk: FaceIJK {
            face: 17,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 91
    BaseCellData {
        home_fijk: FaceIJK {
            face: 15,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 92
    BaseCellData {
        home_fijk: FaceIJK {
            face: 16,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 93
    BaseCellData {
        home_fijk: FaceIJK {
            face: 9,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 94
    BaseCellData {
        home_fijk: FaceIJK {
            face: 15,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 95
    BaseCellData {
        home_fijk: FaceIJK {
            face: 13,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 96
    BaseCellData {
        home_fijk: FaceIJK {
            face: 8,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [13, 17],
    }, // base cell 97
    BaseCellData {
        home_fijk: FaceIJK {
            face: 13,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 98
    BaseCellData {
        home_fijk: FaceIJK {
            face: 17,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 99
    BaseCellData {
        home_fijk: FaceIJK {
            face: 19,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 100
    BaseCellData {
        home_fijk: FaceIJK {
            face: 14,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 101
    BaseCellData {
        home_fijk: FaceIJK {
            face: 19,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 102
    BaseCellData {
        home_fijk: FaceIJK {
            face: 17,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 103
    BaseCellData {
        home_fijk: FaceIJK {
            face: 13,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 104
    BaseCellData {
        home_fijk: FaceIJK {
            face: 17,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 105
    BaseCellData {
        home_fijk: FaceIJK {
            face: 16,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 106
    BaseCellData {
        home_fijk: FaceIJK {
            face: 9,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [14, 18],
    }, // base cell 107
    BaseCellData {
        home_fijk: FaceIJK {
            face: 15,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 108
    BaseCellData {
        home_fijk: FaceIJK {
            face: 15,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 109
    BaseCellData {
        home_fijk: FaceIJK {
            face: 18,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 110
    BaseCellData {
        home_fijk: FaceIJK {
            face: 18,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 111
    BaseCellData {
        home_fijk: FaceIJK {
            face: 19,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 112
    BaseCellData {
        home_fijk: FaceIJK {
            face: 17,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 113
    BaseCellData {
        home_fijk: FaceIJK {
            face: 19,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 114
    BaseCellData {
        home_fijk: FaceIJK {
            face: 18,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 115
    BaseCellData {
        home_fijk: FaceIJK {
            face: 18,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 116
    BaseCellData {
        home_fijk: FaceIJK {
            face: 19,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        is_pentagon: false,
        cw_offset_pent: [-1, -1],
    }, // base cell 117
    BaseCellData {
        home_fijk: FaceIJK {
            face: 19,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 118
    BaseCellData {
        home_fijk: FaceIJK {
            face: 18,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 119
    BaseCellData {
        home_fijk: FaceIJK {
            face: 19,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 120
    BaseCellData {
        home_fijk: FaceIJK {
            face: 18,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        is_pentagon: true,
        cw_offset_pent: [0, 0],
    }, // base cell 121
];

/// @brief Neighboring base cell rotations in each IJK direction.
///
/// For each base cell, for each direction, the number of 60 degree
/// CCW rotations to the coordinate system of the neighbor is given.
/// -1 indicates there is no neighbor in that direction.
pub(super) const BASE_CELL_NEIGHBOR_60CCW_ROTS: [[i32; 7]; NUM_BASE_CELLS] = [
    [0, 5, 0, 0, 1, 5, 1],  // base cell 0
    [0, 0, 1, 0, 1, 0, 1],  // base cell 1
    [0, 0, 0, 0, 0, 5, 0],  // base cell 2
    [0, 5, 0, 0, 2, 5, 1],  // base cell 3
    [0, -1, 1, 0, 3, 4, 2], // base cell 4 (pentagon)
    [0, 0, 1, 0, 1, 0, 1],  // base cell 5
    [0, 0, 0, 3, 5, 5, 0],  // base cell 6
    [0, 0, 0, 0, 0, 5, 0],  // base cell 7
    [0, 5, 0, 0, 0, 5, 1],  // base cell 8
    [0, 0, 1, 3, 0, 0, 1],  // base cell 9
    [0, 0, 1, 3, 0, 0, 1],  // base cell 10
    [0, 3, 3, 3, 0, 0, 0],  // base cell 11
    [0, 5, 0, 0, 3, 5, 1],  // base cell 12
    [0, 0, 1, 0, 1, 0, 1],  // base cell 13
    [0, -1, 3, 0, 5, 2, 0], // base cell 14 (pentagon)
    [0, 5, 0, 0, 4, 5, 1],  // base cell 15
    [0, 0, 0, 0, 0, 5, 0],  // base cell 16
    [0, 3, 3, 3, 3, 0, 3],  // base cell 17
    [0, 0, 0, 3, 5, 5, 0],  // base cell 18
    [0, 3, 3, 3, 0, 0, 0],  // base cell 19
    [0, 3, 3, 3, 0, 3, 0],  // base cell 20
    [0, 0, 0, 3, 5, 5, 0],  // base cell 21
    [0, 0, 1, 0, 1, 0, 1],  // base cell 22
    [0, 3, 3, 3, 0, 3, 0],  // base cell 23
    [0, -1, 3, 0, 5, 2, 0], // base cell 24 (pentagon)
    [0, 0, 0, 3, 0, 0, 3],  // base cell 25
    [0, 0, 0, 0, 0, 5, 0],  // base cell 26
    [0, 3, 0, 0, 0, 3, 3],  // base cell 27
    [0, 0, 1, 0, 1, 0, 1],  // base cell 28
    [0, 0, 1, 3, 0, 0, 1],  // base cell 29
    [0, 3, 3, 3, 0, 0, 0],  // base cell 30
    [0, 0, 0, 0, 0, 5, 0],  // base cell 31
    [0, 3, 3, 3, 3, 0, 3],  // base cell 32
    [0, 0, 1, 3, 0, 0, 1],  // base cell 33
    [0, 3, 3, 3, 3, 0, 3],  // base cell 34
    [0, 0, 3, 0, 3, 0, 3],  // base cell 35
    [0, 0, 0, 3, 0, 0, 3],  // base cell 36
    [0, 3, 0, 0, 0, 3, 3],  // base cell 37
    [0, -1, 3, 0, 5, 2, 0], // base cell 38 (pentagon)
    [0, 3, 0, 0, 3, 3, 0],  // base cell 39
    [0, 3, 0, 0, 3, 3, 0],  // base cell 40
    [0, 0, 0, 3, 5, 5, 0],  // base cell 41
    [0, 0, 0, 3, 5, 5, 0],  // base cell 42
    [0, 3, 3, 3, 0, 0, 0],  // base cell 43
    [0, 0, 1, 3, 0, 0, 1],  // base cell 44
    [0, 0, 3, 0, 0, 3, 3],  // base cell 45
    [0, 0, 0, 3, 0, 3, 0],  // base cell 46
    [0, 3, 3, 3, 0, 3, 0],  // base cell 47
    [0, 3, 3, 3, 0, 3, 0],  // base cell 48
    [0, -1, 3, 0, 5, 2, 0], // base cell 49 (pentagon)
    [0, 0, 0, 3, 0, 0, 3],  // base cell 50
    [0, 3, 0, 0, 0, 3, 3],  // base cell 51
    [0, 0, 3, 0, 3, 0, 3],  // base cell 52
    [0, 3, 3, 3, 0, 0, 0],  // base cell 53
    [0, 0, 3, 0, 3, 0, 3],  // base cell 54
    [0, 0, 3, 0, 0, 3, 3],  // base cell 55
    [0, 3, 3, 3, 0, 0, 3],  // base cell 56
    [0, 0, 0, 3, 0, 3, 0],  // base cell 57
    [0, -1, 3, 0, 5, 2, 0], // base cell 58 (pentagon)
    [0, 3, 3, 3, 3, 3, 0],  // base cell 59
    [0, 3, 3, 3, 3, 3, 0],  // base cell 60
    [0, 3, 3, 3, 3, 0, 3],  // base cell 61
    [0, 3, 3, 3, 3, 0, 3],  // base cell 62
    [0, -1, 3, 0, 5, 2, 0], // base cell 63 (pentagon)
    [0, 0, 0, 3, 0, 0, 3],  // base cell 64
    [0, 3, 3, 3, 0, 3, 0],  // base cell 65
    [0, 3, 0, 0, 0, 3, 3],  // base cell 66
    [0, 3, 0, 0, 3, 3, 0],  // base cell 67
    [0, 3, 3, 3, 0, 0, 0],  // base cell 68
    [0, 3, 0, 0, 3, 3, 0],  // base cell 69
    [0, 0, 3, 0, 0, 3, 3],  // base cell 70
    [0, 0, 0, 3, 0, 3, 0],  // base cell 71
    [0, -1, 3, 0, 5, 2, 0], // base cell 72 (pentagon)
    [0, 3, 3, 3, 0, 0, 3],  // base cell 73
    [0, 3, 3, 3, 0, 0, 3],  // base cell 74
    [0, 0, 0, 3, 0, 0, 3],  // base cell 75
    [0, 3, 0, 0, 0, 3, 3],  // base cell 76
    [0, 0, 0, 3, 0, 5, 0],  // base cell 77
    [0, 3, 3, 3, 0, 0, 0],  // base cell 78
    [0, 0, 1, 3, 1, 0, 1],  // base cell 79
    [0, 0, 1, 3, 1, 0, 1],  // base cell 80
    [0, 0, 3, 0, 3, 0, 3],  // base cell 81
    [0, 0, 3, 0, 3, 0, 3],  // base cell 82
    [0, -1, 3, 0, 5, 2, 0], // base cell 83 (pentagon)
    [0, 0, 3, 0, 0, 3, 3],  // base cell 84
    [0, 0, 0, 3, 0, 3, 0],  // base cell 85
    [0, 3, 0, 0, 3, 3, 0],  // base cell 86
    [0, 3, 3, 3, 3, 3, 0],  // base cell 87
    [0, 0, 0, 3, 0, 5, 0],  // base cell 88
    [0, 3, 3, 3, 3, 3, 0],  // base cell 89
    [0, 0, 0, 0, 0, 0, 1],  // base cell 90
    [0, 3, 3, 3, 0, 0, 0],  // base cell 91
    [0, 0, 0, 3, 0, 5, 0],  // base cell 92
    [0, 5, 0, 0, 5, 5, 0],  // base cell 93
    [0, 0, 3, 0, 0, 3, 3],  // base cell 94
    [0, 0, 0, 0, 0, 0, 1],  // base cell 95
    [0, 0, 0, 3, 0, 3, 0],  // base cell 96
    [0, -1, 3, 0, 5, 2, 0], // base cell 97 (pentagon)
    [0, 3, 3, 3, 0, 0, 3],  // base cell 98
    [0, 5, 0, 0, 5, 5, 0],  // base cell 99
    [0, 0, 1, 3, 1, 0, 1],  // base cell 100
    [0, 3, 3, 3, 0, 0, 3],  // base cell 101
    [0, 3, 3, 3, 0, 0, 0],  // base cell 102
    [0, 0, 1, 3, 1, 0, 1],  // base cell 103
    [0, 3, 3, 3, 3, 3, 0],  // base cell 104
    [0, 0, 0, 0, 0, 0, 1],  // base cell 105
    [0, 0, 1, 0, 3, 5, 1],  // base cell 106
    [0, -1, 3, 0, 5, 2, 0], // base cell 107 (pentagon)
    [0, 5, 0, 0, 5, 5, 0],  // base cell 108
    [0, 0, 1, 0, 4, 5, 1],  // base cell 109
    [0, 3, 3, 3, 0, 0, 0],  // base cell 110
    [0, 0, 0, 3, 0, 5, 0],  // base cell 111
    [0, 0, 0, 3, 0, 5, 0],  // base cell 112
    [0, 0, 1, 0, 2, 5, 1],  // base cell 113
    [0, 0, 0, 0, 0, 0, 1],  // base cell 114
    [0, 0, 1, 3, 1, 0, 1],  // base cell 115
    [0, 5, 0, 0, 5, 5, 0],  // base cell 116
    [0, -1, 1, 0, 3, 4, 2], // base cell 117 (pentagon)
    [0, 0, 1, 0, 0, 5, 1],  // base cell 118
    [0, 0, 0, 0, 0, 0, 1],  // base cell 119
    [0, 5, 0, 0, 5, 5, 0],  // base cell 120
    [0, 0, 1, 0, 1, 5, 1],  // base cell 121
];

/// Neighboring base cell ID in each IJK direction.
///
/// For each base cell, for each direction, the neighboring base
/// cell ID is given. 127 indicates there is no neighbor in that direction.
pub(super) const BASE_CELL_NEIGHBORS: [[i32; 7]; NUM_BASE_CELLS] = [
    [0, 1, 5, 2, 4, 3, 8],                             // base cell 0
    [1, 7, 6, 9, 0, 3, 2],                             // base cell 1
    [2, 6, 10, 11, 0, 1, 5],                           // base cell 2
    [3, 13, 1, 7, 4, 12, 0],                           // base cell 3
    [4, INVALID_BASE_CELL, 15, 8, 3, 0, 12],           // base cell 4 (pentagon)
    [5, 2, 18, 10, 8, 0, 16],                          // base cell 5
    [6, 14, 11, 17, 1, 9, 2],                          // base cell 6
    [7, 21, 9, 19, 3, 13, 1],                          // base cell 7
    [8, 5, 22, 16, 4, 0, 15],                          // base cell 8
    [9, 19, 14, 20, 1, 7, 6],                          // base cell 9
    [10, 11, 24, 23, 5, 2, 18],                        // base cell 10
    [11, 17, 23, 25, 2, 6, 10],                        // base cell 11
    [12, 28, 13, 26, 4, 15, 3],                        // base cell 12
    [13, 26, 21, 29, 3, 12, 7],                        // base cell 13
    [14, INVALID_BASE_CELL, 17, 27, 9, 20, 6],         // base cell 14 (pentagon)
    [15, 22, 28, 31, 4, 8, 12],                        // base cell 15
    [16, 18, 33, 30, 8, 5, 22],                        // base cell 16
    [17, 11, 14, 6, 35, 25, 27],                       // base cell 17
    [18, 24, 30, 32, 5, 10, 16],                       // base cell 18
    [19, 34, 20, 36, 7, 21, 9],                        // base cell 19
    [20, 14, 19, 9, 40, 27, 36],                       // base cell 20
    [21, 38, 19, 34, 13, 29, 7],                       // base cell 21
    [22, 16, 41, 33, 15, 8, 31],                       // base cell 22
    [23, 24, 11, 10, 39, 37, 25],                      // base cell 23
    [24, INVALID_BASE_CELL, 32, 37, 10, 23, 18],       // base cell 24 (pentagon)
    [25, 23, 17, 11, 45, 39, 35],                      // base cell 25
    [26, 42, 29, 43, 12, 28, 13],                      // base cell 26
    [27, 40, 35, 46, 14, 20, 17],                      // base cell 27
    [28, 31, 42, 44, 12, 15, 26],                      // base cell 28
    [29, 43, 38, 47, 13, 26, 21],                      // base cell 29
    [30, 32, 48, 50, 16, 18, 33],                      // base cell 30
    [31, 41, 44, 53, 15, 22, 28],                      // base cell 31
    [32, 30, 24, 18, 52, 50, 37],                      // base cell 32
    [33, 30, 49, 48, 22, 16, 41],                      // base cell 33
    [34, 19, 38, 21, 54, 36, 51],                      // base cell 34
    [35, 46, 45, 56, 17, 27, 25],                      // base cell 35
    [36, 20, 34, 19, 55, 40, 54],                      // base cell 36
    [37, 39, 52, 57, 24, 23, 32],                      // base cell 37
    [38, INVALID_BASE_CELL, 34, 51, 29, 47, 21],       // base cell 38 (pentagon)
    [39, 37, 25, 23, 59, 57, 45],                      // base cell 39
    [40, 27, 36, 20, 60, 46, 55],                      // base cell 40
    [41, 49, 53, 61, 22, 33, 31],                      // base cell 41
    [42, 58, 43, 62, 28, 44, 26],                      // base cell 42
    [43, 62, 47, 64, 26, 42, 29],                      // base cell 43
    [44, 53, 58, 65, 28, 31, 42],                      // base cell 44
    [45, 39, 35, 25, 63, 59, 56],                      // base cell 45
    [46, 60, 56, 68, 27, 40, 35],                      // base cell 46
    [47, 38, 43, 29, 69, 51, 64],                      // base cell 47
    [48, 49, 30, 33, 67, 66, 50],                      // base cell 48
    [49, INVALID_BASE_CELL, 61, 66, 33, 48, 41],       // base cell 49 (pentagon)
    [50, 48, 32, 30, 70, 67, 52],                      // base cell 50
    [51, 69, 54, 71, 38, 47, 34],                      // base cell 51
    [52, 57, 70, 74, 32, 37, 50],                      // base cell 52
    [53, 61, 65, 75, 31, 41, 44],                      // base cell 53
    [54, 71, 55, 73, 34, 51, 36],                      // base cell 54
    [55, 40, 54, 36, 72, 60, 73],                      // base cell 55
    [56, 68, 63, 77, 35, 46, 45],                      // base cell 56
    [57, 59, 74, 78, 37, 39, 52],                      // base cell 57
    [58, INVALID_BASE_CELL, 62, 76, 44, 65, 42],       // base cell 58 (pentagon)
    [59, 63, 78, 79, 39, 45, 57],                      // base cell 59
    [60, 72, 68, 80, 40, 55, 46],                      // base cell 60
    [61, 53, 49, 41, 81, 75, 66],                      // base cell 61
    [62, 43, 58, 42, 82, 64, 76],                      // base cell 62
    [63, INVALID_BASE_CELL, 56, 45, 79, 59, 77],       // base cell 63 (pentagon)
    [64, 47, 62, 43, 84, 69, 82],                      // base cell 64
    [65, 58, 53, 44, 86, 76, 75],                      // base cell 65
    [66, 67, 81, 85, 49, 48, 61],                      // base cell 66
    [67, 66, 50, 48, 87, 85, 70],                      // base cell 67
    [68, 56, 60, 46, 90, 77, 80],                      // base cell 68
    [69, 51, 64, 47, 89, 71, 84],                      // base cell 69
    [70, 67, 52, 50, 83, 87, 74],                      // base cell 70
    [71, 89, 73, 91, 51, 69, 54],                      // base cell 71
    [72, INVALID_BASE_CELL, 73, 55, 80, 60, 88],       // base cell 72 (pentagon)
    [73, 91, 72, 88, 54, 71, 55],                      // base cell 73
    [74, 78, 83, 92, 52, 57, 70],                      // base cell 74
    [75, 65, 61, 53, 94, 86, 81],                      // base cell 75
    [76, 86, 82, 96, 58, 65, 62],                      // base cell 76
    [77, 63, 68, 56, 93, 79, 90],                      // base cell 77
    [78, 74, 59, 57, 95, 92, 79],                      // base cell 78
    [79, 78, 63, 59, 93, 95, 77],                      // base cell 79
    [80, 68, 72, 60, 99, 90, 88],                      // base cell 80
    [81, 85, 94, 101, 61, 66, 75],                     // base cell 81
    [82, 96, 84, 98, 62, 76, 64],                      // base cell 82
    [83, INVALID_BASE_CELL, 74, 70, 100, 87, 92],      // base cell 83 (pentagon)
    [84, 69, 82, 64, 97, 89, 98],                      // base cell 84
    [85, 87, 101, 102, 66, 67, 81],                    // base cell 85
    [86, 76, 75, 65, 104, 96, 94],                     // base cell 86
    [87, 83, 102, 100, 67, 70, 85],                    // base cell 87
    [88, 72, 91, 73, 99, 80, 105],                     // base cell 88
    [89, 97, 91, 103, 69, 84, 71],                     // base cell 89
    [90, 77, 80, 68, 106, 93, 99],                     // base cell 90
    [91, 73, 89, 71, 105, 88, 103],                    // base cell 91
    [92, 83, 78, 74, 108, 100, 95],                    // base cell 92
    [93, 79, 90, 77, 109, 95, 106],                    // base cell 93
    [94, 86, 81, 75, 107, 104, 101],                   // base cell 94
    [95, 92, 79, 78, 109, 108, 93],                    // base cell 95
    [96, 104, 98, 110, 76, 86, 82],                    // base cell 96
    [97, INVALID_BASE_CELL, 98, 84, 103, 89, 111],     // base cell 97 (pentagon)
    [98, 110, 97, 111, 82, 96, 84],                    // base cell 98
    [99, 80, 105, 88, 106, 90, 113],                   // base cell 99
    [100, 102, 83, 87, 108, 114, 92],                  // base cell 100
    [101, 102, 107, 112, 81, 85, 94],                  // base cell 101
    [102, 101, 87, 85, 114, 112, 100],                 // base cell 102
    [103, 91, 97, 89, 116, 105, 111],                  // base cell 103
    [104, 107, 110, 115, 86, 94, 96],                  // base cell 104
    [105, 88, 103, 91, 113, 99, 116],                  // base cell 105
    [106, 93, 99, 90, 117, 109, 113],                  // base cell 106
    [107, INVALID_BASE_CELL, 101, 94, 115, 104, 112],  // base cell 107 (pentagon)
    [108, 100, 95, 92, 118, 114, 109],                 // base cell 108
    [109, 108, 93, 95, 117, 118, 106],                 // base cell 109
    [110, 98, 104, 96, 119, 111, 115],                 // base cell 110
    [111, 97, 110, 98, 116, 103, 119],                 // base cell 111
    [112, 107, 102, 101, 120, 115, 114],               // base cell 112
    [113, 99, 116, 105, 117, 106, 121],                // base cell 113
    [114, 112, 100, 102, 118, 120, 108],               // base cell 114
    [115, 110, 107, 104, 120, 119, 112],               // base cell 115
    [116, 103, 119, 111, 113, 105, 121],               // base cell 116
    [117, INVALID_BASE_CELL, 109, 118, 113, 121, 106], // base cell 117 (pentagon)
    [118, 120, 108, 114, 117, 121, 109],               // base cell 118
    [119, 111, 115, 110, 121, 116, 120],               // base cell 119
    [120, 115, 114, 112, 121, 119, 118],               // base cell 120
    [121, 116, 120, 119, 117, 113, 118],               // base cell 121
];
/// Resolution 0 base cell lookup table for each face.
///
/// Given the face number and a resolution 0 ijk+ coordinate in that face's
/// face-centered ijk coordinate system, gives the base cell located at that
/// coordinate and the number of 60 ccw rotations to rotate into that bas
/// cell's orientation.
///
/// Valid lookup coordinates are from (0, 0, 0) to (2, 2, 2).
///
/// This table can be accessed using the functions `_faceIjkToBaseCell` and
/// `_faceIjkToBaseCellCCWrot60`
pub(super) const FACE_IJK_BASE_CELLS: [[[[BaseCellRotation; 3]; 3]; 3]; NUM_ICOSA_FACES] = [
    [
        [
            [
                BaseCellRotation {
                    base_cell: 16,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 18,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 24,
                    ccw_rot60: 0,
                },
            ],
            [
                BaseCellRotation {
                    base_cell: 33,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 30,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 32,
                    ccw_rot60: 3,
                },
            ],
            [
                BaseCellRotation {
                    base_cell: 49,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 48,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 50,
                    ccw_rot60: 3,
                },
            ],
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 8,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 5,
                    ccw_rot60: 5,
                },
                BaseCellRotation {
                    base_cell: 10,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 22,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 16,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 18,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 41,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 33,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 30,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 4,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 0,
                    ccw_rot60: 5,
                },
                BaseCellRotation {
                    base_cell: 2,
                    ccw_rot60: 5,
                },
            ],
            [
                BaseCellRotation {
                    base_cell: 15,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 8,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 5,
                    ccw_rot60: 5,
                },
            ],
            [
                BaseCellRotation {
                    base_cell: 31,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 22,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 16,
                    ccw_rot60: 0,
                },
            ],
        ],
    ],
    [
        [
            [
                BaseCellRotation {
                    base_cell: 2,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 6,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 14,
                    ccw_rot60: 0,
                },
            ],
            [
                BaseCellRotation {
                    base_cell: 10,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 11,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 17,
                    ccw_rot60: 3,
                },
            ],
            [
                BaseCellRotation {
                    base_cell: 24,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 23,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 25,
                    ccw_rot60: 3,
                },
            ],
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 0,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 1,
                    ccw_rot60: 5,
                },
                BaseCellRotation {
                    base_cell: 9,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 5,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 2,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 6,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 18,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 10,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 11,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation {
                    base_cell: 4,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 3,
                    ccw_rot60: 5,
                },
                BaseCellRotation {
                    base_cell: 7,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 8,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 0,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 1,
                    ccw_rot60: 5,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 16,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 5,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 2,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 2
        [
            [
                BaseCellRotation {
                    base_cell: 7,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 21,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 38,
                    ccw_rot60: 0,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 9,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 19,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 34,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 14,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 20,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 36,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 3,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 13,
                    ccw_rot60: 5,
                },
                BaseCellRotation {
                    base_cell: 29,
                    ccw_rot60: 5,
                },
            ],
            [
                BaseCellRotation {
                    base_cell: 1,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 7,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 21,
                    ccw_rot60: 0,
                },
            ],
            [
                BaseCellRotation {
                    base_cell: 6,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 9,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 19,
                    ccw_rot60: 0,
                },
            ],
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 4,
                    ccw_rot60: 2,
                },
                BaseCellRotation {
                    base_cell: 12,
                    ccw_rot60: 5,
                },
                BaseCellRotation {
                    base_cell: 26,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 0,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 3,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 13,
                    ccw_rot60: 5,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 2,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 1,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 7,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        [
            [
                BaseCellRotation {
                    base_cell: 26,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 42,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 58,
                    ccw_rot60: 0,
                },
            ],
            [
                BaseCellRotation {
                    base_cell: 29,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 43,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 62,
                    ccw_rot60: 3,
                },
            ],
            [
                BaseCellRotation {
                    base_cell: 38,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 47,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 64,
                    ccw_rot60: 3,
                },
            ],
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 12,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 28,
                    ccw_rot60: 5,
                },
                BaseCellRotation {
                    base_cell: 44,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 13,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 26,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 42,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 21,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 29,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 43,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 4,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 15,
                    ccw_rot60: 5,
                },
                BaseCellRotation {
                    base_cell: 31,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 3,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 12,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 28,
                    ccw_rot60: 5,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 7,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 13,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 26,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 4
        [
            [
                BaseCellRotation {
                    base_cell: 31,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 41,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 49,
                    ccw_rot60: 0,
                },
            ],
            [
                BaseCellRotation {
                    base_cell: 44,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 53,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 61,
                    ccw_rot60: 3,
                },
            ],
            [
                BaseCellRotation {
                    base_cell: 58,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 65,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 75,
                    ccw_rot60: 3,
                },
            ],
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 15,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 22,
                    ccw_rot60: 5,
                },
                BaseCellRotation {
                    base_cell: 33,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 28,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 31,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 41,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 42,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 44,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 53,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 4,
                    ccw_rot60: 4,
                },
                BaseCellRotation {
                    base_cell: 8,
                    ccw_rot60: 5,
                },
                BaseCellRotation {
                    base_cell: 16,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 12,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 15,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 22,
                    ccw_rot60: 5,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 26,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 28,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 31,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 5
        [
            [
                BaseCellRotation {
                    base_cell: 50,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 48,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 49,
                    ccw_rot60: 3,
                },
            ], // j ]
            [
                BaseCellRotation {
                    base_cell: 32,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 30,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 33,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 24,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 18,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 16,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 70,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 67,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 66,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 52,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 50,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 48,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 37,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 32,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 30,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 83,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 87,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 85,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 74,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 70,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 67,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 57,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 52,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 50,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 6
        [
            [
                BaseCellRotation {
                    base_cell: 25,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 23,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 24,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 17,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 11,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 10,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 14,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 6,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 2,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 45,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 39,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 37,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 35,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 25,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 23,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 27,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 17,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 11,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 63,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 59,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 57,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 56,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 45,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 39,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 46,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 35,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 25,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 7
        [
            [
                BaseCellRotation {
                    base_cell: 36,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 20,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 14,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 34,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 19,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 9,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 38,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 21,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 7,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 55,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 40,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 27,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 54,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 36,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 20,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 51,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 34,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 19,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 72,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 60,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 46,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 73,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 55,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 40,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 71,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 54,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 36,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        //face 8
        [
            // i 0
            [
                BaseCellRotation {
                    base_cell: 64,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 47,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 38,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 62,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 43,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 29,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 58,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 42,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 26,
                    ccw_rot60: 3,
                },
            ], // j ]
        ],
        [
            // i 1
            [
                BaseCellRotation {
                    base_cell: 84,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 69,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 51,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 82,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 64,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 47,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 76,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 62,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 43,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation {
                    base_cell: 97,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 89,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 71,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 98,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 84,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 69,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 96,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 82,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 64,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 9
        [
            // i 0
            [
                BaseCellRotation {
                    base_cell: 75,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 65,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 58,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 61,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 53,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 44,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 49,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 41,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 31,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation {
                    base_cell: 94,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 86,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 76,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 81,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 75,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 65,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 66,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 61,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 53,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            [
                BaseCellRotation {
                    base_cell: 107,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 104,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 96,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 101,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 94,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 86,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 85,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 81,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 75,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // base_cell:face 10
        [
            // i 0
            [
                BaseCellRotation {
                    base_cell: 57,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 59,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 63,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 74,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 78,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 79,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 83,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 92,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 95,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation {
                    base_cell: 37,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 39,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 45,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 52,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 57,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 59,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 70,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 74,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 78,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation {
                    base_cell: 24,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 23,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 25,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 32,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 37,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 39,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 50,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 52,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 57,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        [
            // i 0
            [
                BaseCellRotation {
                    base_cell: 46,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 60,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 72,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 56,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 68,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 80,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 63,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 77,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 90,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation {
                    base_cell: 27,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 40,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 55,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 35,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 46,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 60,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 45,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 56,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 68,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation {
                    base_cell: 14,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 20,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 36,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 17,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 27,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 40,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 25,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 35,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 46,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        [
            // i 0
            [
                BaseCellRotation {
                    base_cell: 71,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 89,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 97,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 73,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 91,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 103,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 72,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 88,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 105,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation {
                    base_cell: 51,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 69,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 84,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 54,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 71,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 89,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 55,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 73,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 91,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation {
                    base_cell: 38,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 47,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 64,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 34,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 51,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 69,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 36,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 54,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 71,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        [
            // i 0
            [
                BaseCellRotation {
                    base_cell: 96,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 104,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 107,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 98,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 110,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 115,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 97,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 111,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 119,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation {
                    base_cell: 76,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 86,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 94,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 82,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 96,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 104,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 84,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 98,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 110,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation {
                    base_cell: 58,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 65,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 75,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 62,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 76,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 86,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 64,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 82,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 96,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        [
            // i 0
            [
                BaseCellRotation {
                    base_cell: 85,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 87,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 83,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 101,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 102,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 100,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 107,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 112,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 114,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation {
                    base_cell: 66,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 67,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 70,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 81,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 85,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 87,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 94,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 101,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 102,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation {
                    base_cell: 49,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 48,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 50,
                    ccw_rot60: 3,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 61,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 66,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 67,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 75,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 81,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 85,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        [
            // i 0
            [
                BaseCellRotation {
                    base_cell: 95,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 92,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 83,
                    ccw_rot60: 0,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 79,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 78,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 74,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 63,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 59,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 57,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation {
                    base_cell: 109,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 108,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 100,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 93,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 95,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 92,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 77,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 79,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 78,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation {
                    base_cell: 117,
                    ccw_rot60: 4,
                },
                BaseCellRotation {
                    base_cell: 118,
                    ccw_rot60: 5,
                },
                BaseCellRotation {
                    base_cell: 114,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 106,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 109,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 108,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 90,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 93,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 95,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        [
            // i 0
            [
                BaseCellRotation {
                    base_cell: 90,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 77,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 63,
                    ccw_rot60: 0,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 80,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 68,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 56,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 72,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 60,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 46,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation {
                    base_cell: 106,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 93,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 79,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 99,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 90,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 77,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 88,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 80,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 68,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation {
                    base_cell: 117,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 109,
                    ccw_rot60: 5,
                },
                BaseCellRotation {
                    base_cell: 95,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 113,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 106,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 93,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 105,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 99,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 90,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        [
            // i 0
            [
                BaseCellRotation {
                    base_cell: 105,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 88,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 72,
                    ccw_rot60: 0,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 103,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 91,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 73,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 97,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 89,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 71,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation {
                    base_cell: 113,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 99,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 80,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 116,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 105,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 88,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 111,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 103,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 91,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation {
                    base_cell: 117,
                    ccw_rot60: 2,
                },
                BaseCellRotation {
                    base_cell: 106,
                    ccw_rot60: 5,
                },
                BaseCellRotation {
                    base_cell: 90,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 121,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 113,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 99,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 119,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 116,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 105,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        [
            // i 0
            [
                BaseCellRotation {
                    base_cell: 119,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 111,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 97,
                    ccw_rot60: 0,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 115,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 110,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 98,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 107,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 104,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 96,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation {
                    base_cell: 121,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 116,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 103,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 120,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 119,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 111,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 112,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 115,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 110,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation {
                    base_cell: 117,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 113,
                    ccw_rot60: 5,
                },
                BaseCellRotation {
                    base_cell: 105,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 118,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 121,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 116,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 114,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 120,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 119,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        [
            // i 0
            [
                BaseCellRotation {
                    base_cell: 114,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 112,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 107,
                    ccw_rot60: 0,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 100,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 102,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 101,
                    ccw_rot60: 3,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 83,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 87,
                    ccw_rot60: 3,
                },
                BaseCellRotation {
                    base_cell: 85,
                    ccw_rot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation {
                    base_cell: 118,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 120,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 115,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 108,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 114,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 112,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 92,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 100,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 102,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation {
                    base_cell: 117,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 121,
                    ccw_rot60: 5,
                },
                BaseCellRotation {
                    base_cell: 119,
                    ccw_rot60: 5,
                },
            ], // j 0
            [
                BaseCellRotation {
                    base_cell: 109,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 118,
                    ccw_rot60: 0,
                },
                BaseCellRotation {
                    base_cell: 120,
                    ccw_rot60: 0,
                },
            ], // j 1
            [
                BaseCellRotation {
                    base_cell: 95,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 108,
                    ccw_rot60: 1,
                },
                BaseCellRotation {
                    base_cell: 114,
                    ccw_rot60: 0,
                },
            ], // j 2
        ],
    ],
];
