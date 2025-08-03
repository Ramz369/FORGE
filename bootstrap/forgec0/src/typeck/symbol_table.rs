/// Symbol table for type checking

use std::collections::HashMap;
use crate::ast::Type;

/// Symbol table mapping names to types
#[derive(Debug)]
pub struct SymbolTable {
    /// Current scope symbols
    symbols: HashMap<String, Type>,
    /// Parent scope (for nested scopes)
    parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    /// Create a new empty symbol table
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent: None,
        }
    }
    
    /// Insert a symbol with its type
    pub fn insert(&mut self, name: String, ty: Type) {
        self.symbols.insert(name, ty);
    }
    
    /// Look up a symbol's type
    pub fn lookup(&self, name: &str) -> Option<&Type> {
        self.symbols.get(name)
            .or_else(|| self.parent.as_ref()?.lookup(name))
    }
    
    /// Create a new nested scope
    pub fn push_scope(self) -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent: Some(Box::new(self)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_insert_and_lookup() {
        let mut table = SymbolTable::new();
        table.insert("x".to_string(), Type::Int);
        
        assert!(matches!(table.lookup("x"), Some(Type::Int)));
        assert!(table.lookup("y").is_none());
    }
    
    #[test]
    fn test_nested_scope() {
        let mut parent = SymbolTable::new();
        parent.insert("x".to_string(), Type::Int);
        
        let mut child = parent.push_scope();
        child.insert("y".to_string(), Type::Bool);
        
        assert!(matches!(child.lookup("x"), Some(Type::Int)));
        assert!(matches!(child.lookup("y"), Some(Type::Bool)));
    }
}