use std::collections::HashMap;

pub trait QueryBuilder : Sync {
    fn build(&self) -> (String, HashMap<String, String>);
}

