use crate::scanner::{Token, TokenType};

// Definiert die Arten von binären Operatoren
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub enum BinaryOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

// Definiert die Arten von unären Operatoren
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub enum UnaryOpKind {
    Neg,
}

// Definiert die verschiedenen Arten von Ausdrücken
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Expr {
    Number(f64),
    BinaryOp(BinaryOpKind, Box<Expr>, Box<Expr>),
    Var(String),
    Call(Box<Expr>, Vec<Expr>),
    UnaryOp(UnaryOpKind, Box<Expr>),
}

// Parser-Struktur
pub struct Parser<'src> {
    tokens: Vec<Token<'src>>,
    current: usize,
}

impl<'src> Parser<'src> {
    // Erstellt einen neuen Parser
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    // Parst einen Ausdruck
    pub fn expression(&mut self) -> Expr {
        return self.addition();
    }

    // Parst eine Addition oder Subtraktion
    fn addition(&mut self) -> Expr {
        let mut value = self.multiplication();

        // Überprüft auf Minus-Token und parst Subtraktion
        while self.match_token(TokenType::Minus) {
            value = Expr::BinaryOp(BinaryOpKind::Sub, Box::new(value), Box::new(self.multiplication()));
        }

        // Überprüft auf Plus-Token und parst Addition
        while self.match_token(TokenType::Plus) {
            value = Expr::BinaryOp(BinaryOpKind::Add, Box::new(value), Box::new(self.multiplication()));
        }

        value
    }

    // Parst eine Multiplikation oder Division
    fn multiplication(&mut self) -> Expr {
        let mut value = self.power();

        // Überprüft auf Slash-Token und parst Division
        while self.match_token(TokenType::Slash) {
            value = Expr::BinaryOp(BinaryOpKind::Div, Box::new(value), Box::new(self.power()));
        }

        // Überprüft auf Star-Token und parst Multiplikation
        while self.match_token(TokenType::Star) {
            value = Expr::BinaryOp(BinaryOpKind::Mul, Box::new(value), Box::new(self.power()));
        }

        value
    }

    // Parst eine Potenzierung
    fn power(&mut self) -> Expr {
        let mut value = self.unary();

        // Überprüft auf Power-Token und parst Potenzierung
        while self.match_token(TokenType::Power) {
            value = Expr::BinaryOp(BinaryOpKind::Pow, Box::new(value), Box::new(self.unary()));
        }

        value
    }

    // Parst einen unären Operator
    fn unary(&mut self) -> Expr {
        return if self.match_token(TokenType::Minus) {
            Expr::UnaryOp(UnaryOpKind::Neg, Box::new(self.unary()))
        } else {
            self.call()
        }
    }

    // Parst einen Funktionsaufruf
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

    // Beendet das Parsen eines Funktionsaufrufs
    fn finish_call(&mut self) -> Vec<Expr> {
        let mut values = vec![];

        self.consume_token(TokenType::LeftParen, "Erwarte '(' nach Funktionsname.");

        let mut arguments: usize = 0;

        while !self.match_token(TokenType::RightParen) {
            if arguments > 0 {
                self.consume_token(TokenType::Comma, "Erwarte ',' nach Funktionsargument.");
            }

            values.push(self.expression());
            arguments += 1;
        }

        values
    }

    // Parst einen primären Ausdruck
    fn primary(&mut self) -> Expr {
        match self.peek().clone().kind {
            TokenType::Number => {
                Expr::Number(self.consume_number("Erwarte Zahl."))
            },
            TokenType::Identifier => {
                Expr::Var(self.consume_identifier("Erwarte Bezeichner."))
            },
            TokenType::LeftParen => {
                self.advance();
                let expression = self.expression();
                self.consume_token(TokenType::RightParen, "Erwarte ')' nach Ausdruck.");
                expression
            },
            _ => {
                panic!("Erwarteter Ausdruck, aber {:?} erhalten", self.peek());
            }
        }
    }

    // Überprüft, ob das Ende der Token-Liste erreicht ist
    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::Eof
    }

    // Geht zum nächsten Token über
    fn advance(&mut self) {
        self.current += 1;
    }

    // Überprüft, ob das aktuelle Token dem gegebenen Token-Typ entspricht
    fn check(&self, tt: TokenType) -> bool {
        self.peek().kind == tt
    }

    // Überprüft, ob das aktuelle Token dem gegebenen Token-Typ entspricht und geht zum nächsten Token über
    fn match_token(&mut self, tt: TokenType) -> bool {
        if self.check(tt) {
            self.advance();
            true
        } else {
            false
        }
    }

    // Gibt das aktuelle Token zurück
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    // Gibt das nächste Token zurück
    fn peek_next(&self) -> &Token {
        &self.tokens[self.current + 1]
    }

    // Konsumiert das aktuelle Token, wenn es dem gegebenen Token-Typ entspricht, andernfalls wird eine Fehlermeldung ausgegeben
    fn consume_token(&mut self, tt: TokenType, message: &str) {
        if self.check(tt) {
            self.advance();
        } else {
            panic!("{} Erwartet {:?}. Erhalten {:?}", message, tt, self.peek());
        }
    }

    // Konsumiert ein Bezeichner-Token und gibt den Bezeichner als String zurück
    fn consume_identifier(&mut self, message: &str) -> String {
        let identifier = self.peek();

        if identifier.kind == TokenType::Identifier {
            let lexeme = identifier.lexeme.to_string();
            self.advance();
            lexeme
        } else {
            panic!("{} Erwartet Bezeichner.", message);
        }
    }

    // Konsumiert ein Zahlen-Token und gibt die Zahl als f64 zurück
    fn consume_number(&mut self, message: &str) -> f64 {
        let number = self.peek();

        if number.kind == TokenType::Number {
            let number = number.lexeme.parse::<f64>().unwrap();
            self.advance();
            number
        } else {
            panic!("{} Erwartet Zahl.", message);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;
    use super::*;

    // Tests für den Parser
}