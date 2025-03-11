use crate::scanner::{Token, TokenType};


#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub enum BinaryOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub enum UnaryOpKind {
    Neg,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Expr {
    Number(f64),
    BinaryOp(BinaryOpKind, Box<Expr>, Box<Expr>),
    Var(String),
    Call(Box<Expr>, Vec<Expr>),
    UnaryOp(UnaryOpKind, Box<Expr>),
}


#[derive(PartialEq)]
enum FunctionKind {
    Function,
    Method,
    Anonymous,
}

pub struct Parser<'src> {
    tokens: Vec<Token<'src>>,
    current: usize,
}

impl<'src> Parser<'src> {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn expression(&mut self) -> Expr {
        return self.addition();
    }

    fn addition(&mut self) -> Expr {
        let mut value = self.multiplication();


        while self.match_token(TokenType::Minus) {
            value = Expr::BinaryOp(BinaryOpKind::Sub, Box::new(value), Box::new(self.multiplication()));
        }

        while self.match_token(TokenType::Plus) {
            value = Expr::BinaryOp(BinaryOpKind::Add, Box::new(value), Box::new(self.multiplication()));
        }

        value
    }

    fn multiplication(&mut self) -> Expr {
        let mut value = self.power();

        while self.match_token(TokenType::Slash) {
            value = Expr::BinaryOp(BinaryOpKind::Div, Box::new(value), Box::new(self.power()));
        }

        while self.match_token(TokenType::Star) {
            value = Expr::BinaryOp(BinaryOpKind::Mul, Box::new(value), Box::new(self.power()));
        }

        value
    }

    fn power(&mut self) -> Expr {
        let mut value = self.unary();

        while self.match_token(TokenType::Power) {
            value = Expr::BinaryOp(BinaryOpKind::Pow, Box::new(value), Box::new(self.unary()));
        }

        value
    }

    fn unary(&mut self) -> Expr {
        return if self.match_token(TokenType::Minus) {
            Expr::UnaryOp(UnaryOpKind::Neg, Box::new(self.unary()))
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Expr {
        let mut value = self.primary();

        loop {
            match self.peek().kind {
                TokenType::LeftParen => {
                    value = Expr::Call(Box::new(value), self.finish_call());
                }
                _ => break,
            }
        }

        value
    }

    fn finish_call(&mut self) -> Vec<Expr> {
        let mut values = vec![];

        self.consume_token(TokenType::LeftParen, "Expect '(' after function name.");

        let mut arguments: usize = 0;

        while !self.match_token(TokenType::RightParen) {
            if arguments > 0 {
                self.consume_token(TokenType::Comma, "Expect ',' after function argument.");
            }

            values.push(self.expression());
            arguments += 1;
        }

        values
    }

    fn primary(&mut self) -> Expr {
        match self.peek().clone().kind {
            TokenType::Number => {
                Expr::Number(self.consume_number("Expect number."))
            },
            TokenType::Identifier => {
                Expr::Var(self.consume_identifier("Expect identifier."))
            },
            TokenType::LeftParen => {
                self.advance();
                let expression = self.expression();
                self.consume_token(TokenType::RightParen, "Expect ')' after expression.");
                expression
            },
            _ => {
                panic!("Expected expression, got {:?}", self.peek());
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::Eof
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn check(&self, tt: TokenType) -> bool {
        self.peek().kind == tt
    }

    fn match_token(&mut self, tt: TokenType) -> bool {
        if self.check(tt) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek_next(&self) -> &Token {
        &self.tokens[self.current + 1]
    }

    fn consume_token(&mut self, tt: TokenType, message: &str) {
        if self.check(tt) {
            self.advance();
        } else {
            panic!("{} Expected {:?}. Got {:?}", message, tt, self.peek());
        }
    }

    fn consume_identifier(&mut self, message: &str) -> String {
        let identifier = self.peek();

        if identifier.kind == TokenType::Identifier {
            let lexeme = identifier.lexeme.to_string();
            self.advance();
            lexeme
        } else {
            panic!("{} Expected identifier.", message);
        }
    }

    fn consume_number(&mut self, message: &str) -> f64 {
        let number = self.peek();

        if number.kind == TokenType::Number {
            let number = number.lexeme.parse::<f64>().unwrap();
            self.advance();
            number
        } else {
            panic!("{} Expected number.", message);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;
    use super::*;

}