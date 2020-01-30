#[derive(Debug)]
pub enum Instruction {
    Constant {id: u16},
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Null
}

struct LineRL {pub line: u32, pub repeat: u16}
type LineVec = Vec<LineRL>;

pub struct Chunk {
    instrs: Vec<Instruction>,
    consts: Vec<Value>,
    lines: LineVec
}

impl Chunk {
    fn add_line(&mut self, line: u32) {
        match self.lines.last_mut() {
            Some(last_line) => if last_line.line == line { last_line.repeat += 1 },
            None => self.lines.push(LineRL {line, repeat: 1})
        }
    }

    // api
    pub fn new() -> Chunk {
        Chunk {
            instrs: Vec::<Instruction>::new(),
            consts: Vec::<Value>::new(),
            lines: LineVec::new()
        }
    }
    pub fn add_const(&mut self, val: Value) -> u16 {
        self.consts.push(val);
        (self.consts.len() - 1) as u16
    }
    pub fn get_const(&self, const_id: u16) -> &Value {
        self.consts.get(const_id as usize)
            .expect("The VM failed to access a constant. This might be a problem with the interpreter itself.")
    }
    pub fn add_instr(&mut self, instr: Instruction, line_no: u32) {
        self.instrs.push(instr);
        self.add_line(line_no);
    }
    pub fn get_instr(&self, instr_id: usize) -> &Instruction {
        self.instrs.get(instr_id)
            .expect("The VM failed to access an instruction. This might be a problem with the interpreter itself.")
    }
    pub fn get_line_no(&self, instr_id: u32) -> u32 {
        let mut cur: u32 = 0;
        for l in self.lines.iter() {
            cur += l.repeat as u32;
            if cur > instr_id {
                return l.line;
            }
        }
        
        panic!("The VM failed to access a line. This might be a problem with the interpreter itself.")
    }

    // pretty print
    pub fn print(&self, name: &'static str) {
        println!("== {} ==", name);
        for i in 0..self.instrs.len() {
            self.print_instr(i, true);
        }
    }
    pub fn print_instr(&self, instr_id: usize, hide_repeating_lines: bool) {
        print!("{:04} {:>4} ", instr_id,
            if instr_id >= 1
            && self.get_line_no(instr_id as u32) == self.get_line_no((instr_id-1) as u32)
            && hide_repeating_lines {
                "".to_string()
            } else {
                self.get_line_no(instr_id as u32).to_string()
            }
        );
        self.print_instr_info(&self.get_instr(instr_id));
    }
    pub fn print_instr_info(&self, instr: &Instruction) {
        match instr {
            Instruction::Constant {id} => println!("{:?}, value: {:?}", instr, self.get_const(*id)),
            _ => println!("{:?}", instr)
        };
    }
}