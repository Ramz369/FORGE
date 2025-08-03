#!/bin/bash

echo "🔍 FORGE GitHub Setup Audit"
echo "==========================="
echo ""

# Color codes for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Helper functions
check_pass() {
    echo -e "${GREEN}✅ $1${NC}"
}

check_fail() {
    echo -e "${RED}❌ $1${NC}"
}

check_warn() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

echo "1️⃣  Local Repository Checks"
echo "-------------------------"

# Check if in git repo
if git rev-parse --git-dir >/dev/null 2>&1; then
    check_pass "In git repository"
else
    check_fail "Not in git repository"
    exit 1
fi

# Check remote
REMOTE=$(git remote get-url origin 2>/dev/null)
if [ $? -eq 0 ]; then
    check_pass "Remote origin: $REMOTE"
else
    check_fail "No origin remote found"
fi

# Check branches
echo ""
echo "Branches:"
git branch -r | grep -E "(main|tc-001|tc-002)" | while read branch; do
    echo "  - $branch"
done

echo ""
echo "2️⃣  Workflow Files"
echo "----------------"

# Check CI workflow
if [ -f ".github/workflows/ci.yml" ]; then
    check_pass "CI workflow present"
    # Check for required jobs
    if grep -q "cargo fmt" .github/workflows/ci.yml; then
        echo "    ✓ Format check"
    fi
    if grep -q "cargo clippy" .github/workflows/ci.yml; then
        echo "    ✓ Clippy check"
    fi
    if grep -q "cargo test" .github/workflows/ci.yml; then
        echo "    ✓ Test runner"
    fi
else
    check_fail "CI workflow missing"
fi

# Check auto-merge workflow
if [ -f ".github/workflows/auto-merge.yml" ]; then
    check_pass "Auto-merge workflow present"
    
    # Check permissions
    if grep -q "permissions:" .github/workflows/auto-merge.yml; then
        if grep -A2 "permissions:" .github/workflows/auto-merge.yml | grep -q "contents: write" && \
           grep -A2 "permissions:" .github/workflows/auto-merge.yml | grep -q "pull-requests: write"; then
            echo "    ✓ Correct permissions"
        else
            check_warn "    Missing required permissions"
        fi
    else
        check_warn "    No permissions block found"
    fi
    
    # Check LOC limit
    if grep -q "additions > 200" .github/workflows/auto-merge.yml; then
        echo "    ✓ 200 LOC limit check"
    fi
    
    # Check cooldown
    if grep -q "sleep 1800" .github/workflows/auto-merge.yml; then
        echo "    ✓ 30-minute cooldown"
    fi
else
    check_fail "Auto-merge workflow missing"
fi

echo ""
echo "3️⃣  Setup Scripts"
echo "---------------"

[ -f "create_labels.sh" ] && check_pass "Label creation script" || check_fail "Label creation script missing"
[ -f "create_issues.sh" ] && check_pass "Issue creation script" || check_fail "Issue creation script missing"
[ -f "setup_github.sh" ] && check_pass "Setup script" || check_fail "Setup script missing"
[ -f "verify_setup.sh" ] && check_pass "Verify script" || check_fail "Verify script missing"

echo ""
echo "4️⃣  Code Structure"
echo "----------------"

# Check for key directories
[ -d "bootstrap/forgec0/src" ] && check_pass "Source directory" || check_fail "Source directory missing"
[ -d "docs" ] && check_pass "Documentation directory" || check_fail "Documentation directory missing"

# Check for type checker files
if [ -f "bootstrap/forgec0/src/capability_check.rs" ]; then
    check_pass "Capability checking module"
    LOC=$(wc -l < bootstrap/forgec0/src/capability_check.rs)
    echo "    Lines: $LOC"
fi

echo ""
echo "5️⃣  GitHub CLI"
echo "-------------"

if command -v gh &> /dev/null; then
    check_pass "GitHub CLI installed"
    if gh auth status >/dev/null 2>&1; then
        check_pass "GitHub CLI authenticated"
    else
        check_fail "GitHub CLI not authenticated - run: gh auth login"
    fi
else
    check_fail "GitHub CLI not installed"
fi

echo ""
echo "📋 GitHub UI Checklist"
echo "====================="
echo ""
echo "After running ./setup_github.sh, verify on GitHub:"
echo ""
echo "Settings → Branches:"
echo "  □ Rule for 'main' exists"
echo "  □ Require status checks enabled"
echo "  □ 'test' check is required"
echo "  □ Allow auto-merge enabled"
echo ""
echo "Settings → General → Pull Requests:"
echo "  □ Allow auto-merge toggled ON"
echo "  □ Auto-delete head branches ON"
echo ""
echo "Repository → Labels:"
echo "  □ auto-merge-ok (green)"
echo "  □ type-checker (blue)"
echo "  □ documentation (orange)"
echo ""
echo "Repository → Issues:"
echo "  □ TC-001: Type-checker skeleton"
echo "  □ TC-002: Integrate capability lattice"
echo "  □ TC-003: Constraint error reporting"
echo "  □ DOC-002: Type-checker design doc"
echo "  □ Milestone: Week 2 - Capability Type-Checker"
echo ""
echo "Repository → Pull Requests:"
echo "  □ PR #1: [TC-001] with auto-merge-ok label"
echo "  □ PR #2: [TC-002] without auto-merge label"
echo ""
echo "Actions tab:"
echo "  □ CI workflow runs on PRs"
echo "  □ Auto-merge workflow activates with label"
echo ""
echo "README:"
echo "  □ CI badge shows status (not 'no status')"

echo ""
echo "🚀 Next Steps"
echo "============"
echo ""
echo "1. If not authenticated: gh auth login --git-protocol ssh --web"
echo "2. Run: ./setup_github.sh"
echo "3. Configure branch protection in GitHub UI"
echo "4. Wait for TC-001 to auto-merge (35 min total)"
echo "5. Rebase TC-002 and continue development"