use std::marker::PhantomData;

pub mod mountain;

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
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.parse::<i32>() {
            Ok(id) => Ok(Self::new(id)),
            Err(_) => Err(Self::Error::msg("Invalid mountain id.")),
        }
    }
}
