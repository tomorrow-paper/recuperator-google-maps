use super::{AddressComponent, Geometry};

#[derive(Deserialize, Debug, Clone)]
pub struct Address {
    pub address_components: Vec<AddressComponent>,
    pub formatted_address: String,
    pub geometry: Geometry, 
    pub place_id: String,
    pub types: Vec<String>
}