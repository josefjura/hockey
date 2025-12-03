import { apiGet, apiPost, apiPut, apiDelete } from "@/lib/api-client";
import { Match, MatchWithStats, ScoreEvent, CreateMatchRequest, UpdateMatchRequest, CreateScoreEventRequest } from "@/types/match";
import { PaginatedResponse } from "@/types/paging";
import { queryOptions, useMutation, useQueryClient } from '@tanstack/react-query';
import toast from 'react-hot-toast';


// Match API functions
export const fetchMatchList = async (
	page: number = 0,
	filters: {
		searchTerm?: string;
		seasonId?: string;
		teamId?: string;
		status?: string;
		dateFrom?: string;
		dateTo?: string;
	} = {},
	pageSize: number = 20
): Promise<PaginatedResponse<Match>> => {
	const params = new URLSearchParams({
		page: (page + 1).toString(), // Convert from 0-based to 1-based for backend
		page_size: pageSize.toString(),
	});

	if (filters.seasonId) {
		params.append('season_id', filters.seasonId);
	}
	if (filters.teamId) {
		params.append('team_id', filters.teamId);
	}
	if (filters.status) {
		params.append('status', filters.status);
	}
	if (filters.dateFrom) {
		params.append('date_from', filters.dateFrom);
	}
	if (filters.dateTo) {
		params.append('date_to', filters.dateTo);
	}

	return apiGet<PaginatedResponse<Match>>(`/match?${params}`);
};

export const fetchMatchById = async (id: string): Promise<Match> => {
	return apiGet<Match>(`/match/${id}`);
};

export const fetchMatchWithStats = async (id: string): Promise<MatchWithStats> => {
	return apiGet<MatchWithStats>(`/match/${id}/stats`);
};

export const createMatch = async (matchData: CreateMatchRequest): Promise<{ id: number }> => {
	return apiPost<{ id: number }>('/match', {
		season_id: parseInt(matchData.season_id),
		home_team_id: parseInt(matchData.home_team_id),
		away_team_id: parseInt(matchData.away_team_id),
		home_score_unidentified: matchData.home_score_unidentified || 0,
		away_score_unidentified: matchData.away_score_unidentified || 0,
		match_date: matchData.match_date || null,
		status: matchData.status || 'scheduled',
		venue: matchData.venue || null,
	});
};

export const updateMatch = async (id: string, matchData: UpdateMatchRequest): Promise<string> => {
	const requestBody: Record<string, unknown> = {};

	if (matchData.season_id !== undefined) requestBody.season_id = parseInt(matchData.season_id);
	if (matchData.home_team_id !== undefined) requestBody.home_team_id = parseInt(matchData.home_team_id);
	if (matchData.away_team_id !== undefined) requestBody.away_team_id = parseInt(matchData.away_team_id);
	if (matchData.home_score_unidentified !== undefined) requestBody.home_score_unidentified = matchData.home_score_unidentified;
	if (matchData.away_score_unidentified !== undefined) requestBody.away_score_unidentified = matchData.away_score_unidentified;
	if (matchData.match_date !== undefined) requestBody.match_date = matchData.match_date;
	if (matchData.status !== undefined) requestBody.status = matchData.status;
	if (matchData.venue !== undefined) requestBody.venue = matchData.venue;

	return apiPut<string>(`/match/${id}`, requestBody);
};

export const deleteMatch = async (id: string): Promise<string> => {
	return apiDelete<string>(`/match/${id}`);
};

// Score Event API functions
export const fetchScoreEventsForMatch = async (matchId: string): Promise<ScoreEvent[]> => {
	return apiGet<ScoreEvent[]>(`/match/${matchId}/score-events`);
};

export const createScoreEvent = async (matchId: string, eventData: CreateScoreEventRequest): Promise<{ id: number }> => {
	return apiPost<{ id: number }>(`/match/${matchId}/score-events`, {
		team_id: parseInt(eventData.team_id),
		scorer_id: eventData.scorer_id ? parseInt(eventData.scorer_id) : null,
		assist1_id: eventData.assist1_id ? parseInt(eventData.assist1_id) : null,
		assist2_id: eventData.assist2_id ? parseInt(eventData.assist2_id) : null,
		period: eventData.period || null,
		time_minutes: eventData.time_minutes || null,
		time_seconds: eventData.time_seconds || null,
		goal_type: eventData.goal_type || null,
	});
};

export const identifyGoal = async (matchId: string, eventData: CreateScoreEventRequest): Promise<{ id: number }> => {
	return apiPost<{ id: number }>(`/match/${matchId}/identify-goal`, {
		team_id: parseInt(eventData.team_id),
		scorer_id: eventData.scorer_id ? parseInt(eventData.scorer_id) : null,
		assist1_id: eventData.assist1_id ? parseInt(eventData.assist1_id) : null,
		assist2_id: eventData.assist2_id ? parseInt(eventData.assist2_id) : null,
		period: eventData.period || null,
		time_minutes: eventData.time_minutes || null,
		time_seconds: eventData.time_seconds || null,
		goal_type: eventData.goal_type || null,
	});
};

export const deleteScoreEvent = async (matchId: string, eventId: string): Promise<string> => {
	return apiDelete<string>(`/match/${matchId}/score-events/${eventId}`);
};

export const fetchPlayersForTeamSeason = async (seasonId: string, teamId: string): Promise<Array<{ id: number, name: string, nationality: string }>> => {
	return apiGet<Array<{ id: number, name: string, nationality: string }>>(`/season/${seasonId}/team/${teamId}/players`);
};

// Query configurations
export const matchQueries = {
	list: (
		filters: {
			searchTerm?: string;
			seasonId?: string;
			teamId?: string;
			status?: string;
			dateFrom?: string;
			dateTo?: string;
		} = {},
		page: number = 0,
		pageSize: number = 20
	) =>
		queryOptions({
			queryKey: ['matches', filters, page, pageSize],
			queryFn: () => fetchMatchList(page, filters, pageSize),
			staleTime: 5 * 60 * 1000, // 5 minutes
		}),

	byId: (id: string) =>
		queryOptions({
			queryKey: ['match', id],
			queryFn: () => fetchMatchById(id),
			staleTime: 5 * 60 * 1000,
		}),

	withStats: (id: string) =>
		queryOptions({
			queryKey: ['match', id, 'stats'],
			queryFn: () => fetchMatchWithStats(id),
			staleTime: 1 * 60 * 1000, // 1 minute - stats change more frequently
		}),

	scoreEvents: (matchId: string) =>
		queryOptions({
			queryKey: ['match', matchId, 'score-events'],
			queryFn: () => fetchScoreEventsForMatch(matchId),
			staleTime: 1 * 60 * 1000, // 1 minute
		}),

	rosterPlayers: (seasonId: string, teamId: string) =>
		queryOptions({
			queryKey: ['season', seasonId, 'team', teamId, 'players'],
			queryFn: () => fetchPlayersForTeamSeason(seasonId, teamId),
			staleTime: 5 * 60 * 1000, // 5 minutes
			enabled: !!(seasonId && teamId), // Only run if both IDs are provided
		}),
};

// Match mutations
export const useCreateMatch = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: createMatch,
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ['matches'] });
			toast.success(`Match created successfully`);
		},
		onError: (error) => {
			toast.error('Failed to create match. Please try again.');
			console.error('Failed to create match:', error);
		},
	});
};

export const useUpdateMatch = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: ({ id, ...matchData }: { id: string } & UpdateMatchRequest) =>
			updateMatch(id, matchData),
		onSuccess: (data, variables) => {
			queryClient.invalidateQueries({ queryKey: ['matches'] });
			queryClient.invalidateQueries({ queryKey: ['match', variables.id] });
			toast.success(`Match updated successfully`);
		},
		onError: (error) => {
			toast.error('Failed to update match. Please try again.');
			console.error('Failed to update match:', error);
		},
	});
};

export const useDeleteMatch = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: deleteMatch,
		onSuccess: (_data, variables) => {
			queryClient.invalidateQueries({ queryKey: ['matches'] });
			queryClient.removeQueries({ queryKey: ['match', variables] });
			toast.success('Match deleted successfully');
		},
		onError: (error) => {
			toast.error('Failed to delete match. Please try again.');
			console.error('Failed to delete match:', error);
		},
	});
};

// Score Event mutations
export const useCreateScoreEvent = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: ({ matchId, ...eventData }: { matchId: string } & CreateScoreEventRequest) =>
			createScoreEvent(matchId, eventData),
		onSuccess: (data, variables) => {
			queryClient.invalidateQueries({ queryKey: ['match', variables.matchId, 'score-events'] });
			queryClient.invalidateQueries({ queryKey: ['match', variables.matchId, 'stats'] });
			toast.success('Score event created successfully');
		},
		onError: (error) => {
			toast.error('Failed to create score event. Please try again.');
			console.error('Failed to create score event:', error);
		},
	});
};

export const useIdentifyGoal = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: ({ matchId, ...eventData }: { matchId: string } & CreateScoreEventRequest) =>
			identifyGoal(matchId, eventData),
		onSuccess: (_, variables) => {
			queryClient.invalidateQueries({ queryKey: ['match', variables.matchId, 'score-events'] });
			queryClient.invalidateQueries({ queryKey: ['match', variables.matchId, 'stats'] });
			queryClient.invalidateQueries({ queryKey: ['match', variables.matchId] });
			toast.success('Goal identified successfully');
		},
		onError: (error) => {
			toast.error('Failed to identify goal. Please try again.');
			console.error('Failed to identify goal:', error);
		},
	});
};

export const useDeleteScoreEvent = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: ({ matchId, eventId }: { matchId: string; eventId: string }) =>
			deleteScoreEvent(matchId, eventId),
		onSuccess: (data, variables) => {
			queryClient.invalidateQueries({ queryKey: ['match', variables.matchId, 'score-events'] });
			queryClient.invalidateQueries({ queryKey: ['match', variables.matchId, 'stats'] });
			toast.success('Score event deleted successfully');
		},
		onError: (error) => {
			toast.error('Failed to delete score event. Please try again.');
			console.error('Failed to delete score event:', error);
		},
	});
};