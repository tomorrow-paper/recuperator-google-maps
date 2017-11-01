use super::Coordinates;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Viewport {
    pub northeast: Coordinates,
    pub southwest: Coordinates
}