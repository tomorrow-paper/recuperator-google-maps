use tomorrow_recuperator::Request;

pub struct GoogleMapsRequest {
    pub query: String
}

impl GoogleMapsRequest {

    pub fn new(query: &str) -> Self {
        GoogleMapsRequest {
            query: format!("json?address={}", query)
        }
    }
}

impl Request for GoogleMapsRequest {}

