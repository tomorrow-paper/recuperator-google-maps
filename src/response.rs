use tomorrow_recuperator::Response;

use ::models::Record;

pub struct GoogleMapsResponse {
    pub record: Record
}

impl GoogleMapsResponse {

    pub fn new(record: Record) -> Self {
        GoogleMapsResponse {
            record: record
        }
    }
}

impl Response for GoogleMapsResponse {}