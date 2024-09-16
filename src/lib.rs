pub mod country;
pub mod countryinfo;
pub mod geo_json;

#[cfg(test)]
mod tests {
    use countryinfo::CountryInfo;

    use super::*;

    #[test]
    fn load_data() {
        // This test is to check if the loading of data is working fine
        let country_info = CountryInfo::new("India".to_string());
        let countries_length = country_info.countries.len();
        assert_eq!(countries_length > 1, true);
    }
}
