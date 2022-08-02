use crate::{
    error::{H3Error, H3ErrorCode},
    faceijk::FaceIJK,
    CellBoundary, LatLng,
};

/// Identifier for an object (cell, edge, etc) in the H3System.
/// The H3Index fits within a 64-bit unsigned integer.
pub struct H3Index(u64);

impl H3Index {
    pub fn new(i: u64) -> Self {
        Self(i)
    }
}

/// Determines the spherical coordinates of the center point of an H3 index.
impl TryInto<LatLng> for H3Index {
    type Error = H3ErrorCode;

    fn try_into(self) -> Result<LatLng, Self::Error> {
        (&self).try_into()
    }
}

/// Determines the spherical coordinates of the center point of an H3 index.
impl TryInto<LatLng> for &H3Index {
    type Error = H3ErrorCode;

    fn try_into(self) -> Result<LatLng, Self::Error> {
        TryInto::<FaceIJK>::try_into(self)?.try_into()
    }
}

impl TryInto<CellBoundary> for H3Index {
    type Error = H3ErrorCode;

    fn try_into(self) -> Result<CellBoundary, Self::Error> {
        (&self).try_into()
    }
}

impl TryInto<CellBoundary> for &H3Index {
    type Error = H3ErrorCode;

    fn try_into(self) -> Result<CellBoundary, Self::Error> {
        //
    }
}

impl TryInto<FaceIJK> for H3Index {
    type Error = H3ErrorCode;

    fn try_into(self) -> Result<FaceIJK, Self::Error> {
        (&self).try_into()
    }
}

impl TryInto<FaceIJK> for &H3Index {
    type Error = H3ErrorCode;

    fn try_into(self) -> Result<FaceIJK, Self::Error> {
        //
    }
}
