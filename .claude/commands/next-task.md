Complete the next OAuth2 migration task from start to finish.

# Task Developer - Automated Task Completion

You are executing a **complete task workflow** for ONE task from the OAuth2 Authentication Migration milestone.

## Your Mission

Complete ONE task from start to finish:
1. Find the next open task
2. Create a feature branch
3. Implement the solution
4. Write tests
5. Verify everything works
6. Commit and create PR
7. Report completion

## Step 1: Read Context

First, read these files to understand the project:

1. `CLAUDE.md` - Project architecture, patterns, and commands
2. `README_AUTH_MIGRATION.md` - Migration overview and workflow

## Step 2: Find Next Task

Find the next open task to work on:

```bash
gh issue list --milestone "OAuth2 Authentication Migration" \
  --state open \
  --json number,title,body,labels \
  --jq 'sort_by(.number) | .[0]'
```

**Critical**: Pick the LOWEST numbered task. This ensures dependencies are respected.

## Step 3: Verify Dependencies

Read the issue body. If it contains:

**Dependencies**: #1, #2

Then check those issues are closed:

```bash
gh issue view 1 --json state -q .state
gh issue view 2 --json state -q .state
```

If ANY dependency is "OPEN", STOP and report:
- "Cannot proceed with TASK-XX because dependencies are not complete"
- "Please complete: #1, #2 first"

## Step 4: Create Feature Branch

```bash
gh issue develop <issue-number> --checkout --name "task-XX-short-desc"
```

Example: `gh issue develop 1 --checkout --name "task-01-rsa-keys"`

This automatically assigns the issue to you.

## Step 5: Implement Solution

Follow the **Steps** section in the issue description EXACTLY:

**Before coding**:
- Read ALL steps in the issue first
- Use Read tool to understand existing code
- Identify which files need changes

**While coding**:
- Use Edit tool for modifications (don't rewrite entire files)
- Follow patterns from CLAUDE.md
- Keep changes minimal and focused
- Verify each step before moving to next

**Backend tasks** (Rust):
- Files in `backend/src/`
- Run `cargo check` after changes
- Follow existing module patterns

**Frontend tasks** (TypeScript):
- Files in `frontend/src/`
- Run `yarn build` to verify
- Follow existing component patterns

## Step 6: Write Tests

**Backend (Rust)**:
Add tests in the same file or in a `tests` module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        // Test implementation
    }
}
```

Run tests:
```bash
cd backend && cargo test
```

**Frontend (TypeScript)**:
- Primary test is TypeScript compilation
- Run: `cd frontend && yarn build`

## Step 7: Verify Everything

**Backend checklist**:
```bash
cd backend
cargo fmt              # Format code
cargo check            # Verify compilation
cargo test             # Run all tests
cargo clippy -- -D warnings  # Check for issues
```

All commands must succeed.

**Frontend checklist**:
```bash
cd frontend
yarn build             # Must compile without errors
yarn lint              # Must pass linting
```

**Manual testing** (if applicable):
- API endpoint → Test with `curl`
- Configuration → Verify server starts
- Database migration → Check it applies cleanly

## Step 8: Commit Changes

Create a clear commit message:

```bash
git add .

git commit -m "feat: <short description>

<Detailed description of changes>
- Bullet point 1
- Bullet point 2

Closes #<issue-number>"
```

**Commit types**:
- `feat:` - New feature
- `fix:` - Bug fix
- `refactor:` - Code restructuring
- `test:` - Adding tests
- `docs:` - Documentation
- `chore:` - Maintenance

## Step 9: Push and Create PR

```bash
# Push branch
git push -u origin <branch-name>

# Create PR
gh pr create \
  --title "[TASK-XX] <Task Title>" \
  --body "Closes #<issue-number>

## Changes
- Change 1
- Change 2

## Testing
- ✅ Cargo check passed
- ✅ All tests pass
- ✅ Manual testing complete

## Checklist
- [x] Follows project patterns
- [x] Tests added
- [x] Documentation updated (if needed)" \
  --assignee @me
```

## Step 10: Report Completion

Provide a clear summary:

```markdown
## ✅ Task Complete: TASK-XX - <Title>

**Issue**: #X
**PR**: #Y
**Branch**: task-XX-name

### Changes Made
- Implemented X
- Added Y
- Modified Z

### Files Changed
- `backend/src/file.rs` - Description
- `frontend/src/file.ts` - Description

### Verification
✅ All tests pass
✅ Compilation successful
✅ Manual testing complete

### Next Task
Next recommended task: TASK-XX (#Z)
```

## Critical Rules

❌ **DO NOT**:
- Work on multiple tasks
- Skip dependency checks
- Skip tests
- Commit broken code
- Modify unrelated files
- Create unnecessary files

✅ **DO**:
- Follow issue steps exactly
- Verify each step works
- Write tests for new code
- Keep commits focused
- Link PR to issue
- Verify all checks pass

## Pre-Flight Checklist

Before creating PR, verify:

- [ ] Read issue steps completely
- [ ] Checked dependencies are closed
- [ ] Created feature branch
- [ ] Followed all implementation steps
- [ ] Added tests
- [ ] All tests pass
- [ ] No compiler warnings
- [ ] No linter errors
- [ ] Manual testing done
- [ ] Commit message links issue
- [ ] Only modified relevant files

## Error Handling

If you encounter issues:

1. **Dependency not met**: Report and stop
2. **Compilation error**: Read error, fix, retry
3. **Test failure**: Debug and fix
4. **Unclear requirement**: Ask for clarification
5. **Git conflict**: Report issue

DO NOT proceed if dependencies are not met or if you encounter blocking errors.

## Success Criteria

Task is complete when:

✅ Feature branch created
✅ Solution implemented per issue steps
✅ Tests written and passing
✅ All verification steps complete
✅ PR created and linked to issue
✅ Completion report provided

## Remember

You complete ONE task from start to finish. Do not start a second task. Keep context minimal and focused.

Now begin! 🚀
