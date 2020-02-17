use crate::*;

pub struct Env {
    pub locals: Vec<Local>,
    pub scope_depth: u8
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
    pub fn get(&self, id: usize) -> &Local {
        self.locals.get(id).expect("This is a problem with the compiler itself")
    }
    pub fn open(&mut self) {
        self.scope_depth+=1
    }
    pub fn close(&mut self, chk: &mut Chunk) {
        while self.locals.len()!=0 {
            println!("{:?}", self.locals.last());
            if self.locals.last().expect("This is a problem with the compiler itself").depth == self.scope_depth {
                self.locals.pop();
                println!("pop!");
                chk.add_instr(Instruction::Pop, 5);
            }
            else {break;}
        }
        self.scope_depth-=1;
    }

    pub fn is_redefined(&self, other: &Token) -> bool {
        if self.locals.len() == 0 {return false}
        let mut i = self.locals.len() - 1;
        loop {
            let local = self.locals.get(i).expect("This is a problem with the compiler itself");
            if local.depth < self.scope_depth {break;}

            if local.name.val == other.val {
                return true;
            }

            if i <= 0 {break;}
            i -= 1;
        }
        return false
    }
}

#[derive(Debug)]
pub struct Local {
    pub name: Token,
    pub depth: u8
}