#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MatchEntity {
    pub id: i64,
    pub season_id: i64,
    pub season_name: Option<String>,
    pub event_name: Option<String>,
    pub home_team_id: i64,
    pub home_team_name: String,
    pub home_team_country_iso2: Option<String>,
    pub away_team_id: i64,
    pub away_team_name: String,
    pub away_team_country_iso2: Option<String>,
    pub home_score_unidentified: i32,
    pub away_score_unidentified: i32,
    pub match_date: Option<String>,
    pub status: String,
    pub venue: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ScoreEventEntity {
    pub id: i64,
    pub match_id: i64,
    pub team_id: i64,
    pub team_name: String,
    pub scorer_id: Option<i64>,
    pub scorer_name: Option<String>,
    pub assist1_id: Option<i64>,
    pub assist1_name: Option<String>,
    pub assist2_id: Option<i64>,
    pub assist2_name: Option<String>,
    pub period: i32,
    pub time_minutes: Option<i32>,
    pub time_seconds: Option<i32>,
    pub goal_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MatchDetailEntity {
    pub match_info: MatchEntity,
    pub score_events: Vec<ScoreEventEntity>,
    pub home_score_identified: i32,
    pub away_score_identified: i32,
    pub home_score_total: i32,
    pub away_score_total: i32,
}

#[derive(Debug, Clone)]
pub struct MatchFilters {
    pub season_id: Option<i64>,
    pub team_id: Option<i64>, // matches either home or away team
    pub status: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SortField {
    Date,
    Status,
    Event,
}

impl SortField {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "status" => Self::Status,
            "event" => Self::Event,
            _ => Self::Date, // default
        }
    }

    pub fn to_sql(&self) -> &'static str {
        match self {
            Self::Date => "m.match_date",
            Self::Status => "m.status",
            Self::Event => "e.name",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Status => "status",
            Self::Event => "event",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateMatchEntity {
    pub season_id: i64,
    pub home_team_id: i64,
    pub away_team_id: i64,
    pub home_score_unidentified: i32,
    pub away_score_unidentified: i32,
    pub match_date: Option<String>,
    pub status: String,
    pub venue: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateMatchEntity {
    pub season_id: i64,
    pub home_team_id: i64,
    pub away_team_id: i64,
    pub home_score_unidentified: i32,
    pub away_score_unidentified: i32,
    pub match_date: Option<String>,
    pub status: String,
    pub venue: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateScoreEventEntity {
    pub match_id: i64,
    pub team_id: i64,
    pub scorer_id: Option<i64>,
    pub assist1_id: Option<i64>,
    pub assist2_id: Option<i64>,
    pub period: i32,
    pub time_minutes: Option<i32>,
    pub time_seconds: Option<i32>,
    pub goal_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateScoreEventEntity {
    pub team_id: i64,
    pub scorer_id: Option<i64>,
    pub assist1_id: Option<i64>,
    pub assist2_id: Option<i64>,
    pub period: i32,
    pub time_minutes: Option<i32>,
    pub time_seconds: Option<i32>,
    pub goal_type: Option<String>,
}
