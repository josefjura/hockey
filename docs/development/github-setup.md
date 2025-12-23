# GitHub Project Setup Complete

## Summary

✅ **6 Milestones Created** (Phases 1-6)
✅ **27 Issues Created** across all phases
✅ **Labels Created** for organization

## View Your Project

**Milestones**: https://github.com/josefjura/hockey/milestones
**All Issues**: https://github.com/josefjura/hockey/issues?q=is%3Aissue+is%3Aopen+label%3Aphase-1%2Cphase-2%2Cphase-3%2Cphase-4%2Cphase-5%2Cphase-6

## Milestones

1. **Phase 1: Foundation** (2 weeks) - 8 issues
   - Project setup, auth, layout, i18n, Teams CRUD
   - Issues: #64-71

2. **Phase 2: Core Entities** (2 weeks) - 4 issues
   - Events, Players, Seasons, Countries management
   - Issues: #72-75

3. **Phase 3: Table Components** (1 week) - 3 issues
   - Small Lit table, Large HTMX table, Pagination
   - Issues: #76-78

4. **Phase 4: Matches & Scoring** (2 weeks) - 4 issues
   - Matches list, create/edit, detail page, score events
   - Issues: #79-82

5. **Phase 5: Dashboard & Polish** (1 week) - 4 issues
   - Dashboard, UI polish, responsive design, testing
   - Issues: #83-86

6. **Phase 6: Deployment** (1 week) - 4 issues
   - Asset embedding, optimization, deployment, docs
   - Issues: #87-90

## Labels

- `phase-1` through `phase-6` - Phase tracking
- `setup` - Setup and configuration
- `auth` - Authentication
- `crud` - CRUD operations
- `ui` - UI components

## Next Steps

### 1. Start from Scratch (Recommended)

Create a new directory for the rewrite:

```bash
# Create new project
mkdir hockey-rewrite
cd hockey-rewrite
cargo init

# Copy database and migrations
cp ../hockey/backend/hockey.db ./
cp -r ../hockey/backend/migrations ./

# Keep old project as reference
# Don't try to migrate incrementally
```

### 2. Begin Phase 1

Start with issue #64:
```bash
gh issue view 64 --repo josefjura/hockey
```

Work through Phase 1 issues in order:
1. Project setup (#64)
2. Session-based auth (#65)
3. Base layout (#66)
4. i18n setup (#67)
5. Teams list (#68)
6. Teams create (#69)
7. Teams edit/delete (#70)
8. Country selector component (#71)

### 3. Track Progress

Update issues as you work:
```bash
# View issue
gh issue view 71

# Add comment
gh issue comment 71 --body "Started work on country selector"

# Close when done
gh issue close 71 --comment "Completed - component working with search and flags"
```

### 4. Create Pull Requests

For each completed feature/issue:
```bash
git checkout -b feature/teams-crud
# ... work ...
git commit -m "feat: implement teams CRUD with HTMX"
gh pr create --title "Teams CRUD implementation" --body "Closes #68, #69, #70"
```

## Project Structure (Reference)

From `../architecture/rewrite-analysis.md`:

```
hockey-rewrite/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── routes/              # Route handlers (return HTML)
│   │   ├── auth.rs
│   │   ├── teams.rs
│   │   └── ...
│   ├── views/               # Maud templates
│   │   ├── layout.rs
│   │   ├── components/
│   │   └── pages/
│   ├── business/            # Business logic
│   ├── service/             # Data access
│   ├── auth/                # Session management
│   └── i18n/                # Internationalization
├── web_components/          # Lit components
│   └── small-table.ts
├── static/                  # Static assets
│   ├── css/
│   ├── js/
│   └── flags/
├── migrations/              # SQLx migrations
└── hockey.db                # SQLite database
```

## Timeline

- **Total**: 9 weeks
- **Phase 1**: Weeks 1-2
- **Phase 2**: Weeks 3-4
- **Phase 3**: Week 5
- **Phase 4**: Weeks 6-7
- **Phase 5**: Week 8
- **Phase 6**: Week 9

## Success Criteria

From `../architecture/rewrite-analysis.md`:

- [ ] Feature parity with current application
- [ ] Single binary deployment
- [ ] Fast page loads (<500ms)
- [ ] Works without JavaScript (progressive enhancement)
- [ ] Mobile-responsive
- [ ] Czech + English i18n
- [ ] Production-ready error handling
- [ ] Comprehensive user feedback
- [ ] Accessible (keyboard navigation)
- [ ] Maintainable codebase

## Resources

- **Analysis Document**: `../architecture/rewrite-analysis.md`
- **Current Codebase**: `/home/josef/source/hockey/` (reference only)
- **Technologies**:
  - Axum: https://docs.rs/axum/
  - Maud: https://maud.lambda.xyz/
  - HTMX: https://htmx.org/
  - Lit: https://lit.dev/
  - SQLx: https://docs.rs/sqlx/
  - fluent-rs: https://docs.rs/fluent/

---

**Ready to start?** Begin with issue #64: Project Setup

```bash
gh issue view 64 --repo josefjura/hockey --web
```
