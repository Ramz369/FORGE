#!/bin/bash

# Create milestone using GitHub API
echo "Creating milestone..."
gh api repos/Ramz369/FORGE/milestones \
  --method POST \
  --field title="Week 2 - Capability Type-Checker" \
  --field description="Type checker implementation milestone for week 2"

# Get milestone number (we'll assume it's the latest one created)
MILESTONE_NUMBER=$(gh api repos/Ramz369/FORGE/milestones --jq '.[0].number')

echo "Created milestone #$MILESTONE_NUMBER"

# Create Issue TC-001
echo "Creating issue TC-001..."
gh issue create \
  --title "TC-001: Type-checker skeleton - build symbol table, unify types" \
  --body "Implement basic type checker structure with symbol table and type unification" \
  --label "type-checker,auto-merge-ok" \
  --milestone "$MILESTONE_NUMBER"

# Create Issue TC-002
echo "Creating issue TC-002..."
gh issue create \
  --title "TC-002: Integrate capability lattice - use Effect::join / subsumes" \
  --body "Add capability subsumption checking using the effect lattice" \
  --label "type-checker" \
  --milestone "$MILESTONE_NUMBER"

# Create Issue TC-003
echo "Creating issue TC-003..."
gh issue create \
  --title "TC-003: Constraint error reporting - friendly messages + test cases" \
  --body "Implement user-friendly error messages for type constraint violations" \
  --label "type-checker" \
  --milestone "$MILESTONE_NUMBER"

# Create Issue DOC-002
echo "Creating issue DOC-002..."
gh issue create \
  --title "DOC-002: Type-checker design doc - /docs/TC-architecture.md" \
  --body "Document the type checker architecture and design decisions" \
  --label "documentation" \
  --milestone "$MILESTONE_NUMBER"

echo "All issues created and assigned to milestone!"