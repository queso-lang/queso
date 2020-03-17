use crate::*;
use std::collections::HashSet;
use slab::Slab;

#[derive(PartialEq, Clone, Debug)]
pub enum ObjType {
    Function(Function),
    Closure(Closure),
    Class(Class),

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
            ObjType::UpValue(upv) => format!("upv {}", match upv.loc {
                UpValueLocation::Stack(id) => "s".to_string() + &id.to_string(),
                UpValueLocation::Heap(id) => "h".to_string() + &id.to_string()
            }),
            _ => panic!()
        } 
    }
}

impl std::fmt::Display for ObjType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.display())
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
        const CAPACITY: usize = 1000;
        let mem = Slab::with_capacity(CAPACITY);

        Heap {
            mem
        }
    }

    pub fn alloc_val(&mut self, val: Value) -> HeapIdx {
        self.alloc(ObjType::Value(val))
    }

    pub fn alloc(&mut self, obj: ObjType) -> HeapIdx {
        let obj = Obj {
            obj, is_marked: false
        };
        self.mem.insert(obj) as HeapIdx
    }

    pub fn try_get(&self, id: HeapIdx) -> Option<&Obj> {
        match self.mem.get(id as usize) {
            Some(obj) => Some(obj),
            None => panic!(id)
        }
    }

    pub fn try_get_mut(&mut self, id: HeapIdx) -> Option<&mut Obj> {
        self.mem.get_mut(id as usize)
    }

    pub fn get(&self, id: HeapIdx) -> &Obj {
        self.try_get(id).expect("This is a problem with the interpreter itself")
    }

    pub fn get_mut(&mut self, id: HeapIdx) -> &mut Obj {
        self.try_get_mut(id).expect("This is a problem with the interpreter itself")
    }

    pub fn get_val(&self, id: HeapIdx) -> &Value {
        {
            if let ObjType::Value(val) = &self.get(id).obj {
                Some(val)
            }
            else {None}
        }.unwrap()
    }

    pub fn get_upvalue(&self, id: HeapIdx) -> &UpValue {
        {
            if let ObjType::UpValue(upv) = &self.get(id).obj {
                Some(upv)
            }
            else {None}
        }.unwrap()
    }

    pub fn get_upvalue_mut(&mut self, id: HeapIdx) -> &mut UpValue {
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

    pub fn set(&mut self, id: HeapIdx, to: ObjType) {
        self.mem[id as usize].obj = to
    }

    pub fn set_val(&mut self, id: HeapIdx, to: Value) {
        self.mem[id as usize].obj = ObjType::Value(to)
    }
}