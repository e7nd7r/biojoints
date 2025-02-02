use std::convert::From;

pub struct Phylum {
    pub kingdom: String,
    pub phylum: String,
    pub subkingdom: String,
}

pub type PhylumRecord = (String, String, String);

impl From<&PhylumRecord> for Phylum {
    fn from(value: &PhylumRecord) -> Self {
        let (kingdom, phylum, subkingdom) = value;
        
        Self {
            kingdom: kingdom.clone(),
            phylum: phylum.clone(),
            subkingdom: subkingdom.clone(),
        }
    }
}