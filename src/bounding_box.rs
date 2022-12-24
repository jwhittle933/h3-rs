use crate::latlng::LatLng;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct BoundingBox {
    /// North Latitude
    pub north: f64,
    /// South Latitude
    pub south: f64,
    /// East Latitude
    pub east: f64,
    /// West Latitude
    pub west: f64,
}

impl BoundingBox {
    pub fn is_transmeridian(&self) -> bool {
        false
    }

    pub fn center(&self) -> LatLng {
        LatLng { lat: 0., lng: 0. }
    }

    pub fn contains(&self, latlng: &LatLng) -> bool {
        false
    }

    // bbox_hex_estimate
    // line_hex_estimate
}
