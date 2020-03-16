use crate::*;

pub struct GC {
    grays: Vec<u32>,

    prev_mark: bool,

    debug: bool
}

impl GC {
    pub fn new(debug: bool) -> GC {
        GC {
            debug,
            grays: vec![],
            prev_mark: true
        }
    }

    // Debug helper
    fn print_debug_str(&self, s: String) {
        if self.debug {
            println!("{}", s);
        }
    }

    // Debug helper
    fn print_debug(&self, s: &str) {
        self.print_debug_str(s.to_string());
    }

    // Mark Obj
    fn mark_obj(&mut self, obj: &mut Obj, id: u32) {
        // If true, don't push to grays again
        if obj.is_marked {return}

        obj.is_marked = true;

        self.grays.push(id);
    }

    // Mark value on the heap from Value::Heap referencing it
    fn mark_heap_from_value(&mut self, val: &Value, heap: &mut Heap) {
        if let Value::Heap(id) = val {
            let obj = heap.get_mut(*id);
            self.mark_obj(obj, *id);
        }
    }

    // Trace all values referenced by the roots 
    fn blacken_obj(&mut self, id: u32, vm: &mut VM) {
        let mut obj = vm.heap.get_mut(id).clone();
        match &obj.obj {
            ObjType::UpValue(upv) => {
                match upv.loc {
                    UpValueLocation::Stack(id) => self.mark_heap_from_value(&vm.get_stack(id).clone(), &mut vm.heap),
                    UpValueLocation::Heap(id) => {
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
                    let obj = vm.heap.get_mut(*i);
                    self.mark_obj(obj, *i);
                }
            },
            ObjType::Value(val) => {
                self.mark_heap_from_value(&val.clone(), &mut vm.heap)
            }
            _ => {}
        }
    }

    // Trace all values referenced by the roots
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

    // Marks every root
    fn mark_roots(&mut self, vm: &mut VM) {
        // Mark all values on the heap that stack values are referring to (Value::Heap)
        for val in &vm.stack {
            self.mark_heap_from_value(val, &mut vm.heap);
        }

        // Mark every function from closures on the callstack
        for frame in &vm.callstack {
            let id = frame.clsr.func;
            self.mark_obj(vm.heap.get_mut(id), id);
        }
        // ...& the top of the call stack
        let id = vm.frame.clsr.func;
        self.mark_obj(vm.heap.get_mut(id), id);

        // Mark every value from open upvalues
        for upv_id in &vm.open_upvalues {
            let upv = vm.heap.get_mut(*upv_id);
            self.mark_obj(upv, *upv_id);
        }
    }

    // Iterates through the heap and removes (marks the slot as free) every
    // object with is_marked===false
    fn sweep(&mut self, heap: &mut Heap) {
        // Usage of pointers for mutating while iterating
        let unsafe_heap_mem = &mut heap.mem as *mut slab::Slab<Obj>;

        for (i, obj) in heap.mem.iter_mut() {
            if !obj.is_marked {
                unsafe {(*unsafe_heap_mem).remove(i);}
            }
        }

        // Resets the is_marked after sweeping to its default state for
        // future collections
        // TODO: instead of swapping every mark, make a flag inside the
        // gc that indicates which state is the marked one
        for (i, obj) in heap.mem.iter_mut() {
            obj.is_marked = false;
        }
    }

    // Main function
    // Starts the garbage collection by invoking other helper methods
    // Prints the marked obejcts if debug===true
    pub fn collect_garbage(&mut self, vm: &mut VM) {
        // Mark roots first
        self.mark_roots(vm);
        // Find all refs from these roots
        self.trace_refs(vm);

        if self.debug {
            print!("mark  ");
            for (i, obj) in vm.heap.mem.iter() {
                if obj.is_marked {
                    print!("| {}: {} ", i, obj.obj);
                }
            }
            println!("");
        }

        // Finally sweep
        self.sweep(&mut vm.heap);
    }
}
