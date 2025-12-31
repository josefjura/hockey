---
name: publish-version
description: Automate versioning and publishing workflow. Use when preparing a release, bumping version numbers, updating CHANGELOG.md, or publishing changes. Handles version updates in Cargo.toml, changelog finalization, git commits, tags, and push.
allowed-tools: Bash(cargo:*), Bash(git:*), Bash(gh:*), Read, Edit, Grep, Glob
---

# Publish & Versioning Workflow

## Overview

This skill automates the complete versioning and publishing workflow for releasing new versions of the hockey application:

1. **Determine version number** (from argument or ask user)
2. **Validate version format** (semantic versioning: x.y.z)
3. **Update Cargo.toml** with new version
4. **Update CHANGELOG.md** - move [Unreleased] to new version with today's date
5. **Run pre-commit checks** to ensure quality
6. **Create git commit** with "chore: release v{version}" message
7. **Create git tag** with v{version}
8. **Push commit and tag** to origin
9. **Verify build action started** using gh CLI

## Instructions

### Step 1: Determine Version Number

**If version provided as argument:**
- Use the provided version (e.g., "1.2.3")
- Validate it matches semantic versioning format (x.y.z where x, y, z are numbers)

**If no version provided:**
- Read current version from `Cargo.toml` (look for `version = "x.y.z"`)
- Ask user what the new version should be
- Suggest common options:
  - Patch bump: x.y.z → x.y.(z+1) for bug fixes
  - Minor bump: x.y.z → x.(y+1).0 for new features
  - Major bump: x.y.z → (x+1).0.0 for breaking changes

### Step 2: Validate Working Tree

Before starting, ensure the working tree is clean:

```bash
git status --porcelain
```

If there are uncommitted changes, STOP and ask the user to commit or stash them first.

### Step 3: Update Cargo.toml

Read `Cargo.toml` and update the version field:

```toml
[package]
name = "hockey"
version = "NEW_VERSION_HERE"
```

Use the Edit tool to replace the old version with the new version.

### Step 4: Update CHANGELOG.md

Read `CHANGELOG.md` and perform these updates:

1. **Move unreleased changes** from `## [Unreleased]` to `## [NEW_VERSION] - YYYY-MM-DD`
   - Use today's date in ISO format (YYYY-MM-DD)
   - Keep all sections: Added, Changed, Deprecated, Removed, Fixed, Security

2. **Create fresh Unreleased section**:
   ```markdown
   ## [Unreleased]

   ### Added

   ### Changed

   ### Fixed
   ```

3. **Update version links** at the bottom of the file:
   - Update `[unreleased]` compare link to use new version
   - Add new version release link

**Example transformation:**

Before:
```markdown
## [Unreleased]

### Added
- Maximum value validation for player event statistics (#147)

### Fixed
- Player event statistics saved reliably (#146)

## [0.1.7] - 2025-12-31

[unreleased]: https://github.com/josefjura/hockey/compare/v0.1.7...HEAD
[0.1.7]: https://github.com/josefjura/hockey/releases/tag/v0.1.7
```

After (assuming new version 0.1.8):
```markdown
## [Unreleased]

### Added

### Changed

### Fixed

## [0.1.8] - 2025-12-31

### Added
- Maximum value validation for player event statistics (#147)

### Fixed
- Player event statistics saved reliably (#146)

## [0.1.7] - 2025-12-31

[unreleased]: https://github.com/josefjura/hockey/compare/v0.1.8...HEAD
[0.1.8]: https://github.com/josefjura/hockey/releases/tag/v0.1.8
[0.1.7]: https://github.com/josefjura/hockey/releases/tag/v0.1.7
```

### Step 5: Run Pre-commit Checks

**CRITICAL**: Always run pre-commit checks before creating the release:

```bash
make precommit
```

This will:
- Format code with `cargo fmt`
- Run linter with `cargo clippy`
- Run all tests with `cargo test`

If any checks fail:
- STOP the process
- Show the errors to the user
- Ask them to fix the issues
- Do NOT proceed with commit/tag/push until checks pass

### Step 6: Create Git Commit

Add the modified files and create a commit:

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: release v{VERSION}"
```

**Note**: The commit message format is `chore: release v{VERSION}` (e.g., "chore: release v1.2.3")

### Step 7: Create Git Tag

Create an annotated tag for the release:

```bash
git tag -a v{VERSION} -m "Release version {VERSION}"
```

**Note**: Tags must be prefixed with 'v' (e.g., v1.2.3, not 1.2.3)

### Step 8: Push Commit and Tag

Push both the commit and the tag to the remote repository:

```bash
git push origin master && git push origin v{VERSION}
```

Or alternatively:

```bash
git push origin master --follow-tags
```

### Step 9: Verify Build Action Started

Use GitHub CLI to check that the CI/CD workflow started:

```bash
gh run list --limit 3
```

Look for a recent workflow run triggered by the push. Show the user the status.

Optionally, provide a link to view the run:
```bash
gh run view --web
```

### Step 10: Completion Summary

Provide a summary to the user:

✅ Version updated in Cargo.toml: v{VERSION}
✅ CHANGELOG.md updated with release notes
✅ Pre-commit checks passed
✅ Commit created: "chore: release v{VERSION}"
✅ Tag created: v{VERSION}
✅ Pushed to origin/master
✅ Build workflow started (or status)

## Example Usage

**User**: "Publish version 1.0.0"

**Skill response**:
1. Validates version format (1.0.0 is valid semver)
2. Checks git status (working tree clean)
3. Updates Cargo.toml: version = "1.0.0"
4. Updates CHANGELOG.md with release date
5. Runs `make precommit` (all checks pass)
6. Creates commit: "chore: release v1.0.0"
7. Creates tag: v1.0.0
8. Pushes to origin
9. Verifies build action started
10. Shows completion summary

**User**: "Let's publish a new patch release"

**Skill response**:
1. Reads current version from Cargo.toml (e.g., 0.1.7)
2. Suggests next patch version: 0.1.8
3. Asks user to confirm
4. Proceeds with steps above

## Validation Rules

### Version Format
- Must follow semantic versioning: MAJOR.MINOR.PATCH
- All parts must be non-negative integers
- Examples: ✅ 1.0.0, ✅ 0.1.7, ✅ 2.15.3
- Invalid: ❌ 1.0, ❌ v1.0.0, ❌ 1.0.0-beta

### Pre-requisites
- Working tree must be clean (no uncommitted changes)
- `make precommit` must pass all checks
- Git must be configured with user name and email
- Must be on master branch (or ask user to confirm if on different branch)

### Error Handling
- **Uncommitted changes**: Stop and ask user to commit/stash first
- **Test failures**: Stop, show errors, ask user to fix
- **Invalid version**: Ask user to provide valid semver version
- **Push failure**: Show error, ask if they want to retry
- **Build not started**: Show status, may need manual check

## Best Practices

1. **Always review CHANGELOG.md** before publishing to ensure release notes are user-facing and clear
2. **Never skip pre-commit checks** - quality is critical
3. **Test on a branch first** if unsure about the release
4. **Document breaking changes** prominently in CHANGELOG under "Changed" section
5. **Follow semantic versioning** strictly:
   - PATCH: Bug fixes, no API changes
   - MINOR: New features, backward compatible
   - MAJOR: Breaking changes
6. **Keep CHANGELOG entries user-facing** - describe what changed for users, not implementation details

## Troubleshooting

### "Working tree not clean"
- Run `git status` to see uncommitted changes
- Commit or stash changes before running publish

### "Pre-commit checks failed"
- Review the error output from `make precommit`
- Fix the issues (formatting, linting, test failures)
- Re-run the publish workflow

### "Push rejected"
- May need to pull first: `git pull origin master`
- Resolve any conflicts
- Re-run the publish workflow

### "Build action not starting"
- Check GitHub Actions status page
- Verify workflow file exists in `.github/workflows/`
- Check repository permissions for Actions

## Related Files

- `Cargo.toml` - Version number
- `CHANGELOG.md` - Release notes
- `.github/workflows/` - CI/CD workflows
- `Makefile` - Pre-commit target

## Repository Context

This skill is designed for the hockey management application repository. The project follows:
- Semantic versioning (semver.org)
- Keep a Changelog format (keepachangelog.com)
- Pre-commit validation workflow
- GitHub Actions for CI/CD
