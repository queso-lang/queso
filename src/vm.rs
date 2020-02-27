use crate::*;

type Stack = Vec<Value>;

pub struct VM {
    frame: CallFrame,
    callstack: Vec<CallFrame>,

    stack: Stack,

    debug: bool
}

impl VM {
    pub fn new(chk: Chunk, debug: bool) -> VM {
        VM {
            frame: CallFrame::new(chk, 0),
            callstack: Vec::<CallFrame>::new(),
            stack: Stack::new(),
            debug
        }
    }
        
    fn next_instr(&mut self) -> Option<&Instruction> {
        self.frame.cur_instr += 1;
        self.frame.func.chk.try_get_instr(self.frame.cur_instr - 1)
    }

    fn pop_stack(&mut self) -> Value {
        self.stack.pop()
            .expect("Failed to pop a value off the stack. This might be a problem with the interpreter itself.")
    }

    fn get_stack(&self, id: u16) -> &Value {
        self.stack.get(id as usize).expect("Couldn't access a value on the stack. This is a problem with the interpreter itself")
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
            print!("| {:?} ", val);
        }
        println!();
    }

    pub fn execute(&mut self) -> Result<(), &'static str> {
        self.frame.cur_instr = 0;
        self.run()
    }

    fn run(&mut self) -> Result<(), &'static str> {
        if self.debug {
            println!("\nINSTRUCTIONS:");
        }

        loop {

            if self.debug {
                self.print_stack();
                self.frame.func.chk.print_instr(self.frame.cur_instr, false);

                println!();
            }

            if let Some(next) = self.next_instr() {
                match next {
                    Instruction::Return => {
                        if let Some(frame) = self.callstack.pop() {
                            self.frame = frame;
                        }
                        else {
                            break;
                        }
                    },
                    Instruction::PushConstant(id) => {
                        let id = *id;
                        let constant: &Value = self.frame.func.chk.get_const(id);
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
                        let line_no = self.frame.func.chk.get_line_no(self.frame.cur_instr as u32);
                        println!("[{}] {}", line_no, val);
                        //maybe don't pop at all?

                        self.stack.push(a);
                    },
                    Instruction::Pop => {
                        self.pop_stack();
                    },
                    Instruction::PushVariable(id) => {
                        let id = *id + self.frame.stack_base as u16;
                        let var = self.get_stack(id).clone();
                        self.stack.push(var);
                    },
                    Instruction::Assign(id) => {
                        let id = *id;
                        let val = self.get_stack_top().clone();
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
                    Instruction::EndBlock(pop_count) => {
                        let pop_count = pop_count.clone();
                        let hangon = self.pop_stack();
                        for _ in 0..pop_count {
                            self.pop_stack();
                        }
                        self.stack.push(hangon);
                    }

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