use crate::ast::Expr;
use crate::token::{SpannedToken, Token};

pub struct Parser<'a> {
    tokens: &'a [SpannedToken],
    current: usize,
}

macro_rules! peek_token {
    ($self:ident, $token:ident) => {
        match $self.peek() {
            Some(SpannedToken { node: Token::$token, .. }) => true,
            _ => false,
        }
    };
}

macro_rules! match_token {
    ($self:ident, $token:ident) => {
        if peek_token!($self, $token) {
            $self.advance();
            true
        } else {
            false
        }
    };
}

macro_rules! eat {
    ($self:ident, $token:ident) => {
        if peek_token!($self, $token) {
            $self.advance();
        } else {
            panic!("Expected token {:?}", Token::$token);
        }
    };
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [SpannedToken]) -> Self {
        Self { tokens, current: 0 }
    }
    
    pub fn parse(&mut self) -> Vec<Expr> {
        let mut exprs = Vec::new();
        
        while let Some(_) = self.peek() {
            exprs.push(self.expression());
        }
        
        exprs
    }
    
    fn peek(&self) -> Option<&SpannedToken> {
        self.tokens.get(self.current)
    }
    
    fn advance(&mut self) -> Option<&SpannedToken> {
        self.current += 1;
        self.tokens.get(self.current - 1)
    }
    
    fn lambda(&mut self) -> Expr {
        eat!(self, Pipe);
        let mut params = Vec::new();
        
        while let Some(SpannedToken { node: Token::Identifier(name), .. }) = self.peek() {
            params.push(name.clone());
            self.advance();
            
            if !match_token!(self, Comma) {
                break;
            }
        }
        
        eat!(self, Pipe);
        let body = self.expression();
        Expr::Lambda(params, Box::new(body))
    }
    
    fn block(&mut self) -> Expr {
        eat!(self, LBrace);
        let mut exprs = Vec::new();
        
        while !peek_token!(self, RBrace) {
            exprs.push(self.expression());
        }
        
        eat!(self, RBrace);
        Expr::Block(exprs)
    }
    
    fn if_expr(&mut self) -> Expr {
        eat!(self, If);
        let condition = self.expression();
        eat!(self, Then);
        let then_branch = self.expression();
        let else_branch = if match_token!(self, Else) {
            Some(Box::new(self.expression()))
        } else {
            None
        };
        
        Expr::If(Box::new(condition), Box::new(then_branch), else_branch)
    }
    
    fn expression(&mut self) -> Expr {
        self.logical_and()
    }
    
    fn logical_and(&mut self) -> Expr {
        let mut expr = self.comparison();
        
        while match_token!(self, And) {
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), Token::And, Box::new(right));
        }
        
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.sum();
        
        while match_token!(self, Equality)
            || match_token!(self, NotEqual)
            || match_token!(self, LessThan)
            || match_token!(self, GreaterThan)
            || match_token!(self, LessThanEqual)
            || match_token!(self, GreaterThanEqual)
        {
            let op = self.tokens[self.current - 1].node.clone();
            let right = self.sum();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        
        expr
    }
    
    fn sum(&mut self) -> Expr {
        let mut expr = self.product();
        
        while match_token!(self, Plus) || match_token!(self, Minus) {
            let op = self.tokens[self.current - 1].node.clone();
            let right = self.product();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        
        expr
    }
    
    fn product(&mut self) -> Expr {
        let mut expr = self.unary();
        
        while match_token!(self, Star) || match_token!(self, Slash) || match_token!(self, Percent) {
            let op = self.tokens[self.current - 1].node.clone();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        
        expr
    }
    
    fn unary(&mut self) -> Expr {
        if match_token!(self, Minus) || match_token!(self, Not) {
            let op = self.tokens[self.current - 1].node.clone();
            let right = self.unary();
            Expr::Unary(op, Box::new(right))
        } else {
            self.assign()
        }
    }
    
    fn assign(&mut self) -> Expr {
        let expr = self.call();
        
        if match_token!(self, Assign) {
            let value = self.expression();
            Expr::Assign(Box::new(expr), Box::new(value))
        } else {
            expr
        }
    }
    
    fn call(&mut self) -> Expr {
        let mut expr = self.atom();
        if peek_token!(self, LParen) || peek_token!(self, Pipe) || peek_token!(self, Dot) {
            loop {
                if peek_token!(self, LParen) {
                    let mut args = Vec::new();
                    eat!(self, LParen);

                    while !peek_token!(self, RParen) {
                        args.push(self.expression());

                        if !match_token!(self, Comma) {
                            break;
                        }
                    }

                    eat!(self, RParen);
                    
                    if peek_token!(self, Pipe) {
                        args.push(self.lambda());
                    }
                    
                    expr = Expr::Call(Box::new(expr), args);
                } else if peek_token!(self, Pipe) {
                    expr = Expr::Call(Box::new(expr), vec![self.lambda()]);
                } else if peek_token!(self, Dot) {
                    eat!(self, Dot);
                    let next = self.peek().cloned();
                    let property = if let Some(SpannedToken { node: Token::Identifier(name), .. }) = next {
                        self.advance();
                        name
                    } else {
                        panic!("Expected identifier");
                    };
                    expr = Expr::Property(Box::new(expr), property.clone());
                } else {
                    break;
                }
            }
        }
        
        expr
    }
    
    fn atom(&mut self) -> Expr {
        let next = self.peek().cloned();
        if let Some(SpannedToken { node: Token::Number(value), .. }) = next {
            self.advance();
            Expr::Number(value)
        } else if let Some(SpannedToken { node: Token::StringLiteral(value), .. }) = next {
            self.advance();
            Expr::StringLiteral(value)
        } else if match_token!(self, True) {
            Expr::Boolean(true)
        } else if match_token!(self, False) {
            Expr::Boolean(false)
        } else if match_token!(self, Nil) {
            Expr::Nil
        } else if let Some(SpannedToken { node: Token::Identifier(name), .. }) = next {
            self.advance();
            Expr::Identifier(name)
        } else if peek_token!(self, LBrace) {
            self.block()
        } else if peek_token!(self, Pipe) {
            self.lambda()
        } else if peek_token!(self, If) {
            self.if_expr()
        } else if match_token!(self, LParen) {
            if match_token!(self, Comma) {
                eat!(self, RParen);
                Expr::List(Vec::new())
            } else if match_token!(self, Colon) {
                eat!(self, RParen);
                Expr::Map(Vec::new())
            } else {
                let expr = self.expression();
                
                if match_token!(self, Comma) {
                    let mut list = vec![expr];
                    
                    while !peek_token!(self, RParen) {
                        list.push(self.expression());
                        
                        if !match_token!(self, Comma) {
                            break;
                        }
                    }
                    
                    eat!(self, RParen);
                    Expr::List(list)
                } else if match_token!(self, Colon) {
                    let mut map = vec![(expr, self.expression())];
                    
                    while !peek_token!(self, RParen) {
                        if !match_token!(self, Comma) {
                            break;
                        }
                        
                        let key = self.expression();
                        eat!(self, Colon);
                        let value = self.expression();
                        map.push((key, value));
                    }
                    
                    eat!(self, RParen);
                    Expr::Map(map)
                } else {
                    eat!(self, RParen);
                    expr
                }
            }
        } else {
            panic!("Unexpected token {:?}", self.peek().unwrap().node);
        }
    }
}