use crate::{country::Country, geo_json::GeoJSON};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Error},
    path::PathBuf,
};

const PATH: &str = "./data/";
const FILE_EXTENSION: &str = "json";

pub struct CountryInfo {
    pub country: String,
    pub countries: HashMap<String, Country>,
}

fn read_country_data_from_file(path: &PathBuf) -> Result<Country, Error> {
    let file = File::open(&path)?; // .expect(format!("Error in opening {}", &filename).as_str());
    let buf_reader = BufReader::new(file);
    // let country: Country = serde_json::from_reader(buf_reader)
    //     .expect(format!("Error in reading {}. Possibly Invalid JSON.", &filename).as_str());
    let country: Country = serde_json::from_reader(buf_reader)?;
    Ok(country)
}

impl CountryInfo {
    pub fn new(country: String) -> CountryInfo {
        let country = country.to_lowercase();
        let mut countries: HashMap<String, Country> = HashMap::new();

        // Read data from all the JSON files in ./data directory
        for entry in std::fs::read_dir(PATH).unwrap() {
            let entry = entry.unwrap();
            let metadata = entry.metadata().expect("Error in reading file metadata");

            if metadata.is_file() {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == FILE_EXTENSION {
                        let country = read_country_data_from_file(&path);
                        if let Ok(country) = country {
                            countries.insert(country.name.to_string().to_lowercase(), country);
                        } else {
                            // Log the error
                        }
                    }
                }
            }
        }

        CountryInfo { country, countries }
    }
}

impl CountryInfo {
    // Get the list of countries for the country specified in the constructor
    pub fn get_provinces(&self) -> Option<Vec<String>> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.provinces.clone();
        }
        return None;
    }

    // Get the list of alternative spellings for the country specified in the constructor
    pub fn get_alt_spellings(&self) -> Option<Vec<String>> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.alt_spellings.clone();
        }
        return None;
    }

    // Get the wikipedia link for the country specified in the constructor
    pub fn get_wiki(&self) -> Option<String> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.wiki.clone();
        }
        return None;
    }

    // Get the list of translations for the country specified in the constructor
    pub fn get_translations(&self) -> Option<HashMap<String, String>> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.translations.clone();
        }
        return None;
    }

    // Get the top level domains for the country specified in the constructor
    pub fn get_tlds(&self) -> Option<Vec<String>> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.tlds.clone();
        }
        return None;
    }

    // Get timezones for the country specified in the constructor
    pub fn get_timezones(&self) -> Option<Vec<String>> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.timezones.clone();
        }
        return None;
    }

    // Get subregion for the country specified in the constructor
    pub fn get_subregion(&self) -> Option<String> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.subregion.clone();
        }
        return None;
    }

    // Get region for the country specified in the constructor
    pub fn get_region(&self) -> Option<String> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.region.clone();
        }
        return None;
    }

    // Get population of the country specified in the constructor
    pub fn get_population(&self) -> Option<u64> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.population;
        }
        return None;
    }

    // Get native name of the country specified in the constructor
    pub fn get_native_name(&self) -> Option<String> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.native_name.clone();
        }
        return None;
    }

    // Get lat long of the country specified in the constructor
    pub fn get_lat_long(&self) -> Option<Vec<f64>> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.lat_lng.clone();
        }
        return None;
    }

    // Get languages of the country specified in the constructor
    pub fn get_languages(&self) -> Option<Vec<String>> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.languages.clone();
        }
        return None;
    }

    // Get geo-json of the country specified in the constructor
    pub fn get_geo_json(&self) -> Option<GeoJSON> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.geo_json.clone();
        }
        return None;
    }

    // Get SVG link of flag of the country specified in the constructor
    pub fn get_flag(&self) -> Option<String> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.flag.clone();
        }
        return None;
    }

    // Get demonym of the country specified in the constructor
    pub fn get_demonym(&self) -> Option<String> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.demonym.clone();
        }
        return None;
    }

    // Get currencies of the country specified in the constructor
    pub fn get_currencies(&self) -> Option<Vec<String>> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.currencies.clone();
        }
        return None;
    }

    // Get latitude and longitude of the capital of the country specified in the constructor
    pub fn get_capital_lat_long(&self) -> Option<Vec<f64>> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.capital_latlng.clone();
        }
        return None;
    }

    // Get capital of the country specified in the constructor
    pub fn get_capital(&self) -> Option<String> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.capital.clone();
        }
        return None;
    }

    // Get calling codes of the country specified in the constructor
    pub fn get_calling_codes(&self) -> Option<Vec<String>> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.calling_codes.clone();
        }
        return None;
    }

    // Get borders of the country specified in the constructor
    pub fn get_borders(&self) -> Option<Vec<String>> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.borders.clone();
        }
        return None;
    }

    // Get area of the country specified in the constructor
    pub fn get_area(&self) -> Option<u64> {
        if let Some(country) = self.countries.get(&self.country) {
            return country.area.clone();
        }
        return None;
    }

    // Get ISO codes of the country specified in the constructor
    pub fn get_iso_codes(&self, alpha: String) -> Option<String> {
        if let Some(country) = self.countries.get(&self.country) {
            let iso_codes = country.iso.clone().unwrap();
            if alpha == "2" {
                return iso_codes.get("alpha2").map(|v| v.to_string());
            } else if alpha == "3" {
                return iso_codes.get("alpha3").map(|v| v.to_string());
            }
        }
        return None;
    }
}
