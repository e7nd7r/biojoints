use crate::{
    data::data_error::DataError,
    records::image::Image,
};

use super::{query::QueryBuilder, relational_layer::RelationalOps};

#[derive(Clone)]
pub struct ImageModel<Conn> where Conn: RelationalOps {
    conn: Conn,
}

impl <Conn: RelationalOps> ImageModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self {
            conn,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Image>, DataError> {
        let query = QueryBuilder::new()
            .query("SELECT ImageID, ImageName, ImagePath, ImageDescription FROM _image")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }
}


