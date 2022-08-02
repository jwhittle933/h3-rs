use crate::{
    constants::{EARTH_RADIUS_KM, EPSILON, MAX_H3_RES, M_180_PI, M_2PI, M_PI, M_PI_180, M_PI_2},
    error::H3ErrorCode,
    math::ipow,
    CellBoundary, H3Index,
};
use libm;

pub const EPSILON_DEG: f64 = 0.000000001;
pub const EPSILON_RAD: f64 = EPSILON_DEG / M_PI_180;

/// Latitude and Longitude in radians.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

impl LatLng {
    /// Create a new [`LatLng`] from radians.
    pub fn new_radians(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }

    /// Create a new [`LatLng`] from degrees.
    pub fn new_degress(lat: f64, lng: f64) -> Self {
        Self::new_radians(deg_to_rads(lat), deg_to_rads(lng))
    }

    /// Determines if the components of two spherical coordinates are within some
    /// threshold distance of each other.
    pub fn geo_almost_equal_threshold(&self, p2: &LatLng, threshold: f64) -> bool {
        libm::fabs(self.lat - p2.lat) < threshold && libm::fabs(self.lng - p2.lng) < threshold
    }

    /// Determines if the components of two spherical coordinates are within our
    /// standard epsilon distance of each other.
    pub fn geo_almost_equal(&self, p2: &LatLng) -> bool {
        self.geo_almost_equal_threshold(p2, EPSILON_RAD)
    }

    /// The great circle distance in radians between two spherical coordinates.
    ///
    /// This function uses the Haversine formula.
    /// See:
    ///   https://en.wikipedia.org/wiki/Haversine_formula
    ///   https://www.movable-type.co.uk/scripts/latlong.html
    pub fn great_circle_distance_rads(&self, rhs: &Self) -> f64 {
        let sin_lat = libm::sin((rhs.lat - self.lat) / 2.);
        let sin_lng = libm::sin((rhs.lng - self.lng) / 2.);
        let a = sin_lat * sin_lng + libm::cos(self.lat) * libm::cos(rhs.lat) * sin_lat * sin_lng;

        2.0 * libm::atan2(libm::sqrt(a), libm::sqrt(1. - a))
    }

    /// The great circle distance in kilometers between two spherical coordinates.
    pub fn great_circle_distance_km(&self, rhs: &Self) -> f64 {
        self.great_circle_distance_rads(rhs) * EARTH_RADIUS_KM
    }

    /// The great circle distance in meters between two spherical coordinates.
    pub fn great_circle_distance_m(&self, rhs: &Self) -> f64 {
        self.great_circle_distance_km(rhs) * 1000.
    }

    /// Determines the azimuth to `rhs` from `self` in radians.
    fn geo_azimuth_rads(&self, rhs: &Self) -> f64 {
        libm::atan2(
            libm::cos(rhs.lat) * libm::sin(rhs.lng - self.lng),
            libm::cos(self.lat) * libm::sin(rhs.lat)
                - libm::sin(self.lat)
                    * libm::cos(rhs.lat)
                    * libm::cos(rhs.lat)
                    * libm::cos(rhs.lng - self.lng),
        )
    }

    /// Computes the the point on the sphere a specified azumuth and distance from
    /// another point.
    fn geo_azimuth_distance_rads(&self, mut az: f64, distance: f64) -> Self {
        let mut out = LatLng { lat: 0., lng: 0. };
        if distance < EPSILON {
            return self.clone();
        }

        az = pos_angle_rads(az);

        if az < EPSILON || libm::fabs(az - M_PI) < EPSILON {
            out.lat = if az < EPSILON {
                self.lat + distance
            } else {
                self.lat - distance
            };

            if libm::fabs(out.lat - M_PI_2) < EPSILON {
                out.lat = M_PI_2;
                out.lng = 0.;
            } else if libm::fabs(out.lat + M_PI_2) < EPSILON {
                out.lat = -M_PI_2;
                out.lng = 0.;
            } else {
                out.lng = constrain_lng(self.lng);
            }
        } else {
            let mut sin_lat = libm::sin(self.lat) * libm::cos(distance)
                + libm::cos(self.lat) * libm::sin(distance) * libm::cos(az);

            if sin_lat > 1. {
                sin_lat = 1.;
            }
            if sin_lat < -1. {
                sin_lat = -1.;
            }
            out.lat = libm::asin(sin_lat);

            if libm::fabs(out.lat - M_PI_2) < EPSILON {
                out.lat = M_PI_2;
                out.lng = 0.;
            } else if libm::fabs(out.lat + M_PI_2) < EPSILON {
                out.lat = -M_PI_2;
                out.lng = 0.;
            } else {
                let mut sin_lng = libm::sin(az) * libm::sin(distance) / libm::cos(out.lat);
                let mut cos_lng = (libm::cos(distance) - libm::sin(self.lat) * libm::sin(out.lat))
                    / libm::cos(self.lat)
                    / libm::cos(out.lat);

                if sin_lng > 1. {
                    sin_lng = 1.;
                }
                if sin_lng < -1. {
                    sin_lng = -1.;
                }
                if cos_lng > 1. {
                    cos_lng = 1.;
                }
                if cos_lng < -1. {
                    cos_lng = -1.;
                }

                out.lng = constrain_lng(self.lng + libm::atan2(sin_lng, cos_lng));
            }
        }

        out
    }
}

pub fn deg_to_rads(degrees: f64) -> f64 {
    degrees * M_PI_180
}

pub fn rads_to_deg(rads: f64) -> f64 {
    rads * M_180_PI
}

/// Makes sure the latitudes are in the proper bounds.
pub fn constrain_lat(mut lat: f64) -> f64 {
    while lat > M_PI_2 {
        lat = lat - M_PI
    }

    lat
}

/// Make sure the longitudes are the in the proper bounds.
pub fn constrain_lng(mut lng: f64) -> f64 {
    while lng > M_PI {
        lng = lng - (2.0 * M_PI);
    }

    while lng < -M_PI {
        lng = lng + (2.0 * M_PI)
    }

    lng
}

pub fn hexagon_area_average_m2(resolution: i32) -> Result<f64, H3ErrorCode> {
    const AREAS: [f64; 16] = [
        4.357449416078390e+12,
        6.097884417941339e+11,
        8.680178039899731e+10,
        1.239343465508818e+10,
        1.770347654491309e+09,
        2.529038581819452e+08,
        3.612906216441250e+07,
        5.161293359717198e+06,
        7.373275975944188e+05,
        1.053325134272069e+05,
        1.504750190766437e+04,
        2.149643129451882e+03,
        3.070918756316063e+02,
        4.387026794728301e+01,
        6.267181135324322e+00,
        8.953115907605802e-01,
    ];

    if resolution < 0 || resolution > MAX_H3_RES {
        Err(H3ErrorCode::Domain)
    } else {
        Ok(AREAS[resolution as usize])
    }
}

pub fn hexagon_edge_length_average_km(resolution: i32) -> Result<f64, H3ErrorCode> {
    const LENS: [f64; 16] = [
        1107.712591,
        418.6760055,
        158.2446558,
        59.81085794,
        22.6063794,
        8.544408276,
        3.229482772,
        1.220629759,
        0.461354684,
        0.174375668,
        0.065907807,
        0.024910561,
        0.009415526,
        0.003559893,
        0.001348575,
        0.000509713,
    ];

    if resolution < 0 || resolution > MAX_H3_RES {
        Err(H3ErrorCode::Domain)
    } else {
        Ok(LENS[resolution as usize])
    }
}

pub fn hexagon_edge_length_average_m(resolution: i32) -> Result<f64, H3ErrorCode> {
    const LENS: [f64; 16] = [
        1107712.591,
        418676.0055,
        158244.6558,
        59810.85794,
        22606.3794,
        8544.408276,
        3229.482772,
        1220.629759,
        461.3546837,
        174.3756681,
        65.90780749,
        24.9105614,
        9.415526211,
        3.559893033,
        1.348574562,
        0.509713273,
    ];

    if resolution < 0 || resolution > MAX_H3_RES {
        Err(H3ErrorCode::Domain)
    } else {
        Ok(LENS[resolution as usize])
    }
}

pub fn num_cells(resolution: i64) -> Result<i64, H3ErrorCode> {
    if resolution < 0 || resolution > MAX_H3_RES as i64 {
        Err(H3ErrorCode::Domain)
    } else {
        Ok(2 + 120 * ipow(7, resolution))
    }
}

/// Surface area in radians^2 of spherical triangle on unit sphere.
/// For the math, see:
///   https://en.wikipedia.org/wiki/Spherical_trigonometry#Area_and_spherical_excess
pub fn triangle_edge_lengths_to_area(a: f64, b: f64, c: f64) -> f64 {
    let mut s = (a + b + c) / 2.;

    let a = (s - a) / 2.;
    let b = (s - b) / 2.;
    let c = (s - c) / 2.;
    s = s / 2.;

    4. * libm::atan(libm::sqrt(s) * libm::tan(a) * libm::tan(b) * libm::tan(c))
}

/// Compute area in radians^2 of a spherical triangle, given its vertices.
pub(crate) fn triangle_area(a: &LatLng, b: &LatLng, c: &LatLng) -> f64 {
    triangle_edge_lengths_to_area(
        a.great_circle_distance_rads(b),
        b.great_circle_distance_rads(c),
        c.great_circle_distance_rads(a),
    )
}

/// Area of an H3 Cell in radians^2.
///
/// The area is calculated by breaking the cell into spherical triangles and
/// summing up their areas. Note that some H3 cells (hexagons and pentagons)
/// are irregular, and have more than 6 or 5 sides.
pub fn cell_area_rads_2(cell: &H3Index) -> Result<f64, H3ErrorCode> {
    let lat_lng: LatLng = cell.try_into()?;
    let boundary: CellBoundary = cell.try_into()?;
    let mut area = 0.;

    for i in 0..boundary.num_verts {
        let j = (i + 1) % boundary.num_verts;
        area += triangle_area(
            &boundary.verts[i as usize],
            &boundary.verts[j as usize],
            &lat_lng,
        )
    }

    Ok(area)
}

/// Normalizes the radian to a value between 0.0 and two PI.
pub(crate) fn pos_angle_rads(rads: f64) -> f64 {
    let mut tmp = if rads < 0.0 { rads + M_2PI } else { rads };
    if rads >= M_2PI {
        tmp -= M_2PI;
    }

    tmp
}
