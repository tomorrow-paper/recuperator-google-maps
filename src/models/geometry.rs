use super::{Viewport, Coordinates};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Geometry {
    pub location: Coordinates,
    pub location_type: String,
    pub viewport: Viewport,
}