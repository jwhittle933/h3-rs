pub mod algos;
pub mod constants;
pub mod coordijk;
pub mod error;
pub mod faceijk;
pub mod index;
pub mod latlng;
pub mod math;
pub mod vec2d;

pub use coordijk::CoordIJK;
pub use index::H3Index;
pub use latlng::LatLng;

pub const MAX_CELL_BOUNDARY_VERTS: usize = 10;

/// Cell boundary in latitude/longitude.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CellBoundary {
    pub num_verts: i32,
    pub verts: [LatLng; MAX_CELL_BOUNDARY_VERTS],
}

/// Similar to [`CellBoundary`], but holds a reference to [`LatLng`].
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GeoLoop<'a> {
    pub num_verts: i32,
    pub verts: &'a LatLng,
}

/// Simplified core of GeoJSON Polygon coordinates definition.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GeoPolygon<'a> {
    pub geo_loop: GeoLoop<'a>,
    pub num_holes: i32,
    pub holes: &'a GeoLoop<'a>,
}

/// Simplified core of GeoJSON MultiPolygon coordinates definition.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GeoMultiPolygon<'a> {
    pub num_polygons: i32,
    pub polygons: &'a GeoPolygon<'a>,
}

// TODO: scrap the linked lists.
/// A coordinate node in a linked geo structure, part of a linked list.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LinkedLatLng {
    pub vertex: LatLng,
    pub next: Option<Box<LinkedLatLng>>,
}

// TODO: scrap the linked lists.
/// A loop node in a linked geo structure, part of a linked list.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LinkedGeoLoop {
    pub first: Option<Box<LinkedLatLng>>,
    pub last: Option<Box<LinkedLatLng>>,
    pub next: Option<Box<LinkedGeoLoop>>,
}

// TODO: scrap the linked lists.
/// A polygon node in a linked geo structure, part of a linked list.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LinkedGeoPolygon {
    pub first: Option<Box<LinkedGeoLoop>>,
    pub last: Option<Box<LinkedGeoLoop>>,
    pub next: Option<Box<LinkedGeoPolygon>>,
}

/// IJ hexagon coordinates.
///
/// Each axis is spaced 120 degrees apart.
pub struct CoordIJ {
    pub i: i32,
    pub j: i32,
}
