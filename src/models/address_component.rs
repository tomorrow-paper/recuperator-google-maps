#[derive(Deserialize, Debug, Clone)]
pub struct AddressComponent {
    pub long_name: String,
    pub short_name: String,
    pub types: Vec<String>
}