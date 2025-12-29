/// Represents a player's scoring event with full match context
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields used in Maud templates
pub struct PlayerScoringEventEntity {
    pub score_event_id: i64,
    pub match_id: i64,
    pub match_date: Option<String>,
    pub season_id: i64,
    pub season_year: i64,
    pub season_display_name: Option<String>,
    pub event_name: String,
    pub home_team_id: i64,
    pub home_team_name: String,
    pub home_team_iso2: Option<String>,
    pub away_team_id: i64,
    pub away_team_name: String,
    pub away_team_iso2: Option<String>,
    pub team_id: i64, // Team the goal was scored for
    pub team_name: String,
    pub team_iso2: Option<String>,
    pub event_type: String, // "goal", "assist_primary", "assist_secondary"
    pub period: i32,
    pub time_minutes: Option<i32>,
    pub time_seconds: Option<i32>,
    pub goal_type: Option<String>,
    pub scorer_id: Option<i64>,
    pub scorer_name: Option<String>,
    pub assist1_id: Option<i64>,
    pub assist1_name: Option<String>,
    pub assist2_id: Option<i64>,
    pub assist2_name: Option<String>,
}

/// Filters for player scoring events
#[derive(Debug, Clone, Default)]
pub struct PlayerScoringFilters {
    pub event_type: Option<String>, // "goals", "assists", "all"
    pub season_id: Option<i64>,
    pub team_id: Option<i64>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

/// Sortable fields for player scoring events
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScoringEventSortField {
    Date,
    Event,
    EventType,
    Period,
}

impl ScoringEventSortField {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "event" => Self::Event,
            "type" => Self::EventType,
            "period" => Self::Period,
            _ => Self::Date, // default
        }
    }

    pub fn to_sql(&self) -> &'static str {
        match self {
            Self::Date => "m.match_date",
            Self::Event => "e.name",
            Self::EventType => "pe.event_type",
            Self::Period => "pe.period",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Event => "event",
            Self::EventType => "type",
            Self::Period => "period",
        }
    }
}

/// Player season statistics summary (stats for one season/event)
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields used in Maud templates
pub struct PlayerSeasonStats {
    pub season_id: i64,
    pub season_year: i64,
    pub season_display_name: Option<String>,
    pub event_id: i64,
    pub event_name: String,
    pub goals: i32,
    pub assists: i32,
    pub points: i32,
}
