/// Lexer module for Forge Lang - Phase α
/// 
/// This module will handle tokenization including:
/// - Basic tokens (identifiers, keywords, literals)
/// - Capability syntax: !{...}
/// - Intent blocks: ⟦...⟧
/// - Effect annotations

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Identifiers and literals
    Ident(String),
    Number(i64),
    String(String),
    
    // Keywords
    Fn,
    Let,
    Module,
    Use,
    
    // Capability tokens
    Bang,           // !
    LBrace,         // {
    RBrace,         // }
    
    // Intent block tokens
    IntentOpen,     // ⟦
    IntentClose,    // ⟧
    
    // Operators
    Arrow,          // ->
    LessThanEqual,  // ≤
    Colon,          // :
    Comma,          // ,
    
    // Delimiters
    LParen,         // (
    RParen,         // )
    
    // Special
    Eof,
    Unknown(char),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Ident(s) => write!(f, "Ident({})", s),
            Token::Number(n) => write!(f, "Number({})", n),
            Token::String(s) => write!(f, "String(\"{}\")", s),
            Token::Fn => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::Module => write!(f, "module"),
            Token::Use => write!(f, "use"),
            Token::Bang => write!(f, "!"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::IntentOpen => write!(f, "⟦"),
            Token::IntentClose => write!(f, "⟧"),
            Token::Arrow => write!(f, "->"),
            Token::LessThanEqual => write!(f, "≤"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Eof => write!(f, "EOF"),
            Token::Unknown(c) => write!(f, "Unknown({})", c),
        }
    }
}

/// Lexer implementation
#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            current_char: None,
        };
        lexer.read_char();
        lexer
    }
    
    fn read_char(&mut self) {
        self.current_char = self.input.get(self.position).copied();
        self.position += 1;
    }
    
    #[allow(dead_code)]
    fn peek_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }
    
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.read_char();
            } else {
                break;
            }
        }
        ident
    }
    
    fn read_number(&mut self) -> i64 {
        let mut num_str = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_numeric() {
                num_str.push(ch);
                self.read_char();
            } else {
                break;
            }
        }
        num_str.parse().unwrap_or(0)
    }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        match self.current_char {
            None => Token::Eof,
            Some(ch) => {
                let token = match ch {
                    '!' => {
                        self.read_char();
                        Token::Bang
                    }
                    '{' => {
                        self.read_char();
                        Token::LBrace
                    }
                    '}' => {
                        self.read_char();
                        Token::RBrace
                    }
                    '(' => {
                        self.read_char();
                        Token::LParen
                    }
                    ')' => {
                        self.read_char();
                        Token::RParen
                    }
                    ':' => {
                        self.read_char();
                        Token::Colon
                    }
                    ',' => {
                        self.read_char();
                        Token::Comma
                    }
                    '-' => {
                        self.read_char();
                        if self.current_char == Some('>') {
                            self.read_char();
                            Token::Arrow
                        } else {
                            Token::Unknown('-')
                        }
                    }
                    '⟦' => {
                        self.read_char();
                        Token::IntentOpen
                    }
                    '⟧' => {
                        self.read_char();
                        Token::IntentClose
                    }
                    '≤' => {
                        self.read_char();
                        Token::LessThanEqual
                    }
                    _ if ch.is_alphabetic() => {
                        let ident = self.read_identifier();
                        match ident.as_str() {
                            "fn" => Token::Fn,
                            "let" => Token::Let,
                            "module" => Token::Module,
                            "use" => Token::Use,
                            _ => Token::Ident(ident),
                        }
                    }
                    _ if ch.is_numeric() => {
                        let num = self.read_number();
                        Token::Number(num)
                    }
                    _ => {
                        self.read_char();
                        Token::Unknown(ch)
                    }
                };
                token
            }
        }
    }
}

/// Tokenize entire input
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }
    
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tokens() {
        let tokens = tokenize("fn foo()");
        assert_eq!(tokens[0], Token::Fn);
        assert_eq!(tokens[1], Token::Ident("foo".to_string()));
        assert_eq!(tokens[2], Token::LParen);
        assert_eq!(tokens[3], Token::RParen);
    }
    
    #[test]
    fn test_capability_tokens() {
        let tokens = tokenize("!{net, io}");
        assert_eq!(tokens[0], Token::Bang);
        assert_eq!(tokens[1], Token::LBrace);
        assert_eq!(tokens[2], Token::Ident("net".to_string()));
        assert_eq!(tokens[3], Token::Comma);
        assert_eq!(tokens[4], Token::Ident("io".to_string()));
        assert_eq!(tokens[5], Token::RBrace);
    }
    
    #[test]
    fn test_resource_constraint() {
        let tokens = tokenize("tokens ≤ 100");
        assert_eq!(tokens[0], Token::Ident("tokens".to_string()));
        assert_eq!(tokens[1], Token::LessThanEqual);
        assert_eq!(tokens[2], Token::Number(100));
    }
}