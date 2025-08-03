#!/bin/bash

# Create labels for the FORGE repository
echo "Creating labels for FORGE repository..."

# Create type-checker label
gh label create "type-checker" \
  --description "Type checker related tasks" \
  --color "0052CC"

# Create auto-merge-ok label
gh label create "auto-merge-ok" \
  --description "PRs under 200 LOC eligible for auto-merge" \
  --color "00FF00"

# Create documentation label
gh label create "documentation" \
  --description "Documentation tasks" \
  --color "FFA500"

echo "Labels created successfully!"