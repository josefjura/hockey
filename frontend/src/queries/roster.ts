import { apiGet, apiPost, createClientApiClient } from "@/lib/api-client";
import { useMutation, useQueryClient } from '@tanstack/react-query';
import toast from 'react-hot-toast';
import { Player } from '@/types/player';
import { PaginatedResponse } from '@/types/paging';


interface Country {
	id: string;
	name: string;
	iso2_code: string;
}

// Team Participation API functions
export const findOrCreateTeamParticipation = async (seasonId: string, teamId: string, accessToken?: string): Promise<{ id: number }> => {
	const requestBody = {
		season_id: parseInt(seasonId),
		team_id: parseInt(teamId),
	};

	// Client-side: Use createClientApiClient with token
	if (accessToken) {
		const client = createClientApiClient(accessToken);
		return client<{ id: number }>('/team-participation/find-or-create', {
			method: 'POST',
			body: JSON.stringify(requestBody),
		});
	}

	// Server-side: Use apiPost (SSR/server actions)
	return apiPost<{ id: number }>('/team-participation/find-or-create', requestBody);
};

// Player Contract API functions
export const createPlayerContract = async (teamParticipationId: number, playerId: number, accessToken?: string): Promise<{ id: number }> => {
	const requestBody = {
		team_participation_id: teamParticipationId,
		player_id: playerId,
	};

	// Client-side: Use createClientApiClient with token
	if (accessToken) {
		const client = createClientApiClient(accessToken);
		return client<{ id: number }>('/player-contract', {
			method: 'POST',
			body: JSON.stringify(requestBody),
		});
	}

	// Server-side: Use apiPost (SSR/server actions)
	return apiPost<{ id: number }>('/player-contract', requestBody);
};

// Search existing players
export const searchPlayers = async (searchTerm: string, accessToken?: string): Promise<Array<{ id: number, name: string, nationality: string }>> => {
	const params = new URLSearchParams({
		page: '1',
		page_size: '10',
	});

	if (searchTerm) {
		params.append('name', searchTerm);
	}

	let data: PaginatedResponse<Player>;

	// Client-side: Use createClientApiClient with token
	if (accessToken) {
		const client = createClientApiClient(accessToken);
		data = await client<PaginatedResponse<Player>>(`/player?${params}`);
	} else {
		// Server-side: Use apiGet (SSR/prefetch)
		data = await apiGet<PaginatedResponse<Player>>(`/player?${params}`);
	}

	// Convert from paginated response to simple array for search results
	return data.items?.map((player: Player) => ({
		id: parseInt(player.id),
		name: player.name,
		nationality: player.country_name,
	})) || [];
};

// Combined function to add player to roster
export const addPlayerToRoster = async (
	seasonId: string,
	teamId: string,
	playerData: { name: string; nationality: string } | { id: number },
	accessToken?: string
): Promise<{ playerId: number; contractId: number }> => {
	let playerId: number;

	// Step 1: Create player if new, or use existing
	if ('id' in playerData) {
		playerId = playerData.id;
	} else {
		// First, find the country by name
		let countriesData: PaginatedResponse<Country>;

		if (accessToken) {
			const client = createClientApiClient(accessToken);
			countriesData = await client<PaginatedResponse<Country>>('/country?page_size=250');
		} else {
			countriesData = await apiGet<PaginatedResponse<Country>>('/country?page_size=250');
		}

		const country = countriesData.items?.find((c: Country) =>
			c.name.toLowerCase() === playerData.nationality.toLowerCase()
		);

		if (!country) {
			throw new Error(`Country "${playerData.nationality}" not found`);
		}

		// Create the player
		const newPlayerBody = {
			name: playerData.name,
			country_id: parseInt(country.id),
		};

		let newPlayer: { id: number };

		if (accessToken) {
			const client = createClientApiClient(accessToken);
			newPlayer = await client<{ id: number }>('/player', {
				method: 'POST',
				body: JSON.stringify(newPlayerBody),
			});
		} else {
			newPlayer = await apiPost<{ id: number }>('/player', newPlayerBody);
		}

		playerId = newPlayer.id;
	}

	// Step 2: Find or create team participation
	const teamParticipation = await findOrCreateTeamParticipation(seasonId, teamId, accessToken);

	// Step 3: Create player contract
	const contract = await createPlayerContract(teamParticipation.id, playerId, accessToken);

	return {
		playerId,
		contractId: contract.id,
	};
};

// Roster mutations
export const useAddPlayerToRoster = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: ({
			seasonId,
			teamId,
			playerData,
			accessToken
		}: {
			seasonId: string;
			teamId: string;
			playerData: { name: string; nationality: string } | { id: number };
			accessToken?: string;
		}) => addPlayerToRoster(seasonId, teamId, playerData, accessToken),
		onSuccess: (data, variables) => {
			// Invalidate roster queries to refetch data
			queryClient.invalidateQueries({
				queryKey: ['season', variables.seasonId, 'team', variables.teamId, 'players']
			});
			toast.success('Player added to roster successfully');
		},
		onError: (error) => {
			toast.error('Failed to add player to roster. Please try again.');
			console.error('Failed to add player to roster:', error);
		},
	});
};

export const useSearchPlayers = () => {
	return useMutation({
		mutationFn: ({ searchTerm, accessToken }: { searchTerm: string; accessToken?: string }) =>
			searchPlayers(searchTerm, accessToken),
		onError: (error) => {
			toast.error('Failed to search players. Please try again.');
			console.error('Failed to search players:', error);
		},
	});
};
