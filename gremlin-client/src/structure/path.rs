use crate::structure::GValue;

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    pub(crate) labels: Box<GValue>,
    pub(crate) objects: Box<GValue>,
}

impl Path {
    pub fn new(labels: GValue, objects: GValue) -> Self {
        Path {
            labels: Box::new(labels),
            objects: Box::new(objects),
        }
    }
}
