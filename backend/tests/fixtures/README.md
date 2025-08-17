# SQL Test Fixtures

This directory contains SQL fixtures for integration tests. These fixtures provide clean, consistent test data setup for all entities in the hockey management system.

## 🎯 Fixture Overview

| Fixture File | Purpose | Dependencies | Entities Created |
|-------------|---------|--------------|------------------|
| `events.sql` | Basic event test data | Countries (from migrations) | 3 events |
| `players.sql` | Basic player test data | Countries (from migrations) | 3 players |
| `season_deps.sql` | Dependencies for season tests | Countries (from migrations) | 3 events |
| `seasons.sql` | Complete season test data | Countries (from migrations) | 3 events + 3 seasons |
| `match_deps.sql` | Dependencies for match tests | Countries (from migrations) | 2 events + 2 seasons + 4 teams |
| `matches.sql` | Complete match test data | Countries (from migrations) | Full hierarchy + 2 matches |
| `team_participation_deps.sql` | Dependencies for participation tests | Countries (from migrations) | 2 events + 2 seasons + 2 teams |
| `team_participations.sql` | Complete participation test data | Countries (from migrations) | Full hierarchy + 2 participations |
| `player_contract_deps.sql` | Dependencies for contract tests | Countries (from migrations) | Full hierarchy without contracts |
| `player_contracts.sql` | Complete contract test data | Countries (from migrations) | Full hierarchy + 2 contracts |

## 🏗️ Entity Hierarchy

Our fixtures follow the natural entity relationships:

```
Country (from migrations)
  ↓
Event
  ↓
Season
  ↓
Team ← → Player
  ↓       ↓
TeamParticipation
  ↓       ↓
PlayerContract
  ↓
Match
```

## 📝 Fixture Design Principles

### 1. **Reference Migration Data**
All fixtures reference existing countries (IDs 1, 2) from database migrations:
```sql
INSERT INTO event (name, country_id, ...) VALUES 
    ('Czech Hockey League', 1, ...),  -- References Czech Republic
    ('Slovak Hockey League', 2, ...); -- References Slovakia
```

### 2. **Auto-Generated IDs**
Fixtures never specify explicit IDs to avoid conflicts:
```sql
-- ✅ Good: Let auto-increment work
INSERT INTO event (name, country_id, created_at, updated_at) VALUES 
    ('World Championship', 1, '2024-01-01 12:00:00', '2024-01-01 12:00:00');

-- ❌ Bad: Hardcoded IDs cause conflicts
INSERT INTO event (id, name, country_id, ...) VALUES 
    (999, 'World Championship', 1, ...);
```

### 3. **Meaningful Test Data**
Fixture data uses descriptive names that make test assertions clear:
```sql
INSERT INTO team (name, country_id, ...) VALUES 
    ('Home Team', 1, ...),    -- Clear purpose in match tests
    ('Away Team', 1, ...);    -- Clear purpose in match tests
```

### 4. **Consistent Timestamps**
All fixtures use consistent timestamp patterns:
```sql
created_at: '2024-01-01 12:00:00'
updated_at: '2024-01-01 12:00:00'
```

## 🔧 Using Fixtures in Tests

### Basic Usage
```rust
#[sqlx::test(fixtures("entity_deps"))]
async fn test_with_dependencies(pool: SqlitePool) {
    // Fixture data is automatically loaded
    let entity_id: i64 = sqlx::query_scalar("SELECT id FROM entity LIMIT 1")
        .fetch_one(&pool).await.unwrap();
}
```

### Multiple Fixtures
```rust
#[sqlx::test(fixtures("events", "players"))]
async fn test_with_multiple_fixtures(pool: SqlitePool) {
    // Both event and player fixtures are loaded
}
```

### Dynamic ID Resolution
Always query for IDs dynamically instead of hardcoding:
```rust
// ✅ Good: Dynamic resolution
let season_id: i64 = sqlx::query_scalar("SELECT id FROM season LIMIT 1")
    .fetch_one(&pool).await.unwrap();

let team_id: i64 = sqlx::query_scalar("SELECT id FROM team WHERE name = 'Home Team'")
    .fetch_one(&pool).await.unwrap();

// ❌ Bad: Hardcoded assumptions
let season_id = 1;
let team_id = 2;
```

## 📊 Fixture Content Summary

### Core Entities
- **Countries**: 2 entries (from migrations: Czech Republic, Slovakia)
- **Events**: 3 per fixture (World Championship, European Championship, National League)
- **Seasons**: 3 per fixture (2023, 2024, 2025)
- **Teams**: 2-4 per fixture (Sparta Praha, Slovan Bratislava, Home Team, Away Team)
- **Players**: 2-3 per fixture (Jan Novak, Peter Dvorak, Test Player)

### Relationship Entities
- **TeamParticipations**: Link teams to seasons
- **PlayerContracts**: Link players to team participations
- **Matches**: Games between teams in seasons

## 🎯 Choosing the Right Fixture

| Test Scenario | Recommended Fixture | Rationale |
|---------------|-------------------|-----------|
| Creating new entities | `*_deps.sql` | Just dependencies, test creates main entity |
| Reading existing entities | `*.sql` (complete) | Full test data available |
| Testing relationships | `*.sql` (complete) | All related entities present |
| Validation tests | `*_deps.sql` | Minimal setup, focus on validation logic |

## 🔄 Adding New Fixtures

When creating new fixtures:

1. **Follow naming convention**: `entity_name.sql` or `entity_name_deps.sql`
2. **Document in this README**: Update the tables above
3. **Use meaningful data**: Names should make test purposes clear
4. **Reference existing data**: Use country IDs 1 and 2 from migrations
5. **Test thoroughly**: Ensure fixtures load without conflicts

## 🏆 Migration Status

**✅ Fully Migrated Entities:**
- PlayerContract (2 fixtures)
- TeamParticipation (2 fixtures)  
- Match (2 fixtures)
- Season (2 fixtures)

**🔄 Partial/Pending Migration:**
- Player (basic fixture exists)
- Team (can reuse existing fixtures)
- Event (basic fixture exists)

The SQL fixtures dramatically improve test maintainability and consistency across the entire test suite!
