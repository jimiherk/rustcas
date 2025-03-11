use std::path::Path;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub enum TokenType {
    // Einzeichen-Token.
    LeftParen, RightParen,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star, Power,

    // Ein- oder zweizeichen-Token.
    Equal,

    // Literale.
    Identifier, Number,

    Eof, Error
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Token<'src> {
    pub(crate) kind: TokenType,
    pub(crate) lexeme: &'src str,
    pub(crate) line: usize,
    column: usize,
}

impl<'src> Token<'src> {
    // Erzeugt ein synthetisches Token für Fehler
    pub fn synthetic(text: &'src str) -> Token<'src> {
        Token {
            kind: TokenType::Error,
            lexeme: text,
            line: 0,
            column: 0,
        }
    }
}

pub(crate) struct Scanner<'src> {
    source: &'src str,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl<'src> Scanner<'src> {
    // Erstellt einen neuen Scanner für die angegebene Quelltextzeichenkette
    pub fn new(source: &'src str) -> Scanner<'src> {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    // Scannt das nächste Token aus der Quelltextzeichenkette
    pub fn scan_token(&mut self) -> Token<'src> {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        match self.advance() {
            b'(' => self.make_token(TokenType::LeftParen),
            b')' => self.make_token(TokenType::RightParen),
            b';' => self.make_token(TokenType::Semicolon),
            b',' => self.make_token(TokenType::Comma),
            b'.' => self.make_token(TokenType::Dot),
            b'-' => self.make_token(TokenType::Minus),
            b'+' => self.make_token(TokenType::Plus),
            b'/' => self.make_token(TokenType::Slash),
            b'*' => self.make_token(TokenType::Star),
            b'=' => self.make_token(TokenType::Equal),
            b'^' => self.make_token(TokenType::Power),
            c if is_digit(c) => self.number(),
            c if is_alpha(c) => self.identifier(),
            _ => self.error_token("Unexpected character."),
        }
    }

    // Überprüft, ob das Ende der Quelltextzeichenkette erreicht ist
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // Gibt das aktuelle Lexem zurück
    fn lexeme(&self) -> &'src str {
        &self.source[self.start..self.current]
    }

    // Erstellt ein Token des angegebenen Typs
    fn make_token(&self, kind: TokenType) -> Token<'src> {
        Token {
            kind,
            lexeme: self.lexeme(),
            line: self.line,
            column: self.column,
        }
    }

    // Gibt das nächste Zeichen in der Quelltextzeichenkette zurück, ohne den aktuellen Index zu erhöhen
    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current]
        }
    }

    // Gibt das übernächste Zeichen in der Quelltextzeichenkette zurück, ohne den aktuellen Index zu erhöhen
    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current + 1]
        }
    }

    // Erstellt ein Fehler-Token mit der angegebenen Fehlermeldung
    fn error_token(&self, message: &'static str) -> Token<'src> {
        Token {
            kind: TokenType::Error,
            lexeme: message,
            line: self.line,
            column: self.column,
        }
    }

    // Erhöht den aktuellen Index und gibt das aktuelle Zeichen zurück
    fn advance(&mut self) -> u8 {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;
        self.column += 1;
        c
    }

    // Überprüft, ob das nächste Zeichen dem erwarteten Zeichen entspricht, und erhöht den aktuellen Index, falls dies der Fall ist
    fn matches(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.peek() != expected {
            return false;
        }

        self.current += 1;
        self.column += 1;
        true
    }

    // Überspringt Leerzeichen und Kommentare in der Quelltextzeichenkette
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                b' ' | b'\r' | b'\t' => {
                    self.advance();
                },
                b'\n' => {
                    self.line += 1;
                    self.column = 0;
                    self.advance();
                },
                b'/' if self.peek_next() == b'/' => {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                },
                b'/' if self.peek_next() == b'*' => {
                    // Unterstützt auch verschachtelte Kommentare
                    let mut depth = 1;
                    while depth > 0 && !self.is_at_end() {
                        if self.peek() == b'/' && self.peek_next() == b'*' {
                            depth += 1;
                            self.advance();
                        } else if self.peek() == b'*' && self.peek_next() == b'/' {
                            depth -= 1;
                            self.advance();
                        } else if self.peek() == b'\n' {
                            self.line += 1;
                            self.column = 0;
                        }
                        self.advance();
                    }
                },
                _ => return,
            }
        }
    }

    // Scannt eine Zahl aus der Quelltextzeichenkette
    fn number(&mut self) -> Token<'src> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == b'.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    // Scannt einen Bezeichner aus der Quelltextzeichenkette
    fn identifier(&mut self) -> Token<'src> {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    // Bestimmt den Typ des Bezeichners
    fn identifier_type(&self) -> TokenType {
        TokenType::Identifier
    }
}

// Überprüft, ob ein Zeichen eine Ziffer ist
fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}

// Überprüft, ob ein Zeichen ein Buchstabe oder Unterstrich ist
fn is_alpha(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}

// Überprüft, ob ein Zeichen eine alphanumerische Zeichen ist
fn is_alpha_numeric(c: u8) -> bool {
    is_alpha(c) || is_digit(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_tokens() {
        let source = "a = 1;";
        let mut scanner = Scanner::new(source);
        assert_eq!(scanner.scan_token().kind, TokenType::Identifier);
        assert_eq!(scanner.scan_token().kind, TokenType::Equal);
        assert_eq!(scanner.scan_token().kind, TokenType::Number);
        assert_eq!(scanner.scan_token().kind, TokenType::Semicolon);
        assert_eq!(scanner.scan_token().kind, TokenType::Eof);
    }
}