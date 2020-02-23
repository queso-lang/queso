use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    Null,

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

#[derive(Clone)]
pub struct TokenStream {
    lexer: Lexer,

    cur: Token
}

impl TokenStream {
    pub fn new(lexer: Lexer) -> TokenStream {
        let mut lexer = lexer;
        let next = lexer.lex_next();
        TokenStream {
            lexer,
            cur: next
        }
    }

    pub fn next(&mut self) -> Token {
        let last = self.cur.clone();
        self.cur = self.lexer.lex_next();
        last
    }
    pub fn peek(&self) -> &Token {
        &self.cur
    }

    pub fn nextif(&mut self, t: TokenType) -> bool {
        if self.peek().t == t {
            self.next();
            return true;
        }
        return false;
    }
}