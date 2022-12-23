pub const M_PI: f64 = 3.14159265358979323846;
/// pi / 2.0
pub const M_PI_2: f64 = M_PI / 2.;
/// 2.0 * pi
pub const M_2PI: f64 = 2. * M_PI;
/// pi / 180
pub const M_PI_180: f64 = M_PI / 180.;
/// 180 / pi
pub const M_180_PI: f64 = 180. / M_PI;
/// Threshold Epsilon
pub const EPSILON: f64 = 0.0000000000000001;
/// sqrt(3) / 2.0
pub const M_SQRT3_2: f64 = 0.8660254037844386467637231707529361834714;
/// sin(60')
pub const M_SIN60: f64 = M_SQRT3_2;

/// Rotation angle between Class II and Class III resolution axes.
/// asin(sqrt(3.0 / 28.0))
pub const M_AP7_ROT_RADS: f64 = 0.333473172251832115336090755351601070065900389;
/// sin(M_AP7_ROT_RADS)
pub const M_SIN_AP7_ROT: f64 = 0.3273268353539885718950318;
/// cos(M_AP7_ROT_RADS)
pub const M_COS_AP7_ROT: f64 = 0.9449111825230680680167902;

/// Earth radius in kilometers using WGS84 authalic radius
pub const EARTH_RADIUS_KM: f64 = 6371.007180918475;

/// Scaling factor from hex2d resolution 0 unit length
/// (or distance between adjacent cell center points
/// on the plane) to gnomonic unit length.
pub const RES0_U_GNOMONIC: f64 = 0.38196601125010500003;

/// Max H3 resolution; H3 version 1 has 16 resolutions, numbered 0 through 15
pub const MAX_H3_RES: usize = 15;

/// The number of faces on an icosahedron
pub const NUM_ICOSA_FACES: usize = 20;
/// The number of H3 base cells
pub const NUM_BASE_CELLS: usize = 122;
/// The number of vertices in a hexagon
pub const NUM_HEX_VERTS: usize = 6;
/// The number of vertices in a pentagon
pub const NUM_PENT_VERTS: usize = 5;
/// The number of pentagons per resolution
pub const NUM_PENTAGONS: usize = 12;

/// H3 index modes
pub const H3_CELL_MODE: usize = 1;
pub const H3_DIRECTEDEDGE_MODE: usize = 2;
pub const H3_EDGE_MODE: usize = 3;
pub const H3_VERTEX_MODE: usize = 4;
