use countryinfo_rs::countryinfo::CountryInfo;

pub fn main() {
    // Instantiate the struct
    let country_info = CountryInfo::new("India".to_string());

    println!(
        "Capital                    => {:?}",
        &country_info.get_capital().unwrap()
    );
    println!(
        "Alternate Spellings        => {:?}",
        &country_info.get_alt_spellings().unwrap()
    );
    println!(
        "Wikipedia Link             => {:?}",
        &country_info.get_wiki().unwrap()
    );
    println!(
        "Translations               => {:?}",
        &country_info.get_translations().unwrap()
    );
    println!(
        "TLDs                       => {:?}",
        &country_info.get_tlds().unwrap()
    );
    println!(
        "Timezones                  => {:?}",
        &country_info.get_timezones().unwrap()
    );
    println!(
        "Subregion                  => {:?}",
        &country_info.get_subregion().unwrap()
    );
    println!(
        "Region                     => {:?}",
        &country_info.get_region().unwrap()
    );
    println!(
        "Population                 => {:?}",
        &country_info.get_population().unwrap()
    );
    println!(
        "Native Name                => {:?}",
        &country_info.get_native_name().unwrap()
    );
    println!(
        "Latitude - Longitude       => {:?}",
        &country_info.get_lat_long().unwrap()
    );
    println!(
        "Languages                  => {:?}",
        &country_info.get_languages().unwrap()
    );
    println!(
        "Flag                       => {:?}",
        &country_info.get_flag().unwrap()
    );
    println!(
        "Demonym                    => {:?}",
        &country_info.get_demonym().unwrap()
    );
    println!(
        "Currencies                 => {:?}",
        &country_info.get_currencies().unwrap()
    );
    println!(
        "Capital Latitude Longitude => {:?}",
        &country_info.get_capital_lat_long().unwrap()
    );
    println!(
        "Calling Codes              => {:?}",
        &country_info.get_calling_codes().unwrap()
    );
    println!(
        "Borders                    => {:?}",
        &country_info.get_borders().unwrap()
    );
    println!(
        "Area                       => {:?}",
        &country_info.get_area().unwrap()
    );
    println!(
        "ISO Code - alpha2          => {:?}",
        &country_info.get_iso_codes("2".to_string()).unwrap()
    );
    println!(
        "ISO Code - alpha3          => {:?}",
        &country_info.get_iso_codes("3".to_string()).unwrap()
    );
    println!(
        "Geo JSON                   => {:?}",
        &country_info.get_geo_json().unwrap()
    );
}
