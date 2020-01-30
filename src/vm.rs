use crate::*;

type Stack = Vec<Value>;

pub struct VM {
    chk: Chunk,
    cur_instr: usize,
    stack: Stack,

    pub hadError: bool
}

impl VM {
    pub fn new() -> VM {
        VM {
            chk: Chunk::new(),
            cur_instr: 0,
            stack: Stack::new(),
            hadError: false
        }
    }
    pub fn execute(&mut self, chk: Chunk) {
        self.chk = chk;
        self.cur_instr = 0;
        self.run()
    }

    fn run(&mut self) {
        println!("===========\nrun:");

        loop {
            println!();
            self.chk.print_instr(self.cur_instr, false);

            self.print_stack();

            if let Some(next) = self.next_instr() {
                match next {
                    Instruction::Return => {
                        println!("Return: {:?}", self.pop_stack());

                        break;
                    },
                    Instruction::Constant(id) => {
                        let id = *id;
                        let constant: &Value = self.chk.get_const(id);
                        println!("Push constant to stack: {:?}", constant);
                        self.stack.push(constant.clone());
                    },
                    Instruction::Negate => {
                        let val = self.pop_stack();
                        match val {
                            Value::Number(num) => {
                                println!("Negate top of stack: {:?}", val);
                                self.stack.push(Value::Number(-num));
                            },
                            _ => {
                                self.hadError = true;
                                return;
                            }
                        }                        
                    },

                    #[allow(unreachable_patterns)]
                    _ => unimplemented!()
                };
            }
            else {break};
        }
    }

    fn next_instr(&mut self) -> Option<&Instruction> {
        self.cur_instr += 1;
        self.chk.try_get_instr(self.cur_instr - 1)
    }

    fn pop_stack(&mut self) -> Value {
        self.stack.pop()
            .expect("Failed to pop a value of the stack. This might be a problem with the interpreter itself.")
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negate() {
        let mut chk = Chunk::new();
        
        chk.add_const(Value::Number(5.));
        chk.add_instr(Instruction::Constant(0), 0);
        chk.add_instr(Instruction::Negate, 0);
        chk.add_instr(Instruction::Return, 0);

        let mut vm = VM::new();
        vm.execute(chk);

        assert!(!vm.hadError);
    }
}