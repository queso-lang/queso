use crate::*;

#[derive(Clone, PartialEq, Debug)]
pub enum UpValueLocation {
    Stack(u16),
    Heap(u16)
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

    pub fn close(&mut self, id: u16) {
        self.loc = UpValueLocation::Heap(id);
    }
}

impl std::fmt::Debug for UpValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.loc);
        Ok(())
    }
}