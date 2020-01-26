#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    LeftParen, RightParen, LeftBracket, RightBracket, LeftBrace, RightBrace,

    Minus, Plus, Slash, Star,
    Comma, Dot, Colon, ColonColon, Semi, At, AtColon, Pipe, Arrow,
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
    This, Prv, Static, New, Base,
    True, False,

    EOF,

    Invalid
}

pub struct Token {
    pub t: TokenType,
    pub val: String,
    // pub pos: Pos
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