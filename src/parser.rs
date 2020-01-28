use std::slice::*;
use std::iter::*;
use crate::*;


#[repr(u8)]
#[derive(Clone)]
pub enum BP {
    Zero,
    Assignment,
    Or,
    And,
    BitOr,
    BitAnd,
    Equality,
    Comparison,
    Addition,
    Multitplication, //(and mod)
    Exponentiation,
    Unary,
    FnCall,
    Atom
}

impl Token {
    fn lbp(&self) -> u8 {
        match self.t {
            TokenType::Plus | TokenType::Minus => BP::Addition as u8,
            TokenType::Star | TokenType::Slash => BP::Multitplication as u8,
            TokenType::StarStar => BP::Exponentiation as u8,
            TokenType::RightParen | TokenType::LeftParen => BP::Atom as u8,
            _ => BP::Zero as u8
        }
    }
    fn assoc(&self) -> u8 {
        match self.t {
            TokenType::StarStar => 0,
            _ => 1
        }
    }
}

pub struct Parser<'a> {
    pub toks: Peekable<Iter<'a, Token>>
}

impl<'a> Parser<'a> {
    pub fn new(toks: Iter<'a, Token>) -> Parser<'a> {
        Parser{ toks: toks.peekable() }
    }

    pub fn expr(&mut self, rbp: u8) -> Result<Expr, String> {
        let mut left = self.nud()?;
        while self.is_next_tighter(rbp.clone()){
            if let Some(tok) = self.toks.peek() {
                if tok.t == TokenType::RightParen {
                    break;
                }
            }
            left = self.led(left)?;
        }
        Ok(left)
    }

    fn nud(&mut self) -> Result<Expr, String> {
        self.toks.next().map_or(Err("incomplete".to_string()), |tok| {
            match tok.t {
                TokenType::Number => Ok(Expr::Number(tok.clone())),
                TokenType::LeftParen => {
                    let val = self.expr(BP::Zero as u8);
                    let test = val.as_ref().expect("");
                    if let None = self.toks.peek() {
                        return Err("Unmatched (".to_string())
                    }
                    self.toks.next();
                    val
                },

                TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash
                => Err("Unexpected operator".to_string()),

                _ => Err("Expected a literal".to_string())
            }
        })
    }

    fn led(&mut self, expr: Expr) -> Result<Expr, String> {
        self.toks.next().map_or(Err("incomplete".to_string()), |tok| {
            match tok.t {
                TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash | TokenType::StarStar => {
                    let rhs = self.expr(tok.lbp() + tok.assoc())?;
                    Ok(Expr::Binary(Box::new(expr), tok.clone(), Box::new(rhs)))
                },
                TokenType::RightParen => Ok(expr),
                _ => Err("Expected an operator".to_string())
            }
        })
    }

    fn is_next_tighter(&mut self, rbp: u8) -> bool {
        self.toks.peek().map_or(false, |t| { t.lbp() as u8 >= rbp as u8})
    }
}
