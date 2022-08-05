use std::ops::Sub;

use crate::{
    constants::{EPSILON, M_AP7_ROT_RADS, RES0_U_GNOMONIC},
    faceijk::{FACE_AXES_AZ_RADS_CII, FACE_CENTER_GEO},
    is_resolution_classIII,
    latlng::{pos_angle_rads, LatLng},
};

pub const M_SQRT7: f64 = 2.6457513110645905905016157536392604257102;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Calculates the magnitude of a 2D cartesian vector.
    pub fn mag(&self) -> f64 {
        libm::sqrt(self.x * self.x + self.y * self.y)
    }

    ///Determines the center point in the spherical coordinates of a cell given by 2D
    /// hex coordinates on a particular isocahedral face.
    pub(crate) fn to_geo(&self, face: i32, resolution: usize, substrate: i32) -> LatLng {
        let mut r = self.mag();

        let face_center = FACE_CENTER_GEO[face as usize];
        if r < EPSILON {
            return face_center;
        }

        let theta = libm::atan2(self.y, self.x);
        for i in 0..resolution {
            r /= M_SQRT7;
        }

        // scale accordingly if this is a substrate grid
        if substrate > 0 {
            r /= 3.;
            if is_resolution_classIII(resolution) {
                r /= M_SQRT7;
            }
        }

        r *= RES0_U_GNOMONIC;

        // perform inverse gnomonic scaling of r
        r = libm::atan(r);

        // adjust theta fo Class III
        // if a substrate grid, then it's already been adjusted for Class III
        if substrate != 0 && is_resolution_classIII(resolution) {
            theta = pos_angle_rads(theta + M_AP7_ROT_RADS);
        }

        theta = pos_angle_rads(FACE_AXES_AZ_RADS_CII[face as usize][0] - theta);
        face_center.geo_azimuth_distance_rads(theta, r)
    }

    /// Finds the intersection between two lines. Assumes that the lines intersect
    /// and that the intersection is not at an endpoint of either line.
    pub fn intersect(p0: &Vec2d, p1: &Vec2d, p2: &Vec2d, p3: &Vec2d) -> Vec2d {
        let s1 = p1 - p0;
        let s2 = p3 - p2;

        let t = (s2.x * (p0.y - p2.y) - s2.y * (p0.x - p2.x)) / (-s2.x * s1.y + s1.x * s2.y);

        Vec2d {
            x: p0.x + (t * s1.x),
            y: p0.y + (t * s1.y),
        }
    }
}

impl Sub<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn sub(self, rhs: Vec2d) -> Self::Output {
        &self - &rhs
    }
}

impl Sub<&Vec2d> for &Vec2d {
    type Output = Vec2d;

    fn sub(self, rhs: &Vec2d) -> Self::Output {
        Vec2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
