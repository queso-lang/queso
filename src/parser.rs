use crate::*;
use std::collections::HashMap;

#[repr(u8)]
#[derive(Clone)]
pub enum BP {
    Zero,
    KeywordExpr, //trace, return, throw, if, etc.
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
            ParserRule {prefix: Some(Parser::grouping), infix: Some(Parser::fn_call),   bp: BP::FnCall as u8});

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

        parser.rules.insert(TokenType::LeftBrace,
            ParserRule {prefix: Some(Parser::block),    infix: None,                    bp: BP::Zero as u8});

        parser.rules.insert(TokenType::Identifier,
            ParserRule {prefix: Some(Parser::access),   infix: None,                    bp: BP::Zero as u8});

        parser.rules.insert(TokenType::Equal,
            ParserRule {prefix: None,                   infix: Some(Parser::binary),    bp: BP::Assignment as u8});

        parser.rules.insert(TokenType::If,
            ParserRule {prefix: Some(Parser::if_else),  infix: None,                    bp: BP::Zero as u8});

        parser.rules.insert(TokenType::And,
            ParserRule {prefix: None,                   infix: Some(Parser::binary),    bp: BP::And as u8});

        parser.rules.insert(TokenType::Or,
            ParserRule {prefix: None,                   infix: Some(Parser::binary),    bp: BP::Or as u8});

        parser
    }
    
    fn get_rule(&self, t: TokenType) -> ParserRule {
        let default = ParserRule {prefix: None, infix: None, bp: BP::Zero as u8};
        self.rules.get(&t).unwrap_or(&default).clone()
    }

    fn consume(&mut self, t: TokenType, msg: &'static str) -> bool {
        let cur = self.toks.peek().clone();
        if cur.t == t {
            self.toks.next();
            return true;
        }
        self.error(cur.clone(), msg);
        false
    }

    fn error(&mut self, t: Token, msg: &'static str) {
        if (self.panic) {return};
        self.had_error = true;
        self.panic = true;
        error(t, msg);
    }

    fn sync(&mut self) {
        if self.panic {
            self.panic = false;
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

        let cur = cur.clone();
        // if there is no prefix rule for the current token, emit this error
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
            _ => panic!("This is a problem with the parser itself.")
        }
    }

    fn grouping(&mut self) -> Expr {
        self.toks.next();
        let expr = self.expr();
        self.consume(TokenType::RightParen, "Unmatched (");
        expr
    }

    fn block(&mut self) -> Expr {
        self.toks.next();
        let mut stmts = Vec::<Stmt>::new();
        let mut is_first = true;
        while self.toks.peek().t != TokenType::RightBrace && self.toks.peek().t != TokenType::EOF {
            stmts.push(self.stmt(is_first));
            is_first = false;
        }
        self.consume(TokenType::RightBrace, "Unmatched {");
        Expr::Block(stmts)
    }

    fn access(&mut self) -> Expr {
        let cur = self.toks.next();
        Expr::Access(cur.clone())
    }

    fn if_else(&mut self) -> Expr {
        self.toks.next();
        let cond = self.expr();
        self.consume(TokenType::Arrow, "Expected an arrow after the condition");
        let if_branch = self.expr();
        let mut else_branch = None;
        if self.toks.nextif(TokenType::Else) {
            else_branch = Some(Box::new(self.expr()));
        }
        Expr::IfElse(Box::new(cond), Box::new(if_branch), else_branch)
    }

    fn fn_call(&mut self, left: Expr) -> Expr {
        let mut args = Vec::<Expr>::new();
        self.toks.next();
        if self.toks.peek().t != TokenType::RightParen {
            loop {
                args.push(self.expr());

                if !self.toks.nextif(TokenType::Comma) {break;}
            }
        }
        self.consume(TokenType::RightParen, "Expected ) after arguments");

        let argslen = args.len();
        Expr::FnCall(Box::new(left), args, argslen as u16)
    }
}

// STMT
impl Parser {
    pub fn program(&mut self) -> Program {
        let mut stmts = Vec::<Stmt>::new();
        let mut is_first = true;
        while self.toks.peek().t != TokenType::EOF {
            stmts.push(self.stmt(is_first));
            is_first = false;
        }
        stmts
    }
    fn stmt(&mut self, is_first: bool) -> Stmt {
        if !is_first {
            if self.consume(TokenType::Semi, "Expected a SEMI after expression") {
                self.panic = false;
            }
            else {self.sync()};
        }
        
        match self.toks.peek().t {
            TokenType::Mut => self.mut_decl(),
            TokenType::Fn => self.fn_decl(),
            _ => self.expr_stmt()
        }
    }
    fn expr_stmt(&mut self) -> Stmt {
        let stmt = Stmt::Expr(Box::new(self.expr()));
        stmt
    }
    fn mut_decl(&mut self) -> Stmt {
        self.toks.next();
        let name = self.toks.peek().clone();
        self.consume(TokenType::Identifier, "Expected an identifier");

        let mut val = Expr::NullLiteral(name.clone());
        if self.toks.nextif(TokenType::Equal) {
            val = self.expr();
        }

        Stmt::MutDecl(name, Box::new(val))
    }
    fn fn_decl(&mut self) -> Stmt {
        self.toks.next();
        let name = self.toks.peek().clone();
        self.consume(TokenType::Identifier, "Expected function name");

        self.consume(TokenType::LeftParen, "Expected ( after function name");

        let mut params = Vec::<Token>::new();
        if self.toks.peek().t != TokenType::RightParen {
            loop {
                params.push(self.toks.peek().clone());
                self.consume(TokenType::Identifier, "Expected parameter name");

                if !self.toks.nextif(TokenType::Comma) {break;}
            }
        }

        self.consume(TokenType::RightParen, "Expected ) after parameters");

        self.consume(TokenType::Colon, "Expected :");

        let body = self.expr();
        Stmt::FnDecl(name, params, Box::new(body))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}