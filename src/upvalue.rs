use crate::*;

pub type StackIdx = u16;
pub type HeapIdx = u32;

#[derive(Clone, PartialEq, Debug)]
pub enum UpValueLocation {
    Stack(StackIdx),
    Heap(HeapIdx)
}

#[derive(Clone, PartialEq)]
pub struct UpValue {
    pub loc: UpValueLocation
}

impl UpValue {
    pub fn stack(id: StackIdx) -> UpValue {
        UpValue {
            loc: UpValueLocation::Stack(id)
        }
    }

    pub fn close(&mut self, id: HeapIdx) {
        self.loc = UpValueLocation::Heap(id);
    }
}

impl std::fmt::Debug for UpValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.loc);
        Ok(())
    }
}