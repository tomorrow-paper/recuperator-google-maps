extern crate tomorrow_core;
extern crate tomorrow_recuperator;
extern crate tomorrow_http;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

pub mod models;

mod request;
pub use self::request::GoogleMapsRequest;

mod response;
pub use self::response::GoogleMapsResponse;

mod recuperator;
pub use self::recuperator::GoogleMapsRecuperator;

#[cfg(test)]
mod tests {

    use tomorrow_http::mock::MockClient;
    use tomorrow_recuperator::Recuperator;

    use super::*;

    #[test]
    fn test() {
        let query = "42 rue d'Ulm";
        let client = MockClient::with_json(r#"{"results": [{"address_components": [{"long_name": "42","short_name": "42","types": ["street_number"]},{"long_name": "Rue d'Ulm","short_name": "Rue d'Ulm","types": ["route"]},{"long_name": "Paris","short_name": "Paris","types": ["locality","political"]},{"long_name": "Paris","short_name": "Paris","types": ["administrative_area_level_2","political"]},{"long_name": "Île-de-France","short_name": "Île-de-France","types": ["administrative_area_level_1","political"]},{"long_name": "France","short_name": "FR","types": ["country","political"]},{"long_name": "75005","short_name": "75005","types": ["postal_code"]}],"formatted_address": "42 Rue d'Ulm, 75005 Paris, France","geometry": {"location": {"lat": 48.8426125,"lng": 2.3440488},"location_type": "ROOFTOP","viewport": {"northeast": {"lat": 48.8439614802915,"lng": 2.345397780291502},"southwest": {"lat": 48.84126351970851,"lng": 2.342699819708498}}},"place_id": "ChIJ7V00w-lx5kcRc-zyAkaMsNM","types": ["street_address"]},{"address_components": [{"long_name": "42","short_name": "42","types": ["street_number"]},{"long_name": "Rue d'Ulm","short_name": "Rue d'Ulm","types": ["route"]},{"long_name": "Montluçon","short_name": "Montluçon","types": ["locality","political"]},{"long_name": "Allier","short_name": "Allier","types": ["administrative_area_level_2","political"]},{"long_name": "Auvergne-Rhône-Alpes","short_name": "Auvergne-Rhône-Alpes","types": ["administrative_area_level_1","political"]},{"long_name": "France","short_name": "FR","types": ["country","political"]},{"long_name": "03100","short_name": "03100","types": ["postal_code"]}],"formatted_address": "42 Rue d'Ulm, 03100 Montluçon, France","geometry": {"bounds": {"northeast": {"lat": 46.3550739,"lng": 2.5896708},"southwest": {"lat": 46.3549832,"lng": 2.5895017}},"location": {"lat": 46.3550286,"lng": 2.5895863},"location_type": "ROOFTOP","viewport": {"northeast": {"lat": 46.3563775302915,"lng": 2.590935230291502},"southwest": {"lat": 46.3536795697085,"lng": 2.588237269708498}}},"place_id": "ChIJo4UmVIyn8EcRn6kemfjEzqE","types": ["premise"]}],"status": "OK"}"#);

        let request = GoogleMapsRequest::new(query);
        let recuperator = GoogleMapsRecuperator::new(client);

        let response = recuperator.compute(request);
        assert!(response.is_ok());

        let record = response.unwrap().record;
        assert_eq!(record.results.len(), 2);
        assert_eq!(record.results[0].formatted_address, "42 Rue d'Ulm, 75005 Paris, France");
    }
}