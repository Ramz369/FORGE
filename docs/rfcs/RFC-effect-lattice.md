# RFC: Effect Lattice for Forge Lang Ω

**Status**: Draft  
**Author**: Claude  
**Date**: 2025-08-03  
**Tracking**: CE-01

## Summary

Define the effect lattice hierarchy and subsumption rules for Forge's capability type system.

## Motivation

Forge needs a principled way to:
1. Track side effects through function composition
2. Enforce capability constraints at compile time
3. Allow safe effect subsumption (e.g., `pure` code can be used where `io` is allowed)

## Design

### Effect Hierarchy

```
pure ⊆ alloc ⊆ io ⊆ net

Where:
- pure: No observable side effects
- alloc: Can allocate memory
- io: Can perform I/O (includes alloc)
- net: Can make network calls (includes io)
```

### Lattice Properties

1. **Reflexivity**: e ⊆ e
2. **Transitivity**: if e₁ ⊆ e₂ and e₂ ⊆ e₃, then e₁ ⊆ e₃
3. **Join (⊔)**: least upper bound of two effects
4. **Meet (⊓)**: greatest lower bound of two effects

### Operations

```rust
impl Effect {
    /// Check if self is subsumed by other
    pub fn subsumes(&self, other: &Effect) -> bool {
        match (self, other) {
            (Effect::Pure, _) => true,
            (Effect::Alloc, Effect::Alloc) |
            (Effect::Alloc, Effect::Io) |
            (Effect::Alloc, Effect::Net) => true,
            (Effect::Io, Effect::Io) |
            (Effect::Io, Effect::Net) => true,
            (Effect::Net, Effect::Net) => true,
            _ => false,
        }
    }
    
    /// Join two effects (least upper bound)
    pub fn join(&self, other: &Effect) -> Effect {
        match (self, other) {
            (Effect::Net, _) | (_, Effect::Net) => Effect::Net,
            (Effect::Io, _) | (_, Effect::Io) => Effect::Io,
            (Effect::Alloc, _) | (_, Effect::Alloc) => Effect::Alloc,
            (Effect::Pure, Effect::Pure) => Effect::Pure,
        }
    }
}
```

### Function Composition

When composing functions, effects accumulate:

```forge
fn f() !{alloc}
fn g() !{io}
fn h() { f(); g() }  // inferred: !{io}
```

### Capability Inference Rules

1. **Empty capability** = `!{pure}`
2. **Function call**: inherits callee's effects
3. **Sequence**: join of all statement effects
4. **Conditional**: join of all branches

## Examples

### Valid Subsumption

```forge
fn pure_add(x: Int, y: Int) -> Int { x + y }         // !{pure}
fn process(f: fn(Int, Int) -> Int !{io}) !{io} {
    f(1, 2)  // OK: pure_add can be passed, pure ⊆ io
}
```

### Invalid Subsumption

```forge
fn network_fetch() -> Text !{net}
fn local_only(f: fn() -> Text !{io}) !{io} {
    f()  // ERROR: net ⊈ io
}
```

### Effect Propagation

```forge
fn allocate_buffer() -> Vec<Int> !{alloc}
fn write_file(data: Vec<Int>) !{io}
fn pipeline() {
    let buf = allocate_buffer();  // adds alloc
    write_file(buf);              // adds io
}  // inferred: !{io} (since alloc ⊆ io)
```

## Implementation Plan

1. Add effect ordering to `ast::Effect`
2. Implement subsumption checking
3. Add effect inference to type checker
4. Create comprehensive test suite

## Alternatives Considered

1. **Flat effects** (no hierarchy) - Too restrictive
2. **Effect polymorphism** - Too complex for Phase α
3. **Effect variables** - Deferred to Phase β

## Open Questions

1. Should we add `unsafe` as a top-level effect?
2. How do capability budgets interact with effect subsumption?
3. Should pure functions be allowed to panic?