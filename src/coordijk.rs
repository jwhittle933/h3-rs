//! References two [`Vec2d`] cartesian coordinate systems:
//!   1. gnomonic: face-centered polyhedral gnomonic projection space with
//!      traditional scaling and x-axes aligned with the face Class II
//!      i-axes.
//!   2. hex2d: local face-centered coordinate system scaled a specific H3 grid
//!      resolution unit length and with x-axes aligned with the local i-axes.

use core::{
    cmp,
    ops::{Add, Sub},
};

use crate::{constants::M_SIN60, vec2d::Vec2d};

pub const UNIT_VECS: [CoordIJK; 7] = [
    CoordIJK { i: 0, j: 0, k: 0 }, // direction 0
    CoordIJK { i: 0, j: 0, k: 1 }, // direction 1
    CoordIJK { i: 0, j: 1, k: 0 }, // direction 2
    CoordIJK { i: 0, j: 1, k: 1 }, // direction 3
    CoordIJK { i: 1, j: 0, k: 0 }, // direction 4
    CoordIJK { i: 1, j: 0, k: 1 }, // direction 5
    CoordIJK { i: 1, j: 1, k: 0 }, // direction 6
];

/// Used when CoordIJK is realized as IJK coordinates.
struct IJK;
/// Used when CoordIJK is realized as IJ coordinates.
struct IJ;
/// Used when CoordIJK is realized as cube coordinates.
struct Cube;

/// When a unit vector in space is expressed in Cartesian notation as a
/// linear combination of i, j, k, its three scalar components can be
/// referred to as direction cosines. The value of each component is equal
/// to the cosine of the angle formed by the unit vector with the respective
/// basis vector.
///
/// Each axis is spaced 120 degrees apart.
#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd)]
pub struct CoordIJK {
    pub i: i32,
    pub j: i32,
    pub k: i32,
}

impl CoordIJK {
    pub fn new(i: i32, j: i32, k: i32) -> Self {
        Self { i, j, k }
    }

    /// Determine the containing hex in ijk+ coordinates for a 2D cartesian
    /// coordinate vector (from DGGRID).
    ///
    /// From coordijk.c
    pub fn from_hex2d(vector: &Vec2d) -> Self {
        let mut out = Self::new(0, 0, 0);

        let a1 = ::libm::fabs(vector.x);
        let a2 = ::libm::fabs(vector.y);

        // reverse inversion
        let x2 = a2 / M_SIN60;
        let x1 = a1 + x2 / 2.0_f64;

        // check if we have the center of a hex
        let m1 = x1;
        let m2 = x2;

        // otherwise round correctly
        let r1 = x1 - m1;
        let r2 = x2 - m2;

        if r1 < 0.5 {
            if r1 < 1.0 / 3.0 {
                out.i = m1 as i32;
                out.j = if r2 < (1.0 + r1) / 2.0 {
                    m2 as i32
                } else {
                    m2 as i32 + 1
                };
            } else {
                out.j = if r2 < (1.0 - r1) {
                    m2 as i32
                } else {
                    m2 as i32 + 1
                };

                out.i = if (1.0 - r1) <= r2 && r2 < (2.0 * r1) {
                    m1 as i32 + 1
                } else {
                    m1 as i32
                };
            }
        } else {
            if r1 < 2.0 / 3.0 {
                out.j = if r2 < 1.0 - r1 {
                    m2 as i32
                } else {
                    m2 as i32 + 1
                };

                out.i = if (2.0 * r1 - 1.0) < r2 && r2 < (1.0 - r1) {
                    m1 as i32
                } else {
                    m1 as i32 + 1
                };
            } else {
                out.i = m1 as i32 + 1;
                out.j = if r2 < (r1 / 2.0) {
                    m2 as i32
                } else {
                    m2 as i32 + 1
                };
            }
        }

        // fold across the axes if necessary
        if vector.x < 0.0 {
            out.i = if out.j % 2 == 0 {
                let axisi = out.j / 2;
                let diff = out.i - axisi;
                out.i - 2 * diff
            } else {
                let axisi = (out.j + 1) / 2;
                let diff = out.i - axisi;
                out.i - (2 * diff + 1)
            };
        }

        if vector.y < 0.0 {
            out.i = out.i - (2 * out.j + 1) / 2;
            out.j = -1 * out.j;
        }

        out.normalize();
        out
    }

    /// Normalizes ijk coordinates by setting the components to the smallest possible
    /// values. Takes exlclusive access to `self`.
    pub fn normalize(&mut self) {
        if self.i < 0 {
            self.j -= self.i;
            self.k -= self.i;
            self.i = 0;
        }

        if self.j < 0 {
            self.i -= self.j;
            self.k -= self.j;
            self.j = 0;
        }

        if self.k < 0 {
            self.i -= self.k;
            self.j -= self.k;
            self.k = 0;
        }

        let mut min = self.i;
        if self.j < min {
            min = self.j;
        }
        if self.k < min {
            min = self.k;
        }

        if min > 0 {
            self.i -= min;
            self.j -= min;
            self.k -= min;
        }
    }

    /// Returns whether or not two ijk coordinates contain exactly the same
    /// component values.
    pub fn matches(&self, other: &Self) -> bool {
        // Eq handles this comparison.
        return self == other;
    }

    /// Uniformly scale the ijk coordinates by a scalar.
    pub(crate) fn scale(&mut self, factor: i32) {
        self.i *= factor;
        self.j *= factor;
        self.k *= factor;
    }

    /// Find the normalized ijk coordinates of the indexing parent of a cell in a
    /// counter-clockwise aperture 7 grid.
    pub(crate) fn up_aperture_7(&mut self) {
        let i = (self.i - self.k) as f64;
        let j = (self.j - self.k) as f64;
        self.i = ::libm::round((3.0 * i - j) / 7.0) as i32;
        self.j = ::libm::round((i + 2.0 * j) / 7.0) as i32;
        self.k = 0;
        self.normalize()
    }

    /// Find the normalized ijk coordinates of the indexing parent of a cell in a
    /// clockwise aperture 7 grid.
    pub(crate) fn up_aperture_7r(&mut self) {
        let i = (self.i - self.k) as f64;
        let j = (self.j - self.k) as f64;
        self.i = ::libm::round((2.0 * i + j) / 7.0) as i32;
        self.j = ::libm::round((3.0 * j - i) / 7.0) as i32;
        self.k = 0;
        self.normalize()
    }

    /// Find the normalized ijk coordinates of the hex centered on the indicated
    /// hex at the next finer aperture 7 counter-clockwise resolution.
    // TODO: fix repitition
    pub(crate) fn down_aperture_7(&mut self) {
        let (mut i_vec, mut j_vec, mut k_vec) =
            (Self::new(3, 0, 1), Self::new(1, 3, 0), Self::new(0, 1, 3));

        i_vec.scale(self.i);
        j_vec.scale(self.j);
        k_vec.scale(self.k);

        let CoordIJK { i, j, k } = i_vec + j_vec;
        self.i = i;
        self.j = j;
        self.k = k;

        let CoordIJK { i, j, k } = self.add(&k_vec);
        self.i = i;
        self.j = j;
        self.k = k;
        self.normalize()
    }

    /// Find the normalized ijk coordinates of the hex centered on the indicated
    /// hex at the next finer aperture 7 clockwise resolution.
    // TODO: fix repitition
    pub(crate) fn down_aperture_7r(&mut self) {
        let (mut i_vec, mut j_vec, mut k_vec) =
            (Self::new(3, 0, 1), Self::new(0, 3, 1), Self::new(1, 0, 3));

        i_vec.scale(self.i);
        j_vec.scale(self.j);
        k_vec.scale(self.k);

        let CoordIJK { i, j, k } = i_vec + j_vec;
        self.i = i;
        self.j = j;
        self.k = k;

        let CoordIJK { i, j, k } = self.add(&k_vec);
        self.i = i;
        self.j = j;
        self.k = k;
        self.normalize()
    }

    /// Find the normalized ijk coordinates of the hex centered on the indicated
    /// hex at the next finer aperture 3 counter-clockwise resolution.
    // TODO: fix repitition
    pub(crate) fn down_aperture_3(&mut self) {
        let (mut i_vec, mut j_vec, mut k_vec) =
            (Self::new(2, 0, 1), Self::new(1, 2, 0), Self::new(0, 1, 2));

        i_vec.scale(self.i);
        j_vec.scale(self.j);
        k_vec.scale(self.k);

        let CoordIJK { i, j, k } = i_vec + j_vec;
        self.i = i;
        self.j = j;
        self.k = k;

        let CoordIJK { i, j, k } = self.add(&k_vec);
        self.i = i;
        self.j = j;
        self.k = k;
        self.normalize()
    }

    /// Find the normalized ijk coordinates of the hex centered on the indicated
    /// hex at the next finer aperture 3 clockwise resolution.
    // TODO: fix repitition
    pub(crate) fn down_aperture_3r(&mut self) {
        let (mut i_vec, mut j_vec, mut k_vec) =
            (Self::new(2, 1, 0), Self::new(0, 2, 1), Self::new(1, 2, 0));

        i_vec.scale(self.i);
        j_vec.scale(self.j);
        k_vec.scale(self.k);

        let CoordIJK { i, j, k } = i_vec + j_vec;
        self.i = i;
        self.j = j;
        self.k = k;

        let CoordIJK { i, j, k } = self.add(&k_vec);
        self.i = i;
        self.j = j;
        self.k = k;
        self.normalize()
    }

    /// Find the normalized ijk coordinates of the hex in the specified digit
    /// direction from the specified ijk coordinates.
    pub(crate) fn neighbor(&mut self, digit: Direction) {
        if digit > Direction::CenterDigit && digit < Direction::NumDigits {
            let CoordIJK { i, j, k } = self.as_ref() + &UNIT_VECS[digit as usize];
            self.i = i;
            self.j = j;
            self.k = k;
            self.normalize();
        }
    }

    /// Rotates ijk coordinates 60 degrees counter-clockwise.
    pub(crate) fn rotate_60ccw(&mut self) {
        let (mut i_vec, mut j_vec, mut k_vec) =
            (Self::new(1, 1, 0), Self::new(0, 1, 1), Self::new(1, 0, 1));

        i_vec.scale(self.i);
        j_vec.scale(self.j);
        k_vec.scale(self.k);

        let CoordIJK { i, j, k } = i_vec + j_vec;
        self.i = i;
        self.j = j;
        self.k = k;

        let CoordIJK { i, j, k } = self.add(&k_vec);
        self.i = i;
        self.j = j;
        self.k = k;

        self.normalize();
    }

    /// Rotates ijk coordinates 60 degrees clockwise.
    pub(crate) fn rotate_60cw(&mut self) {
        let (mut i_vec, mut j_vec, mut k_vec) =
            (Self::new(1, 0, 1), Self::new(1, 1, 0), Self::new(0, 1, 1));

        i_vec.scale(self.i);
        j_vec.scale(self.j);
        k_vec.scale(self.k);

        let CoordIJK { i, j, k } = i_vec + j_vec;
        self.i = i;
        self.j = j;
        self.k = k;

        let CoordIJK { i, j, k } = self.add(&k_vec);
        self.i = i;
        self.j = j;
        self.k = k;

        self.normalize();
    }

    /// Finds the distance between the two coordinates.
    pub fn distance(&self, rhs: &Self) -> i32 {
        let mut diff = self - rhs;
        diff.normalize();
        diff.i = diff.i.abs();
        diff.j = diff.j.abs();
        diff.k = diff.k.abs();

        cmp::max(diff.i, cmp::max(diff.j, diff.k))
    }

    /// Transforms coordinates from IJK+ coordinate system to the IJ coordinate
    /// system.
    // TODO: add generic constraint for IJK Coords
    pub fn ijk_to_ij(&mut self) -> Self {
        Self::new(self.i - self.k, self.j - self.k, 0)
    }

    /// Transforms coordinates from the IJ coordinate system to the IJK+ coordinate
    /// system.
    // TODO: add generic constraint for IJ Coords
    pub fn ij_to_ijk(&mut self) -> Self {
        let mut out = Self::new(self.i, self.j, 0);
        out.normalize();
        out
    }

    /// Convert IJK coordinates to cube coordinates.
    pub fn to_cube(&mut self) {
        self.i = self.i + self.k;
        self.j = self.j - self.k;
        self.k = -self.i - self.j;
    }

    /// Convert cube coordinates to IJK coordinates.
    pub fn from_cube(&mut self) {
        self.i = self.i + self.k;
        self.j = self.j - self.k;
        self.k = -self.i - self.j;
    }
}

impl Into<Vec2d> for CoordIJK {
    fn into(self) -> Vec2d {
        let i = self.i - self.k;
        let j = self.j - self.k;

        Vec2d {
            x: i as f64 - 5.0 * j as f64,
            y: j as f64 * M_SIN60,
        }
    }
}

impl<'a> Into<Vec2d> for &'a CoordIJK {
    fn into(self) -> Vec2d {
        let i = self.i - self.k;
        let j = self.j - self.k;

        Vec2d {
            x: i as f64 - 5.0 * j as f64,
            y: j as f64 * M_SIN60,
        }
    }
}

impl Add for CoordIJK {
    type Output = Self;

    fn add(self, rhs: Self::Output) -> Self::Output {
        Self::Output::new(self.i + rhs.i, self.j + rhs.j, self.k + rhs.k)
    }
}

impl<'a, 'b> Add<&'b CoordIJK> for &'a CoordIJK {
    type Output = CoordIJK;

    fn add(self, rhs: &'b Self::Output) -> Self::Output {
        Self::Output::new(self.i + rhs.i, self.j + rhs.j, self.k + rhs.k)
    }
}

impl<'a, 'b> Add<&'b CoordIJK> for &'a mut CoordIJK {
    type Output = CoordIJK;

    fn add(self, rhs: &'b Self::Output) -> Self::Output {
        Self::Output::new(self.i + rhs.i, self.j + rhs.j, self.k + rhs.k)
    }
}

impl Sub for CoordIJK {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.i - rhs.i, self.j - rhs.j, self.k - rhs.k)
    }
}

impl<'a, 'b> Sub<&'b CoordIJK> for &'a CoordIJK {
    type Output = CoordIJK;

    fn sub(self, rhs: &'b Self::Output) -> Self::Output {
        Self::Output::new(self.i - rhs.i, self.j - rhs.j, self.k - rhs.k)
    }
}

impl AsRef<CoordIJK> for CoordIJK {
    fn as_ref(&self) -> &CoordIJK {
        return self;
    }
}

impl<'a> Into<Direction> for &'a mut CoordIJK {
    fn into(self) -> Direction {
        self.normalize();

        let mut digit = Direction::InvalidDigit;
        for i in Direction::CenterDigit as usize..Direction::InvalidDigit as usize {
            if self.matches(&UNIT_VECS[i]) {
                digit = i.into();
                break;
            }
        }

        digit
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub enum Direction {
    CenterDigit,
    KAxesDigit,
    JAxesDigit,
    JKAxesDigit,
    IAxesDigit,
    IKAxesDigit,
    IJAxesDigit,
    InvalidDigit,
    NumDigits, // = Self::InvalidDigit,
    PentagonSkippedDigit,
}

impl Direction {
    /// Rotate indexing digit 60 degrees counter-clockwise.
    pub(crate) fn rotate_60ccw(&self) -> Direction {
        match self {
            Self::KAxesDigit => Self::IKAxesDigit,
            Self::IKAxesDigit => Self::IAxesDigit,
            Self::IAxesDigit => Self::IJAxesDigit,
            Self::IJAxesDigit => Self::JAxesDigit,
            Self::JAxesDigit => Self::JKAxesDigit,
            Self::JKAxesDigit => Self::KAxesDigit,
            _ => self.clone(),
        }
    }

    /// Rotate indexing digit 60 degrees clockwise.
    pub(crate) fn rotate_60cw(&self) -> Direction {
        match self {
            Self::KAxesDigit => Self::JKAxesDigit,
            Self::JKAxesDigit => Self::JAxesDigit,
            Self::JAxesDigit => Self::IJAxesDigit,
            Self::IJAxesDigit => Self::IAxesDigit,
            Self::IAxesDigit => Self::IKAxesDigit,
            Self::IKAxesDigit => Self::KAxesDigit,
            _ => self.clone(),
        }
    }
}

impl Into<Direction> for usize {
    fn into(self) -> Direction {
        match self {
            0 => Direction::CenterDigit,
            1 => Direction::KAxesDigit,
            2 => Direction::JAxesDigit,
            3 => Direction::JKAxesDigit,
            4 => Direction::IAxesDigit,
            5 => Direction::IKAxesDigit,
            6 => Direction::IJAxesDigit,
            7 => Direction::InvalidDigit,
            _ => Direction::InvalidDigit,
        }
    }
}
