use crate::*;
use std::collections::HashMap;

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

#[derive(Clone)]
pub struct ParserRule {
    prefix: Option<PrefixFn>,
    infix: Option<InfixFn>,
    bp: u8
}

type PrefixFn = fn(&mut Parser) -> Expr;
type InfixFn = fn(&mut Parser, Expr) -> Expr;

pub struct Parser {
    toks: TokenStream,

    rules: HashMap<TokenType, ParserRule>
}

impl Parser {
    pub fn new(toks: TokenStream) -> Parser {
        let rules: HashMap<TokenType, ParserRule> = HashMap::new();
        let mut parser = Parser {
            toks,
            rules
        };

        parser.rules.insert(TokenType::LeftParen,
            ParserRule {prefix: Some(Parser::grouping), infix: None,                    bp: BP::Zero as u8});

        parser.rules.insert(TokenType::Minus,
            ParserRule {prefix: Some(Parser::unary),    infix: Some(Parser::binary),    bp: BP::Addition as u8});

        parser.rules.insert(TokenType::Plus,
            ParserRule {prefix: None,                   infix: Some(Parser::binary),    bp: BP::Addition as u8});

        parser.rules.insert(TokenType::Slash,
            ParserRule {prefix: None,                   infix: Some(Parser::binary),    bp: BP::Multitplication as u8});

        parser.rules.insert(TokenType::Star,
            ParserRule {prefix: None,                   infix: Some(Parser::binary),    bp: BP::Multitplication as u8});

        parser.rules.insert(TokenType::Bang,
            ParserRule {prefix: Some(Parser::unary),    infix: None,                    bp: BP::Zero as u8});

        parser.rules.insert(TokenType::Number,
            ParserRule {prefix: Some(Parser::number),   infix: None,                    bp: BP::Zero as u8});

        parser
    }
}

// EXPR
impl Parser {
    fn consume(&mut self, t: TokenType, msg: &'static str) {
        if self.toks.peek().t == t {
            self.toks.next();
            return;
        }
        error(self.toks.peek(), msg);
    }

    fn parse_bp(&mut self, bp: u8) -> Expr {
        let cur = self.toks.peek();
        let prefix_rule = self.get_rule(cur.t).prefix;
        
        if let Some(f) = prefix_rule {
            let mut left = f(self);
            loop {
                let cur = self.toks.peek().clone();
                if bp > self.get_rule(cur.t).bp {break;}

                let infix_rule = self.get_rule(cur.t).infix;
                left = infix_rule.expect("This is an error with the compiler itself")(self, left);
            }
            return left;
        }

        error(self.toks.peek(), "Expected an expression");
        return Expr::Error;
    }

    fn get_rule(&self, t: TokenType) -> ParserRule {
        let default = ParserRule {prefix: None, infix: None, bp: BP::Zero as u8};
        self.rules.get(&t).unwrap_or(&default).clone()
    }

    pub fn expr(&mut self) -> Expr {
        self.parse_bp(BP::Assignment as u8)
    }

    fn unary(&mut self) -> Expr {
        let op = self.toks.next();
        let expr = self.parse_bp(BP::Unary as u8);
        Expr::Unary(op, Box::new(expr))
    }

    fn binary(&mut self, left: Expr) -> Expr {
        let op = self.toks.next().clone();
        
        let rule = self.get_rule(op.t);
        let right = self.parse_bp(rule.bp + 1);

        Expr::Binary(Box::new(left), op, Box::new(right))
    }

    fn number(&mut self) -> Expr {
        Expr::Constant(self.toks.next().clone())
    }

    fn grouping(&mut self) -> Expr {
        self.toks.next();
        let expr = self.expr();
        self.consume(TokenType::RightParen, "Unmatched (");
        expr
    }
