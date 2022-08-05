pub mod algos;
pub mod base_cell;
pub mod constants;
pub mod coordijk;
pub mod error;
pub mod faceijk;
pub mod index;
pub mod latlng;
pub mod math;
pub mod vec2d;
pub mod vec3d;

pub use coordijk::CoordIJK;
pub use index::H3Index;
pub use latlng::LatLng;

pub const MAX_CELL_BOUNDARY_VERTS: usize = 10;

pub fn is_resolution_classIII(res: usize) -> bool {
    res % 2 > 0
}

/// Cell boundary in latitude/longitude.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct CellBoundary {
    num_verts: usize,
    pub verts: [LatLng; MAX_CELL_BOUNDARY_VERTS],
}

impl CellBoundary {
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a [`CellBoundary`] with `verts`.
    pub fn with_verts(verts: [LatLng; MAX_CELL_BOUNDARY_VERTS]) -> Self {
        Self {
            num_verts: verts.len(),
            verts,
        }
    }

    /// Pushes a [`LatLng`] into `verts`. Noop if `verts` is full.
    pub fn push_vert_unchecked(&mut self, v: LatLng) {
        if self.num_verts != MAX_CELL_BOUNDARY_VERTS {
            let _ = self.push_vert(v);
        }
    }

    /// Pushes a [`LatLng`] into `verts`.  Takes ownership of `v` if
    /// the it can fit, other wise returns ownership of `v` to the caller
    /// wrapped in an error.
    pub fn push_vert(&mut self, v: LatLng) -> Result<usize, LatLng> {
        if self.num_verts != MAX_CELL_BOUNDARY_VERTS {
            self.verts[self.num_verts] = v;
            self.num_verts += 1;
            Ok(self.num_verts)
        } else {
            Err(v)
        }
    }

    // Reports the number of `verts`.
    pub fn len(&self) -> usize {
        self.num_verts
    }
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
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct LinkedGeoLoop {
    pub first: Option<Box<LinkedLatLng>>,
    pub last: Option<Box<LinkedLatLng>>,
    pub next: Option<Box<LinkedGeoLoop>>,
}

// TODO: scrap the linked lists.
/// A polygon node in a linked geo structure, part of a linked list.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct LinkedGeoPolygon {
    pub first: Option<Box<LinkedGeoLoop>>,
    pub last: Option<Box<LinkedGeoLoop>>,
    pub next: Option<Box<LinkedGeoPolygon>>,
}

/// IJ hexagon coordinates.
///
/// Each axis is spaced 120 degrees apart.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct CoordIJ {
    pub i: i32,
    pub j: i32,
}
