use std::collections::HashMap;

pub struct Query {
    sql: String,
    params: HashMap<String, String>,
}

impl Query {
    pub fn sql(&self) -> &str {
        &self.sql
    }

    pub fn params(&self) -> HashMap<String, String> {
        self.params.clone()
    }
}

pub struct QueryBuilder {
    sql: String,
    params: HashMap<String, String>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            sql: String::new(),
            params: HashMap::new(),
        }
    }

    pub fn query(mut self, sql: &str) -> Self {
        self.sql = sql.to_owned();
        self
    }

    pub fn param(mut self, key: &str, value: String) -> Self {
        self.params.insert(key.to_string(), value);
        self
    }

    pub fn build(self) -> Query {
        Query {
            sql: self.sql,
            params: self.params,
        }
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

