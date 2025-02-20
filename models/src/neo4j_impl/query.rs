use std::collections::HashMap;

pub struct Query {
    query: String,
    params: HashMap<String, String>,
}

impl Query {
    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn params(&self) -> HashMap<String, String> {
        self.params.clone()
    }
}

pub struct QueryBuilder {
    query: String,
    params: HashMap<String, String>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            params: HashMap::new(),
        }
    }

    pub fn query(mut self, query: &str) -> Self {
        self.query = query.to_owned();
        self
    }

    pub fn param(mut self, key: &str, value: &str) -> Self {
        self.params.insert(key.to_owned(), value.to_owned());
        self
    }

    pub fn build(self) -> Query {
        Query {
            query: self.query,
            params: self.params,
        }
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

