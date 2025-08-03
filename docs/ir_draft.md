# Forge-IR Draft (0.7)

* SSA form with explicit borrow/ownership tags.
* Instruction set: `const`, `call`, `phi`, `alloc`, `store`, `load`.
* Borrow tags: `&unique`, `&shared`, `move`.
* Capability field on every call node: `{effects: net | io | alloc}`.
