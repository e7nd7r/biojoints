use std::convert::From;

pub struct Kingdom {
    pub kingdom: String,
    pub superkingdom: String,
}

impl From<&(String, String)> for Kingdom {
    fn from(value: &(String, String)) -> Self {
        let (kingdom, superkingdom) = value;

        Self {
            kingdom: kingdom.clone(),
            superkingdom: superkingdom.clone()
        }
    }
}

