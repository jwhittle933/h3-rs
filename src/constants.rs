pub const M_PI: f64 = 3.14159265358979323846;

/// 2.0 * PI
pub const M_2PI: f64 = 6.28318530717958647692528676655900576839433;
// PI / 2.0
pub const M_PI_2: f64 = 1.5707963267948966;

/// Threshold Epsilon
pub const EPSILON: f64 = 0.0000000000000001;
/// sqrt(3) / 2.0
pub const M_SQRT3_2: f64 = 0.8660254037844386467637231707529361834714;
/// sin(60')
pub const M_SIN60: f64 = M_SQRT3_2;
/// pi / 180
pub const M_PI_180: f64 = 0.0174532925199432957692369076848861271111;
pub const M_180_PI: f64 = 57.29577951308232087679815481410517033240547;

/// Earth radius in kilometers using WGS84 authalic radius.
pub const EARTH_RADIUS_KM: f64 = 6371.007180918475;

pub const MAX_H3_RES: u64 = 15;

/// The number of faces on an icosahedron.
pub const NUM_ICOSA_FACES: usize = 20;
/// The number of H3 base cells.
pub const NUM_BASE_CELLS: usize = 122;
/// The number of vertices in a hexagon.
pub const NUM_HEX_VERTS: usize = 6;
/// The number of vertices in a pentagon.
pub const NUM_PENT_VERTS: usize = 5;
/// The number of pentagons per resolution.
pub const NUM_PENTAGONS: u8 = 12;

/// Scaling factor from hex2d resolution 0 unit length
/// (or distance between adjacent cell center points
/// on the plane) to gnomonic unit length.
pub const RES0_U_GNOMONIC: f64 = 0.38196601125010500003;

/** rotation angle between Class II and Class III resolution axes
 * (asin(sqrt(3.0 / 28.0))) */
pub const M_AP7_ROT_RADS: f64 = 0.333473172251832115336090755351601070065900389;
