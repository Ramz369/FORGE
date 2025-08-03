use forgec0::{Parser, Stmt, lower_module};
use std::fs;

fn main() {
    // Read the demo file
    let input = fs::read_to_string("examples/cap_demo.fg")
        .unwrap_or_else(|_| {
            // Fallback to embedded example if file not found
            include_str!("../../../examples/cap_demo.fg").to_string()
        });
    
    println!("=== Forge Source ===");
    println!("{}", input);
    println!();
    
    // Parse module declaration
    let mut parser = Parser::new(&input);
    let mut module = parser.parse_module().expect("Failed to parse module");
    
    // Parse remaining function declarations
    let lines = input.lines().skip(1); // Skip module declaration
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        let mut parser = Parser::new(line);
        if line.starts_with("fn") {
            match parser.parse_function() {
                Ok(func) => module.statements.push(func),
                Err(e) => eprintln!("Parse error: {:?}", e),
            }
        }
    }
    
    println!("=== Parsed AST ===");
    println!("Module: {}", module.name);
    if let Some(cap) = &module.capability {
        println!("  Capability: {:?}", cap);
    }
    println!("  Functions: {}", module.statements.len());
    for stmt in &module.statements {
        if let Stmt::Function { name, capability, .. } = stmt {
            println!("    - {} {:?}", name, capability);
        }
    }
    println!();
    
    // Lower to IR
    let ir_module = lower_module(&module);
    
    println!("=== Forge IR ===");
    print!("{}", ir_module.debug_print());
}

#[test]
fn test_cap_demo_parses() {
    // Test that the example compiles and runs
    main();
}