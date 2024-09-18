//! Contains the `Country` struct which is used to represent a country.
use crate::geo_json::GeoJSON;
use serde_derive::Deserialize;
use std::collections::HashMap;

/// A struct representing a country.
#[derive(Clone, Deserialize, Debug)]
pub struct Country {
    pub name: String,
    #[serde(rename = "altSpellings")]
    pub alt_spellings: Option<Vec<String>>,
    pub area: Option<u64>,
    pub borders: Option<Vec<String>>,
    #[serde(rename = "callingCodes")]
    pub calling_codes: Option<Vec<String>>,
    pub capital: Option<String>,
    #[serde(rename = "capital_latlng")]
    pub capital_latlng: Option<Vec<f64>>,
    pub currencies: Option<Vec<String>>,
    pub demonym: Option<String>,
    pub flag: Option<String>,
    #[serde(rename = "geoJSON")]
    pub geo_json: Option<GeoJSON>,
    #[serde(rename = "ISO")]
    pub iso: Option<HashMap<String, String>>,
    pub languages: Option<Vec<String>>,
    #[serde(rename = "latlng")]
    pub lat_lng: Option<Vec<f64>>,
    #[serde(rename = "nativeName")]
    pub native_name: Option<String>,
    pub population: Option<u64>,
    pub provinces: Option<Vec<String>>,
    pub region: Option<String>,
    pub subregion: Option<String>,
    pub timezones: Option<Vec<String>>,
    #[serde(rename = "tld")]
    pub tlds: Option<Vec<String>>,
    pub translations: Option<HashMap<String, String>>,
    pub wiki: Option<String>,
}
