/// Coordinate IJK functionality including conversion from lat/lng
/// References two Vec2d cartesian coordinate systems:
///
///    1. gnomonic: face-centered polyhedral gnomonic projection space with
///             traditional scaling and x-axes aligned with the face Class II
///             i-axes.
///
///    2. hex2d: local face-centered coordinate system scaled a specific H3 grid
///             resolution unit length and with x-axes aligned with the local
///             i-axes
use derive_new::new;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref UNIT_VECS: [Coordinate; 7] = [
        Coordinate{i: 0, j: 0, k: 0},  // direction 0
        Coordinate{i: 0, j: 0, k: 1},  // direction 1
        Coordinate{i: 0, j: 1, k: 0},  // direction 2
        Coordinate{i: 0, j: 1, k: 1},  // direction 3
        Coordinate{i: 1, j: 0, k: 0},  // direction 4
        Coordinate{i: 1, j: 0, k: 1},  // direction 5
        Coordinate{i: 1, j: 1, k: 0}   // direction 6
    ];
}

/// Each axis is spaced 120-degress apart.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, new)]
pub struct Coordinate {
    pub i: isize,
    pub j: isize,
    pub k: isize,
}
