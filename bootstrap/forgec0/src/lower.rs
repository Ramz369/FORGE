/// Lowering module - AST to IR conversion

use crate::ast;
use crate::ir;

/// Convert AST capability to IR capability
pub fn lower_capability(cap: &ast::Capability) -> ir::IrCapability {
    ir::IrCapability {
        effects: cap.effects.clone(),
        budgets: cap.budgets.clone(),
    }
}

/// Convert AST type to IR type string
pub fn lower_type(ty: &ast::Type) -> String {
    match ty {
        ast::Type::Int => "Int".to_string(),
        ast::Type::Text => "Text".to_string(),
        ast::Type::Bool => "Bool".to_string(),
        ast::Type::Array(inner) => format!("Array<{}>", lower_type(inner)),
        ast::Type::Function { returns, .. } => format!("Func<{}>", lower_type(returns)),
        ast::Type::Custom(name) => name.clone(),
    }
}

/// Lower AST module to IR module
pub fn lower_module(module: &ast::Module) -> ir::IrModule {
    let mut functions = Vec::new();
    
    // Convert each statement
    for stmt in &module.statements {
        match stmt {
            ast::Stmt::Function { name, params, returns, capability, body } => {
                let ir_params: Vec<(String, String)> = params.iter()
                    .map(|(n, t)| (n.clone(), lower_type(t)))
                    .collect();
                
                let mut ir_body = Vec::new();
                
                // Add a simple return for now
                ir_body.push(ir::IrInst::Return { value: None });
                
                functions.push(ir::IrFunction {
                    name: name.clone(),
                    params: ir_params,
                    returns: lower_type(returns),
                    capability: capability.as_ref().map(lower_capability),
                    body: ir_body,
                });
            }
            _ => {} // Skip other statements for now
        }
    }
    
    ir::IrModule {
        name: module.name.clone(),
        capability: module.capability.as_ref().map(lower_capability),
        functions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;
    
    #[test]
    fn test_lower_simple_function() {
        let mut parser = Parser::new("fn add(x: Int, y: Int) -> Int !{pure}");
        let func = parser.parse_function().unwrap();
        
        // Create a module with the function
        let module = ast::Module {
            name: "test".to_string(),
            capability: None,
            imports: vec![],
            statements: vec![func],
        };
        
        let ir_module = lower_module(&module);
        assert_eq!(ir_module.functions.len(), 1);
        
        let ir_func = &ir_module.functions[0];
        assert_eq!(ir_func.name, "add");
        assert_eq!(ir_func.params.len(), 2);
        assert_eq!(ir_func.returns, "Int");
        
        let cap = ir_func.capability.as_ref().unwrap();
        assert_eq!(cap.effects, vec![ast::Effect::Pure]);
    }
}