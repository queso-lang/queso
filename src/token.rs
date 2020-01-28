use crate::Lexer;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    LeftParen, RightParen, LeftBracket, RightBracket, LeftBrace, RightBrace,

    Minus, Plus, Slash, Star, StarStar, Percent,
    Comma, Dot, Colon, ColonColon, Semi, At, AtColon, Pipe, Arrow, Hash,
    Or, And,
    BitOr, BitAnd,
    Bang, BangEqual,
    Equal, EqualEqual, EqualEqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    Identifier, String, Number,
    Tilde,

    Let, Mut, Class, Fn,
    If, Else, For, While, Match,
    Break, Continue,
    Trace, Return, In, Catch,
    This, Prv, Static, New, Base, Init,
    Emit, On,
    True, False,

    EOF,

    Invalid
}

#[derive(Clone, Debug)]
pub struct Token {
    pub t: TokenType,
    pub val: String,
    pub pos: TokenPos
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let is_empty = self.val.trim().is_empty();
        if self.val.len() > 0 && !is_empty{
            write!(f, "{:?}: {}", self.t, self.val)
        }
        else {write!(f, "{:?}", self.t)}
    }
}

#[derive(Clone, Debug)]
pub struct TokenPos {
    pub from_col: u32,
    pub to_col: u32,
    pub line: u32
}

impl TokenPos {
    pub fn new(from_col: u32, to_col: u32, line: u32) -> TokenPos {
        TokenPos {
            from_col, to_col, line
        }
    }
}

#[derive(Clone)]
enum Nullable<T: Clone> {
    Val(T),
    Null
}

impl<T: Clone> Nullable<T> where T: Clone {
    pub fn get(&self) -> T where T: Clone {
        match self {
            Nullable::Val(val) => {
                return val.clone();
            }
            Nullable::Null => {
                panic!();
            }
        }
    }
    pub fn set(&mut self, val: T) where T: Clone {
        let test = Nullable::Val(val).clone();
        *self = test;
    }
}


pub struct TokenStream {
    lexer: Lexer,

    //last: Option<Token>,
    cur: Option<Token>
}

impl TokenStream {
    pub fn next(&mut self) -> Option<Token> {
        let last = self.cur.clone();
        self.cur = Some(self.lexer.lex_next());
        last
    }
    pub fn peek(&self) -> Option<&Token> {
        self.cur.as_ref()
    }
    // pub fn last(&self) -> Token {
    //     self._last.get()
    // }
    // pub fn cur(&self) -> Token {
    //     self._cur.get()
    // }
}