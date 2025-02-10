use std::convert::From;

#[derive(Debug)]
pub struct Country {
    pub name: String,
    pub code: String,
}

pub type CountryRecord = (String, String);

impl From<CountryRecord> for Country {
    fn from(value: CountryRecord) -> Self {
        let (name, code) = value;

        Self {
            name,
            code,
        }
    }
}
