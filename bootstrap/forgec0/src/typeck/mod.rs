/// Type checking module for Forge Lang - Phase α

use crate::ast::{Type, Module};

pub mod symbol_table;
use symbol_table::SymbolTable;

/// Type checker for Forge programs
#[derive(Debug)]
pub struct TypeChecker {
    /// Symbol table for type information
    symbols: SymbolTable,
    /// Type unification constraints
    constraints: Vec<(Type, Type)>,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        TypeChecker {
            symbols: SymbolTable::new(),
            constraints: Vec::new(),
        }
    }
    
    /// Check types for a module
    pub fn check_module(&mut self, _module: &Module) -> Result<(), TypeError> {
        // TODO: Implement module checking
        Ok(())
    }
}

/// Type checking errors
#[derive(Debug, Clone)]
pub enum TypeError {
    UnboundVariable(String),
    TypeMismatch { expected: Type, found: Type },
    UnificationFailure(Type, Type),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_type_checker() {
        let tc = TypeChecker::new();
        assert!(tc.constraints.is_empty());
    }
}