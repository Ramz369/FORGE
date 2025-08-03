pub mod ast;
pub mod lexer;
pub mod parser;
pub mod ir;
pub mod lower;
pub mod typeck;

// Re-export commonly used types
pub use lexer::{Token, tokenize};
pub use ast::{Effect, Capability, Type, Expr, Stmt, Module};
pub use parser::{Parser, ParseError};
pub use ir::{IrModule, IrFunction, IrCapability};
pub use lower::lower_module;
pub use typeck::{TypeChecker, TypeError};

/// Legacy lexer function for backward compatibility
/// Deprecated: Use lexer::tokenize() instead
pub fn lex(src: &str) -> Vec<String> {
    src.split_whitespace().map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn simple_lex() {
        let toks = lex("hello world");
        assert_eq!(toks, vec!["hello", "world"]);
    }
    
    #[test]
    fn test_effect_hierarchy() {
        use ast::Effect;
        // Effect lattice: pure < alloc < io < net
        let effects = vec![Effect::Pure, Effect::Alloc, Effect::Io, Effect::Net];
        assert_eq!(effects.len(), 4);
    }
    
    #[test]
    fn test_effect_subsumption() {
        use ast::Effect;
        
        // Pure subsumes everything
        assert!(Effect::Pure.subsumes(&Effect::Pure));
        assert!(Effect::Pure.subsumes(&Effect::Alloc));
        assert!(Effect::Pure.subsumes(&Effect::Io));
        assert!(Effect::Pure.subsumes(&Effect::Net));
        
        // Alloc subsumes higher effects
        assert!(!Effect::Alloc.subsumes(&Effect::Pure));
        assert!(Effect::Alloc.subsumes(&Effect::Alloc));
        assert!(Effect::Alloc.subsumes(&Effect::Io));
        assert!(Effect::Alloc.subsumes(&Effect::Net));
        
        // Net only subsumes itself
        assert!(!Effect::Net.subsumes(&Effect::Pure));
        assert!(!Effect::Net.subsumes(&Effect::Alloc));
        assert!(!Effect::Net.subsumes(&Effect::Io));
        assert!(Effect::Net.subsumes(&Effect::Net));
    }
    
    #[test]
    fn test_effect_join() {
        use ast::Effect;
        
        assert_eq!(Effect::Pure.join(&Effect::Pure), Effect::Pure);
        assert_eq!(Effect::Pure.join(&Effect::Net), Effect::Net);
        assert_eq!(Effect::Alloc.join(&Effect::Io), Effect::Io);
        assert_eq!(Effect::Io.join(&Effect::Alloc), Effect::Io);
    }
}
