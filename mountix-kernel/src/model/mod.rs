use std::marker::PhantomData;

pub mod mountain;
pub mod surrounding_mountain;

#[derive(Debug)]
pub struct Id<T> {
    pub value: i32,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

impl<T> From<i32> for Id<T> {
    fn from(value: i32) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

impl<T> TryFrom<String> for Id<T> {
    type Error = ErrorCode;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.parse::<i32>() {
            Ok(id) => Ok(Self::new(id)),
            Err(_) => Err(ErrorCode::InvalidId),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorCode {
    InvalidId,
    InvalidQueryParam,
    ServerError,
}
