use std::collections::HashMap;

use crate::data::query_builder::QueryBuilder;

pub struct FetchKingdomQueryBuilder {
}

impl FetchKingdomQueryBuilder {
    pub fn new() -> Self {
        FetchKingdomQueryBuilder {}
    }
}

impl Default for FetchKingdomQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl QueryBuilder for FetchKingdomQueryBuilder {
    fn build(&self) -> (String, HashMap<String, String>) {
        let params = HashMap::<String, String>::new();

        ("MATCH (n:Kingdom) RETURN n".to_owned(), params)
    }
}

