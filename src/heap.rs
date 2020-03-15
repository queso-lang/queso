use crate::*;
use std::collections::HashSet;
use slab::Slab;

#[derive(PartialEq, Clone, Debug)]
pub enum ObjType {
    Function(Function),
    Closure(Closure),
    Value(Value),
    UpValue(UpValue)
}

impl ObjType {
    pub fn is_truthy(&self) -> bool {
        match self {
            ObjType::Value(val) => val.is_truthy(),
            _ => true
        }
    }

    pub fn to_string(&self) -> Result<String, &'static str> {
        match self {
            ObjType::Value(val) => val.to_string(),
            _ => Err("Can't convert this to a string")
        }
    }

    pub fn display(&self) -> String {
        match self {
            ObjType::Function(func) => format!("fn {}", func.name),
            ObjType::Closure(clsr) => format!("clsr {}", clsr.func),
            ObjType::Value(val) => val.display(),
            _ => panic!()
        } 
    }
}

impl std::fmt::Display for ObjType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ObjType::Function(func) => write!(f, "fn {}", func.name),
            ObjType::Closure(clsr) => write!(f, "clsr {}", clsr.func),
            ObjType::Value(val) => std::fmt::Display::fmt(&val, f),
            _ => panic!()
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Obj {
    pub obj: ObjType,
    pub is_marked: bool
}

#[derive(Clone, Debug)]
pub struct Heap {
    pub mem: Slab<Obj>
}

impl Heap {
    pub fn new() -> Heap {
        const CAPACITY: usize = 100;
        let mem = Slab::with_capacity(CAPACITY);

        Heap {
            mem
        }
    }

    pub fn alloc_val(&mut self, val: Value) -> u16 {
        self.alloc(ObjType::Value(val))
    }

    pub fn alloc(&mut self, obj: ObjType) -> u16 {
        let obj = Obj {
            obj, is_marked: false
        };
        self.mem.insert(obj);

        self.mem.len() as u16 - 1
    }

    pub fn try_get(&self, id: u16) -> Option<&Obj> {
        self.mem.get(id as usize)
    }

    pub fn try_get_mut(&mut self, id: u16) -> Option<&mut Obj> {
        self.mem.get_mut(id as usize)
    }

    pub fn get(&self, id: u16) -> &Obj {
        self.try_get(id).expect("This is a problem with the interpreter itself")
    }

    pub fn get_mut(&mut self, id: u16) -> &mut Obj {
        self.try_get_mut(id).expect("This is a problem with the interpreter itself")
    }

    pub fn get_val(&self, id: u16) -> &Value {
        {
            if let ObjType::Value(val) = &self.get(id).obj {
                Some(val)
            }
            else {None}
        }.unwrap()
    }

    pub fn get_upvalue(&self, id: u16) -> &UpValue {
        {
            if let ObjType::UpValue(upv) = &self.get(id).obj {
                Some(upv)
            }
            else {None}
        }.unwrap()
    }

    pub fn get_upvalue_mut(&mut self, id: u16) -> &mut UpValue {
        {
            if let ObjType::UpValue(upv) = &mut self.get_mut(id).obj {
                Some(upv)
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