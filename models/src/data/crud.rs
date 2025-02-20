use super::data_error::DataError;

#[async_trait::async_trait]
pub trait Create<Conn> : Sized {
     async fn create(&self, conn: Conn) -> Result<Self, DataError> where Conn: 'async_trait;
}

#[async_trait::async_trait]
pub trait Fetch<Conn> : Sized {
    async fn fetch(conn: Conn) -> Result<Vec<Self>, DataError> where Conn: 'async_trait;
}

#[async_trait::async_trait]
pub trait Count<Conn> : Sized {
    async fn count(&self, conn: Conn) -> Result<i32, DataError> where Conn: 'async_trait;
}

#[async_trait::async_trait]
pub trait Exists<Conn: Send> : Count<Conn > {
    async fn exists(&self, conn: Conn) -> Result<bool, DataError> where Conn: 'async_trait
    {
        match self.count(conn).await {
            Ok(count) => Ok(count > 0),
            Err(err) => Err(err)
        }
    }
}

pub trait List : Sized {
    fn list(&self) -> Result<Vec<Self>, DataError>;
}

