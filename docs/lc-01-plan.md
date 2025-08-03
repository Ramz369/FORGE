# LC-01: Capability Annotations in AST - Implementation Plan

## Overview
Extend the lexer and AST to support Forge's capability annotation syntax.

## Syntax to Support

### 1. Capability Declarations
```forge
fn process(data: Text) -> Int !{net, io} 
fn optimize(input: Vec<Int>) -> Vec<Int> !{tokens ≤ 100, latency ≤ 50ms}
module data.pipeline !{energy ≤ 10mJ}
```

### 2. Effect Annotations
- `!{pure}` - No side effects (default)
- `!{alloc}` - Can allocate memory
- `!{io}` - Can perform I/O operations  
- `!{net}` - Can make network calls

### 3. Resource Budgets
- `tokens ≤ N` - LLM token limit
- `latency ≤ Nms` - Execution time limit
- `energy ≤ NmJ` - Energy consumption limit

## Implementation Tasks

### Phase 1: Lexer Extensions
1. Add capability tokens to lexer:
   - `!` (Bang)
   - `{`, `}` (Braces)
   - `≤` (LessThanEqual)
   - Keywords: `tokens`, `latency`, `energy`, `ms`, `mJ`

2. Handle Unicode symbols:
   - `⟦`, `⟧` for intent blocks
   - `≤` for constraints

### Phase 2: AST Extensions
1. Capability node structure (✅ already drafted)
2. Add capability fields to:
   - Function definitions
   - Module declarations
   - Type signatures

### Phase 3: Parser Implementation
1. Parse capability syntax after `!`
2. Handle effect lists
3. Parse resource constraints
4. Validate constraint values

### Phase 4: Tests
1. Lexer tests for all new tokens
2. AST construction tests
3. Round-trip tests (parse → AST → pretty-print)
4. Error cases (invalid effects, negative budgets)

## Example Test Cases

```rust
#[test]
fn test_capability_lexing() {
    let tokens = lex("fn foo() !{net, tokens ≤ 8}");
    // Should produce: [Fn, Ident("foo"), LParen, RParen, Bang, LBrace, ...]
}

#[test]
fn test_effect_parsing() {
    let ast = parse("fn bar() !{io}");
    // Should have capability with Effect::Io
}
```

## Success Criteria
- ✅ All capability syntax tokens recognized
- ✅ AST nodes properly store capabilities
- ✅ Parser handles all syntax variations
- ✅ 100% test coverage on new code
- ✅ No regressions in existing tests

## Final Syntax Decisions

### Confirmed Syntax
1. **Capability Declaration**: `!{effects, constraints}`
2. **Effects**: `pure`, `alloc`, `io`, `net`
3. **Resource Constraints**: 
   - `tokens ≤ N`
   - `latency ≤ Nms` (ms suffix optional)
   - `energy ≤ NmJ` (mJ suffix optional)
4. **Unicode Support**: `≤`, `⟦`, `⟧` fully supported

### Effect Lattice
Implemented as: `pure ⊆ alloc ⊆ io ⊆ net`
- Subsumption checking via `Effect::subsumes()`
- Join/meet operations for effect inference

### Implementation Status
- ✅ Lexer: 265 lines, full Unicode support
- ✅ Parser: 278 lines, recursive descent
- ✅ AST: Extended with capability types
- ✅ Tests: 23 tests across 3 test files
- ✅ RFC: Effect lattice design documented