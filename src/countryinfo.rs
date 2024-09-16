use crate::country::Country;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Error},
    path::PathBuf,
};

const PATH: &str = "./data/";

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
                    if extension == "json" {
                        // let filename = path.file_name().unwrap().to_str().unwrap();
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
