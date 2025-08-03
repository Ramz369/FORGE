/// Parser module for Forge Lang - Phase α
/// 
/// Recursive descent parser that builds AST from token stream

use crate::ast::*;
use crate::lexer::{Token, Lexer};

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: Token },
    UnexpectedEof,
    InvalidEffect(String),
    InvalidResourceBudget,
}

type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }
    
    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }
    
    fn expect(&mut self, expected: Token) -> ParseResult<()> {
        if self.current_token == expected {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: self.current_token.clone(),
            })
        }
    }
    
    fn expect_ident(&mut self) -> ParseResult<String> {
        match &self.current_token {
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: self.current_token.clone(),
            })
        }
    }
    
    fn expect_number(&mut self) -> ParseResult<u32> {
        match &self.current_token {
            Token::Number(n) => {
                let num = *n as u32;
                self.advance();
                Ok(num)
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "number".to_string(),
                found: self.current_token.clone(),
            })
        }
    }
    
    /// Parse capability annotation: !{effects, resource constraints}
    pub fn parse_capability(&mut self) -> ParseResult<Capability> {
        self.expect(Token::Bang)?;
        self.expect(Token::LBrace)?;
        
        let mut effects = Vec::new();
        let mut budgets = ResourceBudget {
            tokens: None,
            latency_ms: None,
            energy_mj: None,
        };
        
        loop {
            match &self.current_token {
                Token::RBrace => break,
                Token::Ident(name) => {
                    match name.as_str() {
                        // Effects
                        "pure" => {
                            effects.push(Effect::Pure);
                            self.advance();
                        }
                        "alloc" => {
                            effects.push(Effect::Alloc);
                            self.advance();
                        }
                        "io" => {
                            effects.push(Effect::Io);
                            self.advance();
                        }
                        "net" => {
                            effects.push(Effect::Net);
                            self.advance();
                        }
                        // Resource constraints
                        "tokens" => {
                            self.advance();
                            self.expect(Token::LessThanEqual)?;
                            budgets.tokens = Some(self.expect_number()?);
                        }
                        "latency" => {
                            self.advance();
                            self.expect(Token::LessThanEqual)?;
                            let ms = self.expect_number()?;
                            // Skip optional "ms" suffix
                            if let Token::Ident(suffix) = &self.current_token {
                                if suffix == "ms" {
                                    self.advance();
                                }
                            }
                            budgets.latency_ms = Some(ms);
                        }
                        "energy" => {
                            self.advance();
                            self.expect(Token::LessThanEqual)?;
                            let mj = self.expect_number()?;
                            // Skip optional "mJ" suffix
                            if let Token::Ident(suffix) = &self.current_token {
                                if suffix == "mJ" {
                                    self.advance();
                                }
                            }
                            budgets.energy_mj = Some(mj);
                        }
                        _ => return Err(ParseError::InvalidEffect(name.clone())),
                    }
                }
                Token::Comma => {
                    self.advance();
                    continue;
                }
                _ => return Err(ParseError::UnexpectedToken {
                    expected: "effect or resource constraint".to_string(),
                    found: self.current_token.clone(),
                }),
            }
            
            // Check for comma or closing brace
            match &self.current_token {
                Token::Comma => self.advance(),
                Token::RBrace => break,
                _ => return Err(ParseError::UnexpectedToken {
                    expected: ", or }".to_string(),
                    found: self.current_token.clone(),
                }),
            }
        }
        
        self.expect(Token::RBrace)?;
        
        Ok(Capability { effects, budgets })
    }
    
    /// Parse type annotation
    pub fn parse_type(&mut self) -> ParseResult<Type> {
        let type_name = self.expect_ident()?;
        
        match type_name.as_str() {
            "Int" => Ok(Type::Int),
            "Text" => Ok(Type::Text),
            "Bool" => Ok(Type::Bool),
            _ => Ok(Type::Custom(type_name)),
        }
    }
    
    /// Parse function parameter: (name: Type, ...)
    pub fn parse_params(&mut self) -> ParseResult<Vec<(String, Type)>> {
        self.expect(Token::LParen)?;
        let mut params = Vec::new();
        
        while self.current_token != Token::RParen {
            let name = self.expect_ident()?;
            self.expect(Token::Colon)?;
            let ty = self.parse_type()?;
            params.push((name, ty));
            
            match &self.current_token {
                Token::Comma => self.advance(),
                Token::RParen => break,
                _ => return Err(ParseError::UnexpectedToken {
                    expected: ", or )".to_string(),
                    found: self.current_token.clone(),
                }),
            }
        }
        
        self.expect(Token::RParen)?;
        Ok(params)
    }
    
    /// Parse function declaration
    pub fn parse_function(&mut self) -> ParseResult<Stmt> {
        self.expect(Token::Fn)?;
        let name = self.expect_ident()?;
        let params = self.parse_params()?;
        
        // Return type
        self.expect(Token::Arrow)?;
        let returns = self.parse_type()?;
        
        // Optional capability
        let capability = if self.current_token == Token::Bang {
            Some(self.parse_capability()?)
        } else {
            None
        };
        
        // For now, empty body
        let body = Vec::new();
        
        Ok(Stmt::Function {
            name,
            params,
            returns,
            capability,
            body,
        })
    }
    
    /// Parse module declaration
    pub fn parse_module(&mut self) -> ParseResult<Module> {
        self.expect(Token::Module)?;
        let name = self.expect_ident()?;
        
        // Handle dotted names (e.g., data.pipeline)
        let mut full_name = name;
        while let Token::Unknown('.') = self.current_token {
            self.advance();
            full_name.push('.');
            full_name.push_str(&self.expect_ident()?);
        }
        
        // Optional capability
        let capability = if self.current_token == Token::Bang {
            Some(self.parse_capability()?)
        } else {
            None
        };
        
        Ok(Module {
            name: full_name,
            capability,
            imports: Vec::new(),
            statements: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_capability() {
        let mut parser = Parser::new("!{net}");
        let cap = parser.parse_capability().unwrap();
        assert_eq!(cap.effects, vec![Effect::Net]);
        assert_eq!(cap.budgets.tokens, None);
    }
    
    #[test]
    fn test_parse_multiple_effects() {
        let mut parser = Parser::new("!{io, net, alloc}");
        let cap = parser.parse_capability().unwrap();
        assert_eq!(cap.effects.len(), 3);
        assert!(cap.effects.contains(&Effect::Io));
        assert!(cap.effects.contains(&Effect::Net));
        assert!(cap.effects.contains(&Effect::Alloc));
    }
    
    #[test]
    fn test_parse_resource_constraints() {
        let mut parser = Parser::new("!{tokens ≤ 100, latency ≤ 50ms}");
        let cap = parser.parse_capability().unwrap();
        assert_eq!(cap.budgets.tokens, Some(100));
        assert_eq!(cap.budgets.latency_ms, Some(50));
    }
    
    #[test]
    fn test_parse_function_with_capability() {
        let mut parser = Parser::new("fn process(data: Text) -> Int !{net, tokens ≤ 8}");
        match parser.parse_function().unwrap() {
            Stmt::Function { name, params, returns, capability, .. } => {
                assert_eq!(name, "process");
                assert_eq!(params.len(), 1);
                assert_eq!(params[0].0, "data");
                matches!(params[0].1, Type::Text);
                matches!(returns, Type::Int);
                
                let cap = capability.unwrap();
                assert!(cap.effects.contains(&Effect::Net));
                assert_eq!(cap.budgets.tokens, Some(8));
            }
            _ => panic!("Expected Function statement"),
        }
    }
}