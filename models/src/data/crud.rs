use std::future::Future;

use super::data_error::DataError;

pub trait Create<Conn> : Sized {
    fn create(&self, conn: Conn) -> impl Future<Output = Result<(), DataError>> + Send;
}

pub trait Fetch<Conn> : Sized {
    fn fetch(conn: Conn) -> Result<Vec<Self>, DataError>;
}

pub trait Count<Conn> : Sized {
    fn count(&self,  conn: Conn) -> impl Future<Output = Result<i32, DataError>> + Send;
}

pub trait Exists<Conn: Send> : Count<Conn> {
    fn exists(&self, conn: Conn) -> impl Future<Output = Result<bool, DataError>> + Send
    where Self : Sync {
    async {
        match self.count(conn).await {
            Ok(count) => Ok(count > 0),
            Err(err) => Err(err)
        }
    } }
}

pub trait List : Sized {
    fn list(&self) -> Result<Vec<Self>, DataError>;
}


