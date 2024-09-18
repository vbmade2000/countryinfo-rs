use crate::country::Country;
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
}
