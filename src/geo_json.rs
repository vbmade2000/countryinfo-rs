use serde_derive::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub coordinates: Option<Vec<Vec<Vec<f64>>>>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Properties {
    pub name: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Feature {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    pub properties: Option<Properties>,
    pub geometry: Option<Geometry>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct GeoJSON {
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub features: Option<Vec<Feature>>,
}
