---
description: Publish a new version with automated version bump, changelog update, commit, tag, and push
argument-hint: [version-number]
allowed-tools: Bash(cargo:*), Bash(git:*), Bash(gh:*), Read, Edit, Grep, Glob
---

# Publish Version Command

Execute the complete publishing workflow using the publish-version skill.

## Usage

If `$ARGUMENTS` is provided (e.g., `/publish 1.2.3`):
- Use the specified version number
- Validate it follows semantic versioning (MAJOR.MINOR.PATCH)

If `$ARGUMENTS` is empty (e.g., `/publish`):
- Read current version from Cargo.toml
- Ask user what the new version should be
- Suggest patch/minor/major bump options

## Workflow

Follow the publish-version skill process:

1. Validate version format and working tree status
2. Update `Cargo.toml` version
3. Update `CHANGELOG.md` - move [Unreleased] to new version with today's date
4. Run `make precommit` to ensure all checks pass
5. Create git commit: "chore: release v$ARGUMENTS"
6. Create git tag: v$ARGUMENTS
7. Push commit and tag to origin
8. Verify build action started with `gh run list`
9. Show completion summary

## Examples

```
/publish 1.2.3
```
Publishes version 1.2.3

```
/publish
```
Asks what version to publish, then proceeds

## Version Format

Must follow semantic versioning:
- MAJOR.MINOR.PATCH (e.g., 1.0.0, 0.1.7, 2.15.3)
- All parts are non-negative integers
- No 'v' prefix in argument (added automatically for git tag)

## Pre-requisites

- Clean working tree (no uncommitted changes)
- On master branch (or user confirmation if different)
- `make precommit` must pass all checks
- Git configured with user credentials

## Error Handling

- Stop if working tree has uncommitted changes
- Stop if pre-commit checks fail
- Stop if version format is invalid
- Show clear error messages and ask user how to proceed
