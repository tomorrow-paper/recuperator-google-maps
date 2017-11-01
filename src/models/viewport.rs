use super::Coordinates;

#[derive(Deserialize, Debug, Clone)]
pub struct Viewport {
    pub northeast: Coordinates,
    pub southwest: Coordinates
}