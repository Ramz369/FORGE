use forgec0::{Parser, Stmt, Module, Effect, Type};

#[test]
fn test_parse_module_with_capability() {
    let mut parser = Parser::new("module data.pipeline !{energy ≤ 10mJ}");
    let module = parser.parse_module().unwrap();
    
    assert_eq!(module.name, "data.pipeline");
    assert!(module.capability.is_some());
    
    let cap = module.capability.unwrap();
    assert_eq!(cap.budgets.energy_mj, Some(10));
}

#[test]
fn test_parse_function_multiple_params() {
    let mut parser = Parser::new("fn transform(input: Vec, config: Config) -> Result !{io, alloc}");
    
    match parser.parse_function().unwrap() {
        Stmt::Function { name, params, returns, capability, .. } => {
            assert_eq!(name, "transform");
            assert_eq!(params.len(), 2);
            
            assert_eq!(params[0].0, "input");
            matches!(params[0].1, Type::Custom(ref s) if s == "Vec");
            
            assert_eq!(params[1].0, "config");
            matches!(params[1].1, Type::Custom(ref s) if s == "Config");
            
            matches!(returns, Type::Custom(ref s) if s == "Result");
            
            let cap = capability.unwrap();
            assert_eq!(cap.effects.len(), 2);
            assert!(cap.effects.contains(&Effect::Io));
            assert!(cap.effects.contains(&Effect::Alloc));
        }
        _ => panic!("Expected Function statement"),
    }
}

#[test]
fn test_parse_complex_capability() {
    let mut parser = Parser::new("!{net, io, tokens ≤ 100, latency ≤ 200ms, energy ≤ 5mJ}");
    let cap = parser.parse_capability().unwrap();
    
    // Check effects
    assert_eq!(cap.effects.len(), 2);
    assert!(cap.effects.contains(&Effect::Net));
    assert!(cap.effects.contains(&Effect::Io));
    
    // Check resource budgets
    assert_eq!(cap.budgets.tokens, Some(100));
    assert_eq!(cap.budgets.latency_ms, Some(200));
    assert_eq!(cap.budgets.energy_mj, Some(5));
}

#[test]
fn test_parse_pure_effect() {
    let mut parser = Parser::new("!{pure}");
    let cap = parser.parse_capability().unwrap();
    assert_eq!(cap.effects, vec![Effect::Pure]);
}

#[test]
fn test_parse_function_without_capability() {
    let mut parser = Parser::new("fn add(x: Int, y: Int) -> Int");
    
    match parser.parse_function().unwrap() {
        Stmt::Function { name, params, returns, capability, .. } => {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
            assert!(capability.is_none());
            matches!(returns, Type::Int);
        }
        _ => panic!("Expected Function statement"),
    }
}

#[test]
fn test_empty_capability() {
    let mut parser = Parser::new("!{}");
    let cap = parser.parse_capability().unwrap();
    assert!(cap.effects.is_empty());
    assert!(cap.budgets.tokens.is_none());
    assert!(cap.budgets.latency_ms.is_none());
    assert!(cap.budgets.energy_mj.is_none());
}

#[test]
fn test_module_simple_name() {
    let mut parser = Parser::new("module main");
    let module = parser.parse_module().unwrap();
    assert_eq!(module.name, "main");
    assert!(module.capability.is_none());
}

#[test]
fn test_all_effects() {
    let mut parser = Parser::new("!{pure, alloc, io, net}");
    let cap = parser.parse_capability().unwrap();
    
    assert_eq!(cap.effects.len(), 4);
    assert!(cap.effects.contains(&Effect::Pure));
    assert!(cap.effects.contains(&Effect::Alloc));
    assert!(cap.effects.contains(&Effect::Io));
    assert!(cap.effects.contains(&Effect::Net));
}