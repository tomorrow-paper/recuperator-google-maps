use super::Address;

#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    pub results: Vec<Address>,
    pub status: String
}