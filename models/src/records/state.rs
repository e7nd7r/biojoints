use std::{collections::HashMap, convert::From};

use crate::data::data_error::DataError;

pub struct State {
    pub country_code: String,
    pub name: String,
    pub code: String,
}

impl State {
    pub fn nonsdt_to_code(code: &str) ->  Result<String, DataError> {
        let map = HashMap::from([
            ("ags.", "agu"),
            ("b.c.", "bcn"),
            ("b.c.s.", "bcs"),
            ("camp.", "cam"),
            ("chis.", "chp"),
            ("chih.", "chh"),
            ("coah.", "coa"),
            ("col.", "col"),
            ("cdmx", "cmx"),
            ("d.f.", "cmx"),
            ("dgo.", "dur"),
            ("gto.", "gua"),
            ("gro.", "gro"),
            ("hgo.", "hid"),
            ("jal.", "jal"),
            ("edomex.", "mex"),
            ("méx.", "mex"),
            ("mich.", "mic"),
            ("mor.", "mor"),
            ("nay.", "nay"),
            ("n.l.", "nle"),
            ("oax.", "oax"),
            ("pue.", "pue"),
            ("qro.", "que"),
            ("q. roo.", "roo"),
            ("q.r.", "roo"),
            ("s.l.p.", "slp"),
            ("sin.", "sin"),
            ("son.", "son"),
            ("tab.", "tab"),
            ("tamps.", "tam"),
            ("tlax.", "tla"),
            ("ver.", "ver"),
            ("yuc.", "yuc"),
            ("zac.", "zac"),
        ]);

        map
        .get(code)
        .map(|x| x.to_string())
        .ok_or(DataError::UnexpectedCode(format!("Invalid code. | {}", code)))
    }
}

pub type StateRecord = (String, String, String);

impl From<StateRecord> for State {
    fn from(value: StateRecord) -> Self {
        let (country_code, name, code) = value;

        Self {
            country_code,
            name,
            code,
        }
    }
}