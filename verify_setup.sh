#!/bin/bash

echo "Verifying FORGE GitHub setup..."
echo "================================"

# Check if gh is authenticated
if ! gh auth status >/dev/null 2>&1; then
    echo "❌ GitHub CLI not authenticated"
    echo "   Run: gh auth login --git-protocol ssh --web"
    exit 1
else
    echo "✅ GitHub CLI authenticated"
fi

# Check if we're in a git repo
if ! git rev-parse --git-dir >/dev/null 2>&1; then
    echo "❌ Not in a git repository"
    exit 1
else
    echo "✅ Git repository found"
fi

# Check remote
if ! git remote get-url origin >/dev/null 2>&1; then
    echo "❌ No origin remote found"
    exit 1
else
    REMOTE=$(git remote get-url origin)
    echo "✅ Remote origin: $REMOTE"
fi

# Check branches
echo ""
echo "Local branches:"
git branch | sed 's/^/   /'

echo ""
echo "Required files:"
[ -f ".github/workflows/ci.yml" ] && echo "✅ CI workflow" || echo "❌ CI workflow missing"
[ -f ".github/workflows/auto-merge.yml" ] && echo "✅ Auto-merge workflow" || echo "❌ Auto-merge workflow missing"
[ -f "create_labels.sh" ] && echo "✅ Label creation script" || echo "❌ Label creation script missing"
[ -f "create_issues.sh" ] && echo "✅ Issue creation script" || echo "❌ Issue creation script missing"

echo ""
echo "Next steps:"
echo "1. Run: ./setup_github.sh"
echo "2. Configure branch protection on GitHub UI"
echo "3. Enable auto-merge in repository settings"
echo ""
echo "After setup, verify on GitHub:"
echo "- Labels: type-checker, auto-merge-ok, documentation"
echo "- Issues: TC-001, TC-002, TC-003, DOC-002"
echo "- Milestone: Week 2 - Capability Type-Checker"
echo "- PRs: Should show 2 open PRs after running setup"