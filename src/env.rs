use crate::*;

pub struct Env {
    pub locals: Vec<LocalIndex>,
    pub upvalues: Vec<UpValueIndex>,
    pub captured: Vec<u16>,
    pub scope_depth: u8
}

impl Env {
    pub fn new() -> Env {
        Env {
            locals: Vec::<LocalIndex>::new(),
            upvalues: Vec::<UpValueIndex>::new(),
            captured: Vec::<u16>::new(),
            scope_depth: 0,
        }
    }
    pub fn add_local(&mut self, name: Token) -> u16 {
        self.locals.push(LocalIndex {name, depth: self.scope_depth as u8});
        self.locals.len() as u16 - 1
    }
    pub fn add_upvalue(&mut self, upvalue: UpValueIndex) -> u16 {
        for (i, upv) in self.upvalues.iter().enumerate() {
            if upv.id == upvalue.id && upv.is_local == upvalue.is_local {
                return i as u16;
            }
        }
        self.upvalues.push(upvalue);
        self.upvalues.len() as u16 - 1
    }
    pub fn get(&self, id: usize) -> &LocalIndex {
        self.locals.get(id).expect("This is a problem with the compiler itself")
    }
    pub fn open(&mut self) {
        self.scope_depth+=1
    }
    pub fn close(&mut self) {
        self.scope_depth-=1;
    }

    pub fn is_redefined(&self, other: &Token) -> bool {
        if self.locals.len() == 0 {return false}
        let mut i = self.locals.len() - 1;
        loop {
            let local = self.locals.get(i).expect("This is a problem with the compiler itself");
            if local.depth < self.scope_depth {break;}

            if local.name.val == other.val {
                return true;
            }

            if i <= 0 {break;}
            i -= 1;
        }
        return false
    }

    pub fn capture(&mut self, id: u16) {
        self.captured.push(id);
    }
}

#[derive(Debug)]
pub struct LocalIndex {
    pub name: Token,
    pub depth: u8
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpValueIndex {
    pub id: u16,
    pub is_local: bool
}
