use super::Address;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    pub results: Vec<Address>,
    pub status: String
}