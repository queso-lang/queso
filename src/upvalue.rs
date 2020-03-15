use crate::*;

#[derive(Clone, PartialEq, Debug)]
pub enum UpValueLocation {
    Stack(u16),
    Heap(u16)
}

#[derive(Clone, PartialEq)]
pub struct ObjUpValue {
    pub loc: UpValueLocation
}

impl ObjUpValue {
    pub fn stack(id: u16) -> ObjUpValue {
        ObjUpValue {
            loc: UpValueLocation::Stack(id)
        }
    }

    pub fn close(&mut self, id: u16) {
        self.loc = UpValueLocation::Heap(id);
    }
}

impl std::fmt::Debug for ObjUpValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.loc);
        Ok(())
    }
}