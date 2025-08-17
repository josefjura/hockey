use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod business;
pub mod routes;
pub mod service;

/// A hockey match between two teams in a season.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Match {
    pub id: i64,
    /// The season this match belongs to.
    pub season_id: i64,
    /// The home team ID.
    pub home_team_id: i64,
    /// The away team ID.
    pub away_team_id: i64,
    /// Number of unidentified goals for the home team.
    pub home_score_unidentified: i32,
    /// Number of unidentified goals for the away team.
    pub away_score_unidentified: i32,
    /// Total goals for the home team (identified + unidentified).
    pub home_score_total: i32,
    /// Total goals for the away team (identified + unidentified).
    pub away_score_total: i32,
    /// The date and time of the match (ISO 8601 format).
    pub match_date: Option<String>,
    /// The current status of the match.
    pub status: String,
    /// The venue where the match is played.
    pub venue: Option<String>,

    // Joined fields for display
    /// The name of the season.
    pub season_name: String,
    /// The name of the home team.
    pub home_team_name: String,
    /// The name of the away team.
    pub away_team_name: String,
}

/// A detailed scoring event within a match.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScoreEvent {
    pub id: i64,
    /// The match this goal belongs to.
    pub match_id: i64,
    /// The team that scored.
    pub team_id: i64,
    /// The player who scored (can be null for unknown scorer).
    pub scorer_id: Option<i64>,
    /// The first assist (can be null).
    pub assist1_id: Option<i64>,
    /// The second assist (can be null).
    pub assist2_id: Option<i64>,
    /// The period when the goal was scored (1, 2, 3, 4=OT, 5=SO).
    pub period: Option<i32>,
    /// The minute within the period.
    pub time_minutes: Option<i32>,
    /// The seconds within the minute.
    pub time_seconds: Option<i32>,
    /// The type of goal (even_strength, power_play, etc.).
    pub goal_type: Option<String>,

    // Joined fields for display
    /// The name of the scorer.
    pub scorer_name: Option<String>,
    /// The name of the first assist.
    pub assist1_name: Option<String>,
    /// The name of the second assist.
    pub assist2_name: Option<String>,
}

/// Match statistics with calculated totals.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MatchWithStats {
    /// The match information.
    #[serde(flatten)]
    pub match_info: Match,
    /// Total home team score (unidentified + detailed events).
    pub home_total_score: i32,
    /// Total away team score (unidentified + detailed events).
    pub away_total_score: i32,
    /// Number of detailed score events for home team.
    pub home_detailed_goals: i32,
    /// Number of detailed score events for away team.
    pub away_detailed_goals: i32,
}
