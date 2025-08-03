export interface Match {
	id: string;
	season_id: string;
	home_team_id: string;
	away_team_id: string;
	home_score_unidentified: number;
	away_score_unidentified: number;
	match_date: string | null;
	status: string;
	venue: string | null;
	
	// Joined fields for display
	season_name: string;
	home_team_name: string;
	away_team_name: string;
}

export interface ScoreEvent {
	id: string;
	match_id: string;
	team_id: string;
	scorer_id: string | null;
	assist1_id: string | null;
	assist2_id: string | null;
	period: number | null;
	time_minutes: number | null;
	time_seconds: number | null;
	goal_type: string | null;
	
	// Joined fields for display
	scorer_name: string | null;
	assist1_name: string | null;
	assist2_name: string | null;
}

export interface MatchWithStats {
	match_info: Match;
	home_total_score: number;
	away_total_score: number;
	home_detailed_goals: number;
	away_detailed_goals: number;
}

export interface CreateMatchRequest {
	season_id: string;
	home_team_id: string;
	away_team_id: string;
	home_score_unidentified?: number;
	away_score_unidentified?: number;
	match_date?: string;
	status?: string;
	venue?: string;
}

export interface UpdateMatchRequest {
	season_id?: string;
	home_team_id?: string;
	away_team_id?: string;
	home_score_unidentified?: number;
	away_score_unidentified?: number;
	match_date?: string;
	status?: string;
	venue?: string;
}

export interface CreateScoreEventRequest {
	team_id: string;
	scorer_id?: string;
	assist1_id?: string;
	assist2_id?: string;
	period?: number;
	time_minutes?: number;
	time_seconds?: number;
	goal_type?: string;
}