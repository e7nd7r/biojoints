use crate::data::data_error::DataError;

pub trait FromData<T> : Sized {
    fn from(data: T) -> Result<Self, DataError>;
}

