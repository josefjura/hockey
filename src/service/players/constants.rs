//! Constants for player scoring event types

/// Event type: Goal scored by the player
pub const EVENT_TYPE_GOAL: &str = "goal";

/// Event type: Primary assist (first assist on a goal)
pub const EVENT_TYPE_ASSIST_PRIMARY: &str = "assist_primary";

/// Event type: Secondary assist (second assist on a goal)
pub const EVENT_TYPE_ASSIST_SECONDARY: &str = "assist_secondary";

/// Filter value: Show only goals
pub const FILTER_EVENT_TYPE_GOALS: &str = "goals";

/// Filter value: Show only assists (both primary and secondary)
pub const FILTER_EVENT_TYPE_ASSISTS: &str = "assists";
