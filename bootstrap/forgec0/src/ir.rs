/// Forge IR module - Phase α
/// 
/// Intermediate representation for Forge programs

use crate::ast::{Effect, ResourceBudget};

/// IR capability (mirrors AST capability)
#[derive(Debug, Clone)]
pub struct IrCapability {
    pub effects: Vec<Effect>,
    pub budgets: ResourceBudget,
}

/// IR instruction types
#[derive(Debug, Clone)]
pub enum IrInst {
    /// Constant value
    Const { dest: String, value: IrValue },
    
    /// Function call with capability
    Call {
        dest: String,
        func: String,
        args: Vec<String>,
        capability: Option<IrCapability>,
    },
    
    /// Allocate memory
    Alloc { dest: String, size: u32 },
    
    /// Return value
    Return { value: Option<String> },
}

/// IR value types
#[derive(Debug, Clone)]
pub enum IrValue {
    Int(i64),
    Text(String),
    Bool(bool),
}

/// IR function definition
#[derive(Debug)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<(String, String)>, // (name, type)
    pub returns: String,
    pub capability: Option<IrCapability>,
    pub body: Vec<IrInst>,
}

/// IR module (compilation unit)
#[derive(Debug)]
pub struct IrModule {
    pub name: String,
    pub capability: Option<IrCapability>,
    pub functions: Vec<IrFunction>,
}

impl IrModule {
    /// Pretty-print IR for debugging
    pub fn debug_print(&self) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("module {} ", self.name));
        if let Some(cap) = &self.capability {
            output.push_str(&format!("!{:?} ", cap));
        }
        output.push_str("{\n");
        
        for func in &self.functions {
            output.push_str(&format!("\n  fn {}(", func.name));
            for (i, (name, ty)) in func.params.iter().enumerate() {
                if i > 0 { output.push_str(", "); }
                output.push_str(&format!("{}: {}", name, ty));
            }
            output.push_str(&format!(") -> {} ", func.returns));
            
            if let Some(cap) = &func.capability {
                output.push_str(&format!("!{{"));
                for (i, effect) in cap.effects.iter().enumerate() {
                    if i > 0 { output.push_str(", "); }
                    output.push_str(&format!("{:?}", effect).to_lowercase());
                }
                if let Some(tokens) = cap.budgets.tokens {
                    output.push_str(&format!(", tokens ≤ {}", tokens));
                }
                if let Some(latency) = cap.budgets.latency_ms {
                    output.push_str(&format!(", latency ≤ {}ms", latency));
                }
                output.push_str("} ");
            }
            
            output.push_str("{\n");
            for inst in &func.body {
                output.push_str(&format!("    {:?}\n", inst));
            }
            output.push_str("  }\n");
        }
        
        output.push_str("}\n");
        output
    }
}