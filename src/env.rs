use crate::*;

pub struct Env {
    locals: Vec<Local>,
    scope_depth: u8
}

impl Env {
    pub fn new() -> Env {
        Env {
            locals: Vec::<Local>::new(),
            scope_depth: 0
        }
    }
    pub fn add(&mut self, name: Token) {
        self.locals.push(Local {name, depth: self.scope_depth as u8});
    }
    pub fn open(&mut self) {
        self.scope_depth+=1
    }
    pub fn close(&mut self, chk: &mut Chunk) {
        self.scope_depth-=1;
        while self.locals.len()!=0 {
            if self.locals.last().expect("This is a problem with the compiler itself").depth == self.scope_depth {
                self.locals.pop();
                println!("pop!");
                chk.add_instr(Instruction::Pop, chk.get_last_line());
            }
            else {break;}
        }
    }

    pub fn is_redefined(&self, other: &Token) -> bool {
        if self.locals.len() == 0 {return false}
        for i in (self.locals.len() - 1)..0{
            let local = self.locals.get(i).expect("This is a problem with the compiler itself");
            if local.depth < self.scope_depth {break;}

            if local.name.val == other.val {
                return true;
            }
        }
        return false
    }
}

pub struct Local {
    pub name: Token,
    pub depth: u8
}