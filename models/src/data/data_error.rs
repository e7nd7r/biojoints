
#[derive(Debug)]
pub enum DataError {
    NotInsertedEntity(String),
    QueryError(String),
    AlreadyExist(String),
    UnexpectedCode(String),
}

