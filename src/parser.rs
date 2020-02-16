use crate::*;
use std::collections::HashMap;

#[repr(u8)]
#[derive(Clone)]
pub enum BP {
    Zero,
    KeywordExpr, //trace, return, throw,
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
    pub had_error: bool,
    panic: bool,
    rules: HashMap<TokenType, ParserRule>
}

// utils
impl Parser {
    pub fn new(toks: TokenStream) -> Parser {
        let rules: HashMap<TokenType, ParserRule> = HashMap::new();
        let mut parser = Parser {
            toks,
            had_error: false,
            rules,
            panic: false
        };

        parser.rules.insert(TokenType::LeftParen,
            ParserRule {prefix: Some(Parser::grouping), infix: None,                    bp: BP::Zero as u8});

        parser.rules.insert(TokenType::Minus,
            ParserRule {prefix: Some(Parser::unary),    infix: Some(Parser::binary),    bp: BP::Addition as u8});

        parser.rules.insert(TokenType::Plus,
            ParserRule {prefix: Some(Parser::unary),    infix: Some(Parser::binary),    bp: BP::Addition as u8});

        parser.rules.insert(TokenType::Slash,
            ParserRule {prefix: None,                   infix: Some(Parser::binary),    bp: BP::Multitplication as u8});

        parser.rules.insert(TokenType::Star,
            ParserRule {prefix: None,                   infix: Some(Parser::binary),    bp: BP::Multitplication as u8});

        parser.rules.insert(TokenType::Bang,
            ParserRule {prefix: Some(Parser::unary),    infix: None,                    bp: BP::Zero as u8});

        parser.rules.insert(TokenType::Number,
            ParserRule {prefix: Some(Parser::literal),  infix: None,                    bp: BP::Zero as u8});

        parser.rules.insert(TokenType::String,
            ParserRule {prefix: Some(Parser::literal),  infix: None,                    bp: BP::Zero as u8});

        parser.rules.insert(TokenType::True,
            ParserRule {prefix: Some(Parser::literal),  infix: None,                    bp: BP::Zero as u8});

        parser.rules.insert(TokenType::False,
            ParserRule {prefix: Some(Parser::literal),  infix: None,                    bp: BP::Zero as u8});

        parser.rules.insert(TokenType::Null,
            ParserRule {prefix: Some(Parser::literal),  infix: None,                    bp: BP::Zero as u8});

        parser.rules.insert(TokenType::EqualEqual,
            ParserRule {prefix: None,                   infix: Some(Parser::binary),    bp: BP::Equality as u8});

        parser.rules.insert(TokenType::BangEqual,
            ParserRule {prefix: None,                   infix: Some(Parser::binary),    bp: BP::Equality as u8});

        parser.rules.insert(TokenType::Greater,
            ParserRule {prefix: None,                   infix: Some(Parser::binary),    bp: BP::Comparison as u8});

        parser.rules.insert(TokenType::GreaterEqual,
            ParserRule {prefix: None,                   infix: Some(Parser::binary),    bp: BP::Comparison as u8});

        parser.rules.insert(TokenType::Less,
            ParserRule {prefix: None,                   infix: Some(Parser::binary),    bp: BP::Comparison as u8});

        parser.rules.insert(TokenType::LessEqual,
            ParserRule {prefix: None,                   infix: Some(Parser::binary),    bp: BP::Comparison as u8});

        parser.rules.insert(TokenType::Trace,
            ParserRule {prefix: Some(Parser::unarykw),  infix: None,                    bp: BP::Zero as u8});

        parser
    }
    
    fn get_rule(&self, t: TokenType) -> ParserRule {
        let default = ParserRule {prefix: None, infix: None, bp: BP::Zero as u8};
        self.rules.get(&t).unwrap_or(&default).clone()
    }

    fn consume(&mut self, t: TokenType, msg: &'static str) -> TokenType {
        let cur = self.toks.peek().clone();
        if cur.t == t {
            self.toks.next();
            return cur.t;
        }
        self.error(cur.clone(), msg);
        cur.t
    }

    // fn consume_and_sync(&mut self, t: TokenType, msg: &'static str) {
    //     self.consume(t, msg);
    // }

    fn error(&mut self, t: Token, msg: &'static str) {
        if (self.panic) {return};
        self.had_error = true;
        self.panic = true;
        error(t, msg);
    }

    fn sync(&mut self) {
        if self.panic {
            self.panic = false;
            println!("{:?}", self.toks.peek().t);
            while self.toks.peek().t != TokenType::EOF {
                match self.toks.next().t {
                    TokenType::Semi
                    | TokenType::Class
                    | TokenType::Fn
                    | TokenType::Return
                    | TokenType::Let
                    | TokenType::Mut
                    => return,

                    _ => {}
                }
            }
        }
    }
}

// EXPR
impl Parser {
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

        let cur = self.toks.peek().clone();
        self.error(cur, "Expected an expression");
        return Expr::Error;
    }

    pub fn expr(&mut self) -> Expr {
        self.parse_bp(BP::Assignment as u8)
    }

    fn unary(&mut self) -> Expr {
        let op = self.toks.next();
        let expr = self.parse_bp(BP::Unary as u8);
        Expr::Unary(op, Box::new(expr))
    }

    fn unarykw(&mut self) -> Expr {
        let op = self.toks.next();
        let expr = self.parse_bp(BP::KeywordExpr as u8);
        Expr::Unary(op, Box::new(expr))
    }

    fn binary(&mut self, left: Expr) -> Expr {
        let op = self.toks.next().clone();
        
        let rule = self.get_rule(op.t);
        let right = self.parse_bp(rule.bp + 1);

        Expr::Binary(Box::new(left), op, Box::new(right))
    }

    fn literal(&mut self) -> Expr {
        let tok = self.toks.next().clone();
        match tok.t {
            TokenType::Number => Expr::Constant(tok),
            TokenType::String => Expr::Constant(tok),
            TokenType::True   => Expr::TrueLiteral(tok),
            TokenType::False  => Expr::FalseLiteral(tok),
            TokenType::Null   => Expr::NullLiteral(tok),
            _ => panic!("This is a problem with the interpreter itself.")
        }
    }

    fn grouping(&mut self) -> Expr {
        self.toks.next();
        let expr = self.expr();
        self.consume(TokenType::RightParen, "Unmatched (");
        expr
    }
}

// STMT
impl Parser {
    pub fn program(&mut self) -> Stmt {
        let mut stmts = Vec::<Stmt>::new();
        while self.toks.peek().t != TokenType::EOF {
            stmts.push(self.stmt());
        }
        Stmt::Program(stmts)
    }
    fn stmt(&mut self) -> Stmt {
        let stmt = self.expr_stmt();
        if self.consume(TokenType::Semi, "Expected a SEMI after expression") == TokenType::Semi {
            self.panic = false;
        }
        else {self.sync()};
        stmt
    }
    fn expr_stmt(&mut self) -> Stmt {
        let stmt = Stmt::Expr(self.expr());
        //self.consume_and_sync(TokenType::Semi, "Expected a SEMI after expression");
        stmt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}