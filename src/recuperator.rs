use tomorrow_core::Result;
use tomorrow_http::Builder;
use tomorrow_http::json::*;
use tomorrow_recuperator::Recuperator;

use ::{GoogleMapsRequest, GoogleMapsResponse};
use ::models::Record;

const API_URL: &'static str = "https://maps.google.com/maps/api/geocode";

pub struct GoogleMapsRecuperator<T> where T: Requester {
    requester: T
}

impl <T> GoogleMapsRecuperator<T> where T: Requester {
    
    pub fn new(requester: T) -> Self {
        GoogleMapsRecuperator {
            requester: requester
        }
    }
}

impl <T> Recuperator<GoogleMapsRequest, GoogleMapsResponse> for GoogleMapsRecuperator<T> where T: Requester {

    fn compute(&self, request: GoogleMapsRequest) -> Result<GoogleMapsResponse> {
        let record = self.requester.request::<Record>(&request.query)?;
        let response = GoogleMapsResponse::new(record);

        Ok(response)
    }
}

impl Default for GoogleMapsRecuperator<Client> {

    fn default() -> Self {
        let client: Client = Builder::https(API_URL).into();
        GoogleMapsRecuperator::new(client)
    }
}