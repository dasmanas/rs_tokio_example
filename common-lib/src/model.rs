use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Zipcode {
    #[serde(rename = "post code")]
    pub post_code: String,
    pub country: String,
    #[serde(rename = "country abbreviation")]
    pub country_abbreviation: String,
    pub places: Vec<Place>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Place {
    #[serde(rename = "place name")]
    pub place_name: String,
    pub longitude: String,
    pub state: String,
    #[serde(rename = "state abbreviation")]
    pub state_abbreviation: String,
    pub latitude: String,
}
