use crate::conversion::{BorrowFromGValue, FromGValue};
use crate::prelude::{GValue, GremlinResult};

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub(crate) label: String,
    pub(crate) value: Box<GValue>,
    pub(crate) element: Box<GValue>,
}

impl Property {
    pub fn new<T, GT>(label: T, value: GT) -> Property
    where
        T: Into<String>,
        GT: Into<GValue>,
    {
        Property {
            label: label.into(),
            value: Box::new(value.into()),
            element: Box::new(GValue::Null), // TODO
        }
    }

    pub fn value(&self) -> &GValue {
        &self.value
    }

    pub fn take<T>(self) -> GremlinResult<T>
    where
        T: FromGValue,
    {
        T::from_gvalue(*self.value)
    }

    pub fn get<'a, T>(&'a self) -> GremlinResult<&'a T>
    where
        T: BorrowFromGValue,
    {
        T::from_gvalue(&self.value)
    }

    pub fn label(&self) -> &String {
        &self.label
    }
}
