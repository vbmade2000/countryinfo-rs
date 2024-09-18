pub mod country;
pub mod countryinfo;
pub mod geo_json;

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};

    use countryinfo::CountryInfo;
    use geo_json::GeoJSON;

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

    #[test]
    fn get_timezones() {
        let country_info = CountryInfo::new("India".to_string());
        let timezones_length = country_info.get_tlds().unwrap().len();
        assert_eq!(timezones_length == 1, true);
    }

    #[test]
    fn get_subregion() {
        let country_info = CountryInfo::new("India".to_string());
        let subregion = country_info.get_subregion().unwrap();
        assert_eq!(subregion, "Southern Asia".to_string());
    }

    #[test]
    fn get_region() {
        let country_info = CountryInfo::new("India".to_string());
        let subregion = country_info.get_region().unwrap();
        assert_eq!(subregion, "Asia".to_string());
    }

    #[test]
    fn get_population() {
        let country_info = CountryInfo::new("India".to_string());
        let population = country_info.get_population().unwrap();
        assert_eq!(population, 1263930000);
    }

    #[test]
    fn get_native_name() {
        let country_info = CountryInfo::new("India".to_string());
        let native_name = country_info.get_native_name().unwrap();
        assert_eq!(native_name, "भारत");
    }

    #[test]
    fn get_lat_long() {
        let country_info = CountryInfo::new("India".to_string());
        let lat_long = country_info.get_lat_long().unwrap();
        assert_eq!(lat_long.len() == 2, true);
    }

    #[test]
    fn get_languages() {
        let country_info = CountryInfo::new("India".to_string());
        let languages = country_info.get_languages().unwrap();
        assert_eq!(languages.len() == 2, true);
    }

    #[test]
    fn get_geo_json() {
        let country_info = CountryInfo::new("India".to_string());
        let geo_json = country_info.get_geo_json().unwrap();
        assert_eq!(TypeId::of::<GeoJSON>() == geo_json.type_id(), true);
    }

    #[test]
    fn get_flag() {
        let country_info = CountryInfo::new("India".to_string());
        let flag_link = country_info.get_flag().unwrap();
        assert_eq!(flag_link, "".to_string());
    }

    #[test]
    fn get_demonym() {
        let country_info = CountryInfo::new("India".to_string());
        let demonym = country_info.get_demonym().unwrap();
        assert_eq!(demonym, "Indian".to_string());
    }

    #[test]
    fn get_currencies() {
        let country_info = CountryInfo::new("India".to_string());
        let currencies = country_info.get_currencies().unwrap();
        assert_eq!(currencies.len(), 1);
    }

    #[test]
    fn get_capital_lat_long() {
        let country_info = CountryInfo::new("India".to_string());
        let capital_lat_long = country_info.get_capital_lat_long().unwrap();
        assert_eq!(capital_lat_long.len() == 2, true);
    }

    #[test]
    fn get_capital() {
        let country_info = CountryInfo::new("India".to_string());
        let capital = country_info.get_capital().unwrap();
        assert_eq!(capital, "New Delhi");
    }

    #[test]
    fn get_calling_codes() {
        let country_info = CountryInfo::new("India".to_string());
        let calling_codes = country_info.get_calling_codes().unwrap();
        assert_eq!(calling_codes.len(), 1);
    }

    #[test]
    fn get_borders() {
        let country_info = CountryInfo::new("India".to_string());
        let borders = country_info.get_borders().unwrap();
        assert_eq!(borders.len(), 8);
    }

    #[test]
    fn get_area() {
        let country_info = CountryInfo::new("India".to_string());
        let area = country_info.get_area().unwrap();
        assert_eq!(area, 3287590);
    }

    #[test]
    fn get_iso_codes() {
        let country_info = CountryInfo::new("India".to_string());
        let alpha2_iso_code = country_info.get_iso_codes("2".to_string()).unwrap();
        let alpha3_iso_code = country_info.get_iso_codes("3".to_string()).unwrap();
        assert_eq!(alpha2_iso_code, "IN".to_string());
        assert_eq!(alpha3_iso_code, "IND".to_string());
    }
}
