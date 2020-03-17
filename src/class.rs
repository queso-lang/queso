use crate::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Class {
    pub name: Box<str>
}

impl Class {
    pub fn new(name: String) -> Class {
        Class {
            name: name.into_boxed_str()
        }
    }
}