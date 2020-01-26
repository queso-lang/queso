use crate::{TokenType, Token, TokenPos};

pub struct Lexer {
    src: Vec<char>,

    from_col: u32,
    to_col: u32,
    line: u32
}

//utils
impl Lexer {
    pub fn new(src: String) -> Lexer {
        Lexer {
            src: src.chars().collect(),

            from_col: 0,
            to_col: 0,
            line: 1
        }
    }

    fn next(&mut self) -> &char {
        self.to_col += 1;
        &self.src[(self.to_col - 1) as usize]
    }
    fn peek(&self, ahead: u8) -> &char {
        if self.to_col + ahead as u32 >= self.src.len() as u32 { return &'\0' }
        &self.src[(self.to_col + ahead as u32) as usize]
    }
    fn new_token(&self, t: TokenType) -> Token {
        let val = &self.src[(self.from_col as usize)..(self.to_col as usize)];
        let val: String = val.iter().collect();
        let pos = TokenPos::new(self.from_col, self.to_col, self.line);
        Token {t, val, pos}
    }
    fn match_token(&mut self, expect: char) -> bool {
        if self.is_eof() || self.src[self.to_col as usize] != expect { return false; }
        
        self.to_col += 1;
        true
    }
    fn is_eof(&self) -> bool { self.to_col >= self.src.len() as u32 }
}