#!/bin/bash

echo "Setting up GitHub infrastructure for FORGE..."

# 1. Create labels
echo "Creating labels..."
./create_labels.sh

# 2. Create issues and milestone
echo "Creating issues and milestone..."
./create_issues.sh

# 3. Create PR for TC-001
echo "Creating PR for TC-001..."
gh pr create \
  --base main \
  --head tc-001-type-skeleton \
  --title "[TC-001] Type-checker skeleton" \
  --body "Implement basic type checker structure with symbol table and type unification.

## Changes
- Add typeck module with TypeChecker struct
- Implement symbol table with nested scopes
- Basic type representation and unification skeleton

## Testing
- Symbol table scope tests
- Basic type checker initialization

Closes #1" \
  --label "auto-merge-ok"

# 4. Create PR for TC-002
echo "Creating PR for TC-002..."
gh pr create \
  --base main \
  --head tc-002-capability-lattice \
  --title "[TC-002] Integrate capability lattice" \
  --body "Add capability subsumption checking using the effect lattice.

## Changes
- Add capability_check module with subsumption logic
- Implement effect_join() for least upper bound
- Add resource budget validation
- Include comprehensive tests

## Testing
- Effect subsumption tests
- Resource budget validation tests
- All tests passing ✅

Closes #2" \
  --label "type-checker"

echo "Setup complete! Check GitHub for:"
echo "- 4 issues created"
echo "- 1 milestone created"
echo "- 2 PRs created"
echo "- CI workflows ready"
echo ""
echo "Next steps:"
echo "1. Enable branch protection on 'main' (Settings > Branches)"
echo "2. Enable 'Allow auto-merge' in repository settings"
echo "3. Wait for CI to pass and auto-merge timer"