use crate::{TokenType, Token, TokenPos};
use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
     "let" => TokenType::Let, "mut" => TokenType::Mut, "class" => TokenType::Class, "fn" => TokenType::Fn,
     "if" => TokenType::If, "else" => TokenType::Else, "for" => TokenType::For, "while" => TokenType::While,
     "break" => TokenType::Break, "continue" => TokenType::Continue,
     "trace" => TokenType::Trace, "return" => TokenType::Return, "in" => TokenType::In, "catch" => TokenType::Catch,
     "this" => TokenType::This, "prv" => TokenType::Prv, "static" => TokenType::Static, "new" => TokenType::New, "base" => TokenType::Base,
     "emit" => TokenType::Emit, "on" => TokenType::On,
     "true" => TokenType::True, "false" => TokenType::False
};

#[derive(Clone)]
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

//implementation
impl Lexer {
    pub fn lex_next(&mut self) -> Token{
        self.from_col = self.to_col;
        
        if self.is_eof() {
            return self.new_token(TokenType::EOF);
        }

        let c = self.next();
        match c {
            '(' => self.new_token(TokenType::LeftParen),
            ')' => self.new_token(TokenType::RightParen),
            '[' => self.new_token(TokenType::LeftBracket),
            ']' => self.new_token(TokenType::RightBracket),
            '{' => self.new_token(TokenType::LeftBrace),
            '}' => self.new_token(TokenType::RightBrace),

            ';' => self.new_token(TokenType::Semi),
            '~' => self.new_token(TokenType::Null),
            '+' => self.new_token(TokenType::Plus),
            ',' => self.new_token(TokenType::Comma),
            '.' => self.new_token(TokenType::Dot),
            '#' => self.new_token(TokenType::Hash),
            '%' => self.new_token(TokenType::Percent),

            '*' => {
                let t = if self.match_token('*') {TokenType::StarStar} else {TokenType::Star};
                self.new_token(t)
            },
            '-' => {
                let t = if self.match_token('>') {TokenType::Arrow} else {TokenType::Minus};
                self.new_token(t)
            },
            '@' => {
                let t = if self.match_token(':') {TokenType::AtColon} else {TokenType::At};
                self.new_token(t)
            },
            ':' => {
                let t = if self.match_token(':') {TokenType::ColonColon} else {TokenType::Colon};
                self.new_token(t)
            },
            '!' => {
                let t = if self.match_token('=') {TokenType::BangEqual} else {TokenType::Bang};
                self.new_token(t)
            },
            '=' => {
                let t = if self.match_token('=') {TokenType::EqualEqual} else {TokenType::Equal};
                self.new_token(t)
            },
            '<' => {
                let t = if self.match_token('=') {TokenType::LessEqual} else {TokenType::Less};
                self.new_token(t)
            },
            '>' => {
                let t = if self.match_token('=') {TokenType::GreaterEqual} else {TokenType::Greater};
                self.new_token(t)
            },
            '|' => {
                let t = 
                    if self.match_token('|') {TokenType::Or}
                    else if self.match_token('>') {TokenType::Pipe}
                    else {TokenType::BitOr};
                self.new_token(t)
            },
            '&' => {
                let t = if self.match_token('&') {TokenType::BitAnd} else {TokenType::And};
                self.new_token(t)
            },
            '/' => {
                if self.match_token('/') {
                    while self.peek(0) != & '\n' && !self.is_eof() {
                        self.next();
                    }
                    self.lex_next()
                }
                else if self.match_token('*') {
                    while self.peek(0) != & '*' && self.peek(1) != & '/' && self.is_eof() {
                        self.next();
                    }
                    //improve this
                    if !self.is_eof() {self.next();}
                    if !self.is_eof() {self.next();}

                    self.lex_next()
                }
                else {self.new_token(TokenType::Slash)}
            },
            ' ' | '\r' | '\t' => { self.lex_next() },
            '\n' => { self.line += 1; self.lex_next() },
            '"' => { self.make_string('"') },
            '\'' => { self.make_string('\'') }
            _ => {
                if c.is_ascii_digit() {
                    return self.make_number()
                }
                else if c.is_ascii_alphabetic() || c == & '_' {
                    return self.make_identifier()
                }
                self.new_token(TokenType::Invalid)
            }
        }
    }

    fn make_string(&mut self, quote_type: char) -> Token {
        while self.peek(0) != &quote_type && !self.is_eof() {
            self.next();
        }
        if self.is_eof() {
            //report error
            panic!();
        }
        self.next();

        self.new_token(TokenType::String)
    }
    fn make_number(&mut self) -> Token {
        while self.peek(0).is_ascii_digit() {
            self.next();
        }
        if self.peek(0) == & '.' && self.peek(1).is_ascii_digit() {
            self.next();

            while self.peek(0).is_ascii_digit() {self.next();}
        }

        self.new_token(TokenType::Number)
    }
    fn make_identifier(&mut self) -> Token {
        while self.peek(0).is_ascii_alphabetic() || self.peek(0) == & '_' {
            self.next();
        }

        let val = &self.src[(self.from_col as usize)..(self.to_col as usize)];
        let val: String = val.iter().collect();

        if let Some(t) = KEYWORDS.get(&*val) {
            let t: TokenType = t.to_owned();
            self.new_token(t)
        }
        else { self.new_token(TokenType::Identifier) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strings() {
        let mut lexer = Lexer::new(String::from("\"foo\" \'bar\' \"b\'uz\'z\" \'y\"ee\"t\'"));
        
        assert_eq!(lexer.lex_next().val, "\"foo\"");
        assert_eq!(lexer.lex_next().val, "\'bar\'");
        assert_eq!(lexer.lex_next().val, "\"b\'uz\'z\"");
        assert_eq!(lexer.lex_next().val, "\'y\"ee\"t\'");
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new(String::from("abc let def emit"));
        
        assert_eq!(lexer.lex_next().t, TokenType::Identifier);
        assert_eq!(lexer.lex_next().t, TokenType::Let);
        assert_eq!(lexer.lex_next().val, "def");
        assert_eq!(lexer.lex_next().val, "emit");
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new(String::from("123 4.56"));
        
        assert_eq!(lexer.lex_next().t, TokenType::Number);
        assert_eq!(lexer.lex_next().val, "4.56");
    }

    #[test]
    fn test_invalid() {
        let mut lexer = Lexer::new(String::from("^ ` Ł"));
        
        assert_eq!(lexer.lex_next().t, TokenType::Invalid);
        assert_eq!(lexer.lex_next().t, TokenType::Invalid);
        assert_eq!(lexer.lex_next().t, TokenType::Invalid);
    }

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new(String::from("//test"));
        
        assert_eq!(lexer.lex_next().t, TokenType::EOF);
    }

    #[test]
    fn test_newline() {
        let mut lexer = Lexer::new(String::from("abc\ndef"));
        
        assert_eq!(lexer.lex_next().val, "abc");
        assert_eq!(lexer.lex_next().val, "def");
    }
}