# Forge Lang Ω

[![CI](https://github.com/Ramz369/FORGE/actions/workflows/ci.yml/badge.svg)](https://github.com/Ramz369/FORGE/actions/workflows/ci.yml)

A revolutionary programming language designed as the genetic substrate for self-evolving AI ecosystems.

## Vision

Forge Lang Ω aims to deliver **10³× aggregate efficiency** over current software/hardware stacks through:
- **AI-first** language constructs
- **Hardware-fluid** compilation targets
- **LLM-agnostic** runtime execution

## Phase α Status

- ✅ Lexer with full Unicode support
- ✅ Parser with capability annotations
- ✅ AST with effect lattice
- ✅ Basic IR representation
- 🚧 Type checker (in progress)
- 🚧 WASM backend (in progress)

## Quick Start

```bash
# Build the bootstrap compiler
cargo build

# Run tests
cargo test

# Run capability demo
cargo run --example cap_demo
```

## Example

```forge
module demo.capabilities !{energy ≤ 10mJ}

fn process(data: Text) -> Int !{net, io, tokens ≤ 100}

fn pure_compute(x: Int, y: Int) -> Int !{pure}
```

## Documentation

- [Language Planning](docs/lc-01-plan.md)
- [Effect Lattice RFC](docs/rfcs/RFC-effect-lattice.md)
- [IR Design](docs/ir_draft.md)
- [Contributing](CONTRIBUTING.md)

## License

TBD

---

*This is Phase α of the Forge Lang Ω project. First compiler release targeted for Q1 2026.*