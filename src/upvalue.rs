use crate::*;

#[derive(Clone, PartialEq, Debug)]
pub enum UpValueLocation {
    Stack(u16),
    Heap(*mut Value)
}

#[derive(Clone, PartialEq)]
pub struct UpValue {
    pub loc: UpValueLocation
}

impl UpValue {
    pub fn stack(id: u16) -> UpValue {
        UpValue {
            loc: UpValueLocation::Stack(id)
        }
    }

    pub fn close(&mut self, at: *mut Value ) {
        self.loc = UpValueLocation::Heap(at);
    }
}

impl std::fmt::Debug for UpValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.loc);
        if let UpValueLocation::Heap(ptr) = self.loc {
            unsafe {
                write!(f, " val {}", *ptr);
            }
        }
        Ok(())
    }
}