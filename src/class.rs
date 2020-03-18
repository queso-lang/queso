use crate::*;
use std::collections::HashMap;

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

#[derive(PartialEq, Clone, Debug)]
pub struct Instance {
    pub class: HeapIdx,
    pub fields: HashMap<Box<str>, Value> 
}

impl Instance {
    pub fn instantiate(from: (HeapIdx, &Class)) -> Instance {
        Instance {
            class: from.0,
            fields: HashMap::new()
        }
    }
}