use std::convert::From;

pub struct Image {
    pub file_name: String,
    pub notes: String,
    pub specie_code: String,
    pub author_id: i32,
    pub valid_specie_code: String,
}

pub type ImageRecord = (
    String,
    String,
    String,
    i32,
    String,
);

impl From<ImageRecord> for Image {
    fn from(value: ImageRecord) -> Self {
        let (
            file_name,
            notes,
            specie_code,
            author_id,
            valid_specie_code,
        ) = value;

        Self {
            file_name,
            notes,
            specie_code,
            author_id,
            valid_specie_code,
        }
    }
}