use std::convert::From;

pub struct Family {
    pub order: String,
    pub family: String,
    pub suborder: String,
    pub superfamily: String,
}

pub type FamilyRecord = (String, String, String, String);

impl From<FamilyRecord> for Family {
    fn from(value: FamilyRecord) -> Self {
        let (order, family, suborder, superfamily) = value;

        Self {
            order,
            family,
            suborder,
            superfamily,
        }
    }
}