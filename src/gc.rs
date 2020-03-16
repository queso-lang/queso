use crate::*;

pub struct GC {
    grays: Vec<u32>,

    debug: bool
}

impl GC {
    pub fn new(debug: bool) -> GC {
        GC {debug, grays: vec![]}
    }

    fn print_debug_str(&self, s: String) {
        // #[cfg(not(debug_assertions))]
        // return;

        if self.debug {
            println!("{}", s);
        }
    }

    fn print_debug(&self, s: &str) {
        // #[cfg(not(debug_assertions))]
        self.print_debug_str(s.to_string());
    }

    fn mark_obj(&mut self, obj: &mut Obj, id: u32) {
        if obj.is_marked {return}

        obj.is_marked = true;

        self.grays.push(id);

        self.print_debug_str(format!("mark {}: {}", id, obj.obj.display()));
    }

    fn mark_heap_from_value(&mut self, val: &Value, heap: &mut Heap) {
        if let Value::Heap(id) = val {
            let obj = heap.get_mut(*id);
            self.mark_obj(obj, *id);
        }
    }

    fn blacken_obj(&mut self, id: u32, vm: &mut VM) {
        let mut obj = vm.heap.get_mut(id).clone();
        match &obj.obj {
            ObjType::UpValue(upv) => {
                // println!("upv {:?}", upv.loc);
                match upv.loc {
                    UpValueLocation::Stack(id) => self.mark_heap_from_value(&vm.get_stack(id).clone(), &mut vm.heap),
                    UpValueLocation::Heap(id) => {
                        // println!("test {:?}", vm.heap.get(id).obj);
                        let obj = vm.heap.get_mut(id);
                        self.mark_obj(obj, id);
                        self.mark_heap_from_value(&vm.heap.get_val(id).clone(), &mut vm.heap)
                    }
                };
            },
            ObjType::Closure(clsr) => {
                let id = clsr.func;
                self.mark_obj(vm.heap.get_mut(id), id);

                for i in &clsr.upvalues {
                    // println!("upv {}", i);
                    let obj = vm.heap.get_mut(*i);
                    self.mark_obj(obj, *i);

                    // {
                    //     if let ObjType::UpValue(upv) = &mut vm.heap.get_mut(*i).obj {
                    //         Some(upv.loc)
                    //     }
                    //     else {None}
                    // }.unwrap();
                }
            },
            ObjType::Value(val) => {
                self.mark_heap_from_value(&val.clone(), &mut vm.heap)
            }
            _ => {}
        }
    }

    fn trace_refs(&mut self, vm: &mut VM) {
        loop {
            if let Some(id) = self.grays.first() {
                let id = *id;
                self.grays.remove(0);
                self.blacken_obj(id, vm);
            }
            else {
                break;
            }
        }
    }

    fn mark_roots(&mut self, vm: &mut VM) {
        for val in &vm.stack {
            self.mark_heap_from_value(val, &mut vm.heap);
        }

        for frame in &vm.callstack {
            let id = frame.clsr.func;
            self.mark_obj(vm.heap.get_mut(id), id);
        }

        let id = vm.frame.clsr.func;
        self.mark_obj(vm.heap.get_mut(id), id);

        for upv_id in &vm.open_upvalues {
            let upv = vm.heap.get_mut(*upv_id);
            self.mark_obj(upv, *upv_id);
        }
    }

    fn clean(&mut self, heap: &mut Heap) {
        let unsafe_heap_mem = &mut heap.mem as *mut slab::Slab<Obj>;

        let mut to_be_removed = Vec::<usize>::new();

        for (i, obj) in heap.mem.iter_mut() {
            if !obj.is_marked {
                // println!("removing {} {}", i, obj.obj.display());
                // to_be_removed.push(i);
                unsafe {(*unsafe_heap_mem).remove(i);}
            }
        }

        // for i in to_be_removed {
        //     heap.mem.remove(i);
        // }

        for (i, obj) in unsafe {(*unsafe_heap_mem).iter_mut()} {
            if obj.is_marked {
                unsafe {obj.is_marked = false;}
            }
        }
    }

    pub fn collect_garbage(&mut self, vm: &mut VM) {
        // self.print_debug("GC start");

        self.mark_roots(vm);
        // self.print_debug("GC trace");
        self.trace_refs(vm);
        // self.print_debug("GC print");
        for (i, obj) in vm.heap.mem.iter() {
            // print!("{}", " | ".to_string() + &i.to_string());
        }
        self.print_debug("");
        // self.print_debug("GC clean");
        self.clean(&mut vm.heap);

        // self.grays.truncate(0);

        // self.print_debug("GC end");
    }
}
