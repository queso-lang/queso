use crate::*;

#[derive(Clone, PartialEq, Debug)]
pub struct UpValue {
    pub loc: *mut Value,
    // next: *const UpValue
}

impl UpValue {
    pub fn from_ref(val: &mut Value) -> UpValue {
        UpValue {
            loc: val as *mut Value,
            // next: std::ptr::null()
        }
    } 
}