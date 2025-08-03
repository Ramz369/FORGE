# FORGE GitHub Setup Troubleshooting

## Common Issues and Solutions

### 1. Auto-merge workflow fails with 403

**Symptom**: Auto-merge action fails with "Resource not accessible by integration"

**Solution**:
- Ensure `.github/workflows/auto-merge.yml` has permissions block:
  ```yaml
  permissions:
    contents: write
    pull-requests: write
  ```
- Check repository settings → Actions → General → Workflow permissions
- Select "Read and write permissions"

### 2. CI doesn't start on PR

**Symptom**: No CI checks appear on pull request

**Solutions**:
- Push workflows to `main` first: `git push origin main`
- Check branch protection rule includes required status checks
- Verify workflow syntax with: `actionlint .github/workflows/*.yml`

### 3. PR never auto-merges

**Symptom**: PR sits forever even with auto-merge-ok label

**Check**:
```bash
# Check PR size
git diff --stat origin/main...HEAD

# Should show < 200 additions
```

**Solutions**:
- Verify label is exactly `auto-merge-ok`
- Check Actions tab for workflow runs
- Ensure CI passed (green checkmark)
- Wait full 30 minutes after label added

### 4. Label/Issue creation fails

**Symptom**: `gh` commands fail with authentication errors

**Solution**:
```bash
# Re-authenticate with proper scopes
gh auth login --git-protocol ssh --web

# Select:
# - GitHub.com
# - SSH
# - Authenticate via web browser
# - Grant "repo" scope
```

### 5. Merge conflicts on TC-002

**After TC-001 merges**:
```bash
git checkout main
git pull origin main
git checkout tc-002-capability-lattice
git rebase main
# Resolve any conflicts
git push --force-with-lease
```

### 6. CI failures

**Format issues**:
```bash
cd bootstrap/forgec0
cargo fmt
```

**Clippy warnings**:
```bash
cargo clippy --fix
```

**Test failures**:
```bash
cargo test -- --nocapture
```

## Verification Commands

```bash
# Check workflow syntax locally
npm install -g @rhysd/actionlint
actionlint .github/workflows/*.yml

# Verify GitHub auth
gh auth status

# Check PR status
gh pr status

# View workflow runs
gh run list

# Check specific PR
gh pr view 1
```

## Emergency Fixes

If auto-merge is stuck:
```bash
# Manually merge if needed
gh pr merge 1 --squash --delete-branch
```

If workflows won't trigger:
```bash
# Force push to trigger
git commit --amend --no-edit
git push --force-with-lease
```