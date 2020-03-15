use crate::*;

#[derive(PartialEq, Clone, Debug)]
pub enum ObjType {
    Function(Function),
    Closure(Closure),
    Value(Value)
}

#[derive(PartialEq, Clone, Debug)]
pub struct Obj {
    pub obj: ObjType,
    pub is_marked: bool
}

#[derive(PartialEq, Clone, Debug)]
pub struct Heap {
    mem: Vec<Obj>
}

impl Heap {
    pub fn new() -> Heap {
        Heap {mem: vec![]}
    }

    pub fn alloc_val(&mut self, val: Value) -> u16 {
        self.alloc(ObjType::Value(val))
    }

    pub fn alloc(&mut self, obj: ObjType) -> u16 {
        self.mem.push(Obj{obj, is_marked: false});
        self.mem.len() as u16 - 1
    }

    pub fn try_get(&self, id: u16) -> Option<&Obj> {
        self.mem.get(id as usize)
    }

    pub fn get(&self, id: u16) -> &Obj {
        self.try_get(id).expect("This is a problem with the interpreter itself")
    }

    pub fn get_val(&self, id: u16) -> &Value {
        {
            if let ObjType::Value(val) = &self.get(id).obj {
                Some(val)
            }
            else {None}
        }.unwrap()
    }

    pub fn get_clsr_fn(&self, clsr: &Closure) -> &Function {
        {
            if let ObjType::Function(func) = &self.get(clsr.func).obj {
                Some(func)
            }
            else {None}
        }.unwrap()
    }

    pub fn set(&mut self, id: u16, to: ObjType) {
        self.mem[id as usize].obj = to
    }

    pub fn set_val(&mut self, id: u16, to: Value) {
        self.mem[id as usize].obj = ObjType::Value(to)
    }
}