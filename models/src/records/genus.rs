use std::convert::From;

pub struct Genus {
    pub family: String,
    pub genus: String,
    pub subfamily: String,
    pub tribe: String
}

pub type GenusRecord = (String, String, String, Option<String>);

impl From<GenusRecord> for Genus {
    fn from(value: GenusRecord) -> Self {
        let (family, genus, subfamily, tribe) = value;

        Self {
            family,
            genus,
            subfamily,
            tribe: tribe.unwrap_or("".to_owned()),
        }
    }
}