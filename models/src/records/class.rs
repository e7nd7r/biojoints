use std::convert::From;

pub struct Class {
    pub phylum: String,
    pub class: String,
    pub subphylum: String,
}

pub type ClassRecord = (String, String, String);

impl From<ClassRecord> for Class {
    fn from(value: ClassRecord) -> Self {
        let (phylum, class, subphylum) = value;

        Self {
            phylum,
            class,
            subphylum
        }
    }
}