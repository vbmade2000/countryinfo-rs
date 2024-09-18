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

    #[test]
    fn get_countries() {
        let country_info = CountryInfo::new("India".to_string());
        let provinces_length = country_info.get_provinces().unwrap().len();
        assert_eq!(provinces_length, 36);
    }

    #[test]
    fn get_alt_spellings() {
        let country_info = CountryInfo::new("India".to_string());
        let alt_spellings_length = country_info.get_alt_spellings().unwrap().len();
        assert_eq!(alt_spellings_length, 4);
    }

    #[test]
    fn get_wiki() {
        let country_info = CountryInfo::new("India".to_string());
        let wiki_link = country_info.get_wiki().unwrap();
        assert_eq!(wiki_link, "http://en.wikipedia.org/wiki/india".to_string());
    }

    #[test]
    fn get_translations() {
        let country_info = CountryInfo::new("India".to_string());
        let translations_length = country_info.get_translations().unwrap().len();
        assert_eq!(translations_length > 1, true);
    }

    #[test]
    fn get_tlds() {
        let country_info = CountryInfo::new("India".to_string());
        let tlds_length = country_info.get_tlds().unwrap().len();
        assert_eq!(tlds_length == 1, true);
    }
}
