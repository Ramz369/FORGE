/// AST module for Forge Lang - Phase α
/// 
/// Core AST nodes with capability annotations support

use std::collections::HashMap;

/// Effect types that can be declared in capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Effect {
    Pure = 0,
    Alloc = 1,
    Io = 2,
    Net = 3,
}

impl Effect {
    /// Check if self is subsumed by other (self ⊆ other)
    pub fn subsumes(&self, other: &Effect) -> bool {
        self <= other
    }
    
    /// Join two effects (least upper bound)
    pub fn join(&self, other: &Effect) -> Effect {
        use std::cmp::max;
        max(self, other).clone()
    }
    
    /// Meet two effects (greatest lower bound)
    pub fn meet(&self, other: &Effect) -> Effect {
        use std::cmp::min;
        min(self, other).clone()
    }
}

/// Resource constraints in capability annotations
#[derive(Debug, Clone)]
pub struct ResourceBudget {
    pub tokens: Option<u32>,
    pub latency_ms: Option<u32>,
    pub energy_mj: Option<u32>,
}

/// Capability annotation: !{effects, resource budgets}
#[derive(Debug, Clone)]
pub struct Capability {
    pub effects: Vec<Effect>,
    pub budgets: ResourceBudget,
}

/// Type representations
#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Text,
    Bool,
    Array(Box<Type>),
    Function {
        params: Vec<Type>,
        returns: Box<Type>,
        capability: Option<Capability>,
    },
    Custom(String),
}

/// Expression nodes
#[derive(Debug, Clone)]
pub enum Expr {
    Ident(String),
    Number(i64),
    String(String),
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    IntentBlock {
        intent: String,
        constraints: HashMap<String, String>,
    },
}

/// Statement nodes
#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        ty: Option<Type>,
        value: Expr,
    },
    Function {
        name: String,
        params: Vec<(String, Type)>,
        returns: Type,
        capability: Option<Capability>,
        body: Vec<Stmt>,
    },
    Expression(Expr),
}

/// Module definition
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub capability: Option<Capability>,
    pub imports: Vec<String>,
    pub statements: Vec<Stmt>,
}

// TODO: Implement visitor pattern for AST traversal in LC-01