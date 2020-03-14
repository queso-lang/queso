use crate::*;
use std::time::{Instant, Duration};

type Stack = Vec<Value>;

pub struct VM {
    frame: CallFrame,
    callstack: Vec<CallFrame>,

    open_upvalues: Vec<MutRc<UpValue>>, 

    stack: Stack,
    debug: bool
}

impl VM {
    pub fn new(chk: Chunk, debug: bool) -> VM {
        let stack = Vec::<Value>::with_capacity(100);

        VM {
            frame: CallFrame::new(chk, 0),
            callstack: vec![],

            open_upvalues: vec![],

            stack,
            debug
        }
    }
        
    fn next_instr(&mut self) -> Option<&Instruction> {
        self.frame.cur_instr += 1;
        self.frame.clsr.func.chk.try_get_instr(self.frame.cur_instr - 1)
    }

    fn pop_stack(&mut self) -> Value {
        self.stack.pop()
            .expect("Failed to pop a value off the stack. This might be a problem with the interpreter itself.")
    }

    fn get_stack(&self, id: u16) -> &Value {
        self.stack.get(id as usize).expect("Couldn't access a value on the stack. This is a problem with the interpreter itself")
    }

    fn get_stack_mut(&mut self, id: u16) -> &mut Value {
        self.stack.get_mut(id as usize).expect("Couldn't access a value on the stack. This is a problem with the interpreter itself")
    }

    fn get_stack_top(&self) -> &Value {
        self.get_stack(self.stack.len() as u16 - 1)
    }

    fn set_stack(&mut self, id: u16, val: Value) {
        self.stack[id as usize] = val;
    }

    fn print_stack(&self) {
        print!("stack ");
        if self.stack.len() == 0 {
            print!("<empty>");
        }
        for val in self.stack.iter() {
            print!("| {} ", val);
        }
        println!();
    }

    fn capture_upvalue(&mut self, upvalueid: &UpValueIndex) -> MutRc<UpValue> {
        if upvalueid.is_local {
            let slot = self.frame.stack_base as u16 + upvalueid.id;

            Rc::new(RefCell::new(UpValue::stack(slot)))
        }
        else {
            Rc::clone(self.frame.clsr.upvalues.get_mut(upvalueid.id as usize).expect(""))
        }
    }

    fn close_upvalues(&mut self) {
        let captured = self.frame.clsr.func.captured.clone();

        for id in captured.iter() {
            let slot = self.frame.stack_base as u16 + id;

            let cur_value = self.get_stack_mut(slot);

            let on_heap = Box::new(cur_value.clone());
            let on_heap_address = Box::into_raw(on_heap);

            for upv in &self.open_upvalues {
                let mut upv = upv.borrow_mut();
                let loc = &mut upv.loc;
                if let UpValueLocation::Stack(l) = loc {
                    if *l == slot {
                        upv.close(on_heap_address);
                    }
                }
            }
        }
    }

    pub fn execute(&mut self) -> Result<(), &'static str> {
        self.run()
    }

    fn run(&mut self) -> Result<(), &'static str> {
        if self.debug {
            println!("\nINSTRUCTIONS:");
        }

        loop {

            if self.debug {
                self.print_stack();
                self.frame.clsr.func.chk.print_instr(self.frame.cur_instr, false);

                println!();
            }

            if let Some(next) = self.next_instr() {
                match next {
                    Instruction::Return => {
                        if let Some(frame) = self.callstack.pop() {
                            let val = self.pop_stack();
                            self.close_upvalues();
                            self.stack.truncate(self.frame.stack_base);
                            self.open_upvalues.truncate(0);
                            self.frame = frame;
                            self.stack.push(val);
                        }
                        else {
                            break;
                        }
                    },
                    Instruction::PushConstant(id) => {
                        let id = *id;
                        let constant: &Value = self.frame.clsr.func.chk.get_const(id);
                        self.stack.push(constant.clone());
                    },
                    Instruction::PushTrue => {
                        self.stack.push(Value::Bool(true));
                    },
                    Instruction::PushFalse => {
                        self.stack.push(Value::Bool(false));
                    },
                    Instruction::PushNull => {
                        self.stack.push(Value::Null);
                    },
                    Instruction::Negate => {
                        let val = self.pop_stack();
                        match val.to_number() {
                            Ok(num) => self.stack.push(Value::Number(-num)),
                            Err(err) => return Err(err)
                        }                       
                    },
                    Instruction::ToNumber => {
                        let val = self.pop_stack();
                        match val.to_number() {
                            Ok(num) => self.stack.push(Value::Number(num)),
                            Err(err) => return Err(err)
                        }                      
                    },
                    Instruction::Not => {
                        let val = self.pop_stack();
                        self.stack.push(Value::Bool(!val.is_truthy()))                     
                    },
                    Instruction::Add => {
                        let b = self.pop_stack();
                        let a = self.pop_stack();

                        match (a, b) {
                            (Value::Number(n1), Value::Number(n2)) => {
                                self.stack.push(Value::Number(n1 + n2));
                            },
                            (Value::String(s1), v @ _) | (v @ _, Value::String(s1)) => {
                                match v.to_string() {
                                    Ok(s2) => self.stack.push(Value::String(s1 + &s2)),
                                    Err(err) => return Err(err)
                                }
                            },
                            _ => {
                                return Err("The addition operator can only be used with numbers and strings");
                            }
                        }
                    },
                    Instruction::Subtract => {
                        let b = self.pop_stack();
                        let a = self.pop_stack();

                        match (a, b) {
                            (Value::Number(n1), Value::Number(n2)) => {
                                self.stack.push(Value::Number(n1 - n2));
                            },
                            _ => {
                                return Err("The subtraction operator can only be used with numbers");
                            }
                        }
                    },
                    Instruction::Multiply => {
                        let b = self.pop_stack();
                        let a = self.pop_stack();

                        match (a, b) {
                            (Value::Number(n1), Value::Number(n2)) => {
                                self.stack.push(Value::Number(n1 * n2));
                            },
                            _ => {
                                return Err("The multiplication operator can only be used with numbers");
                            }
                        }
                    },
                    Instruction::Divide => {
                        let b = self.pop_stack();
                        let a = self.pop_stack();

                        match (a, b) {
                            (Value::Number(n1), Value::Number(n2)) => {
                                if n2 == 0. {
                                    return Err("Cannot divide by 0");
                                }
                                self.stack.push(Value::Number(n1 / n2));
                            },
                            _ => {
                                return Err("The division operator can only be used with numbers");
                            }
                        }
                    },
                    Instruction::Equal => {
                        let b = self.pop_stack();
                        let a = self.pop_stack();

                        self.stack.push(Value::Bool(a.is_equal_to(&b)));
                    },
                    Instruction::NotEqual => {
                        let b = self.pop_stack();
                        let a = self.pop_stack();

                        self.stack.push(Value::Bool(!a.is_equal_to(&b)));
                    },
                    Instruction::GreaterEqual => {
                        let b = self.pop_stack();
                        let a = self.pop_stack();

                        self.stack.push(Value::Bool(a.is_equal_to(&b) || a.is_greater_than(&b)));
                    },
                    Instruction::LessEqual => {
                        let b = self.pop_stack();
                        let a = self.pop_stack();

                        self.stack.push(Value::Bool(a.is_equal_to(&b) || b.is_greater_than(&a)));
                    },
                    Instruction::Greater => {
                        let b = self.pop_stack();
                        let a = self.pop_stack();

                        self.stack.push(Value::Bool(a.is_greater_than(&b)));
                    },
                    Instruction::Less => {
                        let b = self.pop_stack();
                        let a = self.pop_stack();

                        self.stack.push(Value::Bool(b.is_greater_than(&a)));
                    },
                    Instruction::Trace => {
                        let a = self.pop_stack();

                        let val = a.to_string().unwrap_or("".to_string());
                        //add filename
                        let line_no = self.frame.clsr.func.chk.get_line_no(self.frame.cur_instr as u32);
                        println!("[{}] {}", line_no, val);
                        //maybe don't pop at all?

                        self.stack.push(a);
                    },
                    Instruction::Pop => {
                        self.pop_stack();
                    },
                    Instruction::GetLocal(id) => {
                        let id = *id + self.frame.stack_base as u16;
                        let var = self.get_stack(id).clone();
                        self.stack.push(var);
                    },
                    Instruction::GetUpValue(id) => {
                        unsafe {
                            let id = *id;
                            
                            let upvalue = self.frame.clsr.upvalues.get(id as usize).expect("");

                            let value = match upvalue.borrow().loc {
                                UpValueLocation::Stack(id) => self.get_stack(id).clone(),
                                UpValueLocation::Heap(ptr) => (*ptr).clone()
                            };
                            
                            self.stack.push(value);
                        }
                    },
                    Instruction::SetLocal(id) => {
                        let id = *id + self.frame.stack_base as u16;
                        let val = self.get_stack_top().clone();
                        self.set_stack(id, val);
                    },
                    Instruction::SetUpValue(id) => {
                        let id = *id;
                        let set_to = self.get_stack_top().clone();
                        let mut upvalue = self.frame.clsr.upvalues.get_mut(id as usize).expect("").clone();

                        unsafe {
                            match upvalue.borrow_mut().loc {
                                UpValueLocation::Stack(id) => self.set_stack(id, set_to),
                                UpValueLocation::Heap(ptr) => (*ptr) = set_to
                            }
                        };
                    },
                    Instruction::Declare(id) => {
                        let id = *id + self.frame.stack_base as u16;
                        let val = self.pop_stack();
                        self.set_stack(id, val);
                    },
                    Instruction::JumpIfFalsy(jump_count) => {
                        let jump_count = *jump_count as usize;
                        let val = self.get_stack_top();
                        if !val.is_truthy() {
                            self.frame.cur_instr += jump_count;
                        }
                    },
                    Instruction::PopAndJumpIfFalsy(jump_count) => {
                        let jump_count = *jump_count as usize;
                        let val = self.pop_stack();
                        if !val.is_truthy() {
                            self.frame.cur_instr += jump_count;
                        }
                    },
                    Instruction::JumpIfTruthy(jump_count) => {
                        let jump_count = *jump_count as usize;
                        let val = self.get_stack_top();
                        if val.is_truthy() {
                            self.frame.cur_instr += jump_count;
                        }
                    },
                    Instruction::Jump(jump_count) => {
                        let jump_count = *jump_count as usize;
                        self.frame.cur_instr += jump_count;
                    },
                    Instruction::FnCall(arg_count) => {
                        let arg_count = arg_count.clone();

                        let funcpos = self.stack.len() as u16 - 1 - arg_count;

                        let clsr = self.get_stack(funcpos);

                        if let Value::Closure(clsr) = clsr {
                            let mut new_frame = CallFrame::from_closure(clsr.clone(), funcpos as usize);

                            let parent_frame = std::mem::replace(&mut self.frame, new_frame);

                            self.callstack.push(parent_frame);
                        }
                        else {
                            return Err("Tried to call a value which isn't a function");
                        }
                    },
                    Instruction::Reserve(reserve_count) => {
                        let reserve_count = *reserve_count;
                        self.stack.resize(self.stack.len() + reserve_count as usize, Value::Uninitialized);
                    },
                    Instruction::Closure(id, const_id, upvalueids) => {
                        let id = *id;
                        let const_id = *const_id;
                        let upvalueids = upvalueids.clone();
                        
                        let stack_base = self.frame.stack_base.clone() as u16;
                        let id = id + stack_base;
                        
                        let func = self.frame.clsr.func.chk.get_const(const_id).clone();
                        if let Value::Function(func) = func {

                            let mut upvalues = Vec::<MutRc<UpValue>>::new();

                            for upvalueid in upvalueids {
                                let mut upv = self.capture_upvalue(&upvalueid);

                                upvalues.push(upv);

                                let mut r = Rc::clone(upvalues.last_mut().expect(""));

                                self.open_upvalues.push(r)
                            }

                            let closure = Closure {
                                func,
                                upvalues
                            };

                            let val = Value::Closure(closure);
                        
                            self.set_stack(id, val);
                        }
                    },

                    #[allow(unreachable_patterns)]
                    _ => unimplemented!()
                };
            }
            else {break};
        }

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negate() {
        let mut chk = Chunk::new();
        
        chk.add_const(Value::Number(5.));
        chk.add_instr(Instruction::PushConstant(0), 0);
        chk.add_instr(Instruction::Negate, 0);
        chk.add_instr(Instruction::Return, 0);

        let mut vm = VM::new(chk, true);

        assert_eq!(vm.execute(), Ok(()));
    }

    #[test]
    fn test_arithmetic() {
        let mut chk = Chunk::new();
        
        //5 - 5 / 2.5 + 1 * 2 = 5
        //consts[0] = 5, consts[1] = 5, consts[2] = 2.5, consts[3] = 1, consts[4] = 2
        //push 0
        //push 1
        //push 2
        //divide
        //subtract
        //push 3
        //push 4
        //multiply
        //add
        chk.add_const(Value::Number(5.));
        chk.add_instr(Instruction::PushConstant(0), 0);

        chk.add_const(Value::Number(5.));
        chk.add_instr(Instruction::PushConstant(1), 0);

        chk.add_const(Value::Number(2.5));
        chk.add_instr(Instruction::PushConstant(2), 0);

        chk.add_instr(Instruction::Divide, 0);

        chk.add_instr(Instruction::Subtract, 0);

        chk.add_const(Value::Number(1.));
        chk.add_instr(Instruction::PushConstant(3), 0);

        chk.add_const(Value::Number(2.));
        chk.add_instr(Instruction::PushConstant(4), 0);

        chk.add_instr(Instruction::Multiply, 0);

        chk.add_instr(Instruction::Add, 0);

        chk.add_instr(Instruction::Return, 0);

        let mut vm = VM::new(chk, true);

        assert_eq!(vm.execute(), Ok(()));
    }

    #[test]
    fn test_kwexpr() {
        let mut chk = Chunk::new();
        chk.add_line(0);
        
        chk.add_const(Value::Number(5.));
        chk.add_instr(Instruction::PushConstant(0), 0);

        chk.add_instr(Instruction::Trace, 0);

        chk.add_instr(Instruction::Return, 0);

        let mut vm = VM::new(chk, true);

        assert_eq!(vm.execute(), Ok(()));
    }
}
