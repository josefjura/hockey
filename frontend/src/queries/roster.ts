import { useMutation, useQueryClient } from '@tanstack/react-query';
import toast from 'react-hot-toast';
import { Player } from '@/types/player';
import { PaginatedResponse } from '@/types/paging';

const API_URL = process.env.BACKEND_URL || 'http://localhost:8080';

interface Country {
	id: string;
	name: string;
	iso2_code: string;
}

// Team Participation API functions
export const findOrCreateTeamParticipation = async (seasonId: string, teamId: string): Promise<{ id: number }> => {
	const response = await fetch(`${API_URL}/team-participation/find-or-create`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({
			season_id: parseInt(seasonId),
			team_id: parseInt(teamId),
		}),
	});
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	return response.json();
};

// Player Contract API functions
export const createPlayerContract = async (teamParticipationId: number, playerId: number): Promise<{ id: number }> => {
	const response = await fetch(`${API_URL}/player-contract`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({
			team_participation_id: teamParticipationId,
			player_id: playerId,
		}),
	});
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	return response.json();
};

// Search existing players
export const searchPlayers = async (searchTerm: string): Promise<Array<{ id: number, name: string, nationality: string }>> => {
	const params = new URLSearchParams({
		page: '1',
		page_size: '10',
	});

	if (searchTerm) {
		params.append('name', searchTerm);
	}

	const response = await fetch(`${API_URL}/player?${params}`);
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}

	const data: PaginatedResponse<Player> = await response.json();
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
	playerData: { name: string; nationality: string } | { id: number }
): Promise<{ playerId: number; contractId: number }> => {
	let playerId: number;

	// Step 1: Create player if new, or use existing
	if ('id' in playerData) {
		playerId = playerData.id;
	} else {
		// First, find the country by name
		const countriesResponse = await fetch(`${API_URL}/country?page_size=250`); // Get more countries at once
		if (!countriesResponse.ok) {
			throw new Error('Failed to fetch countries');
		}
		const countriesData: PaginatedResponse<Country> = await countriesResponse.json();
		const country = countriesData.items?.find((c: Country) =>
			c.name.toLowerCase() === playerData.nationality.toLowerCase()
		);

		if (!country) {
			throw new Error(`Country "${playerData.nationality}" not found`);
		}

		// Create the player
		const playerResponse = await fetch(`${API_URL}/player`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify({
				name: playerData.name,
				country_id: parseInt(country.id),
			}),
		});

		if (!playerResponse.ok) {
			throw new Error('Failed to create player');
		}

		const newPlayer: { id: number } = await playerResponse.json();
		playerId = newPlayer.id;
	}

	// Step 2: Find or create team participation
	const teamParticipation = await findOrCreateTeamParticipation(seasonId, teamId);

	// Step 3: Create player contract
	const contract = await createPlayerContract(teamParticipation.id, playerId);

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
			playerData
		}: {
			seasonId: string;
			teamId: string;
			playerData: { name: string; nationality: string } | { id: number };
		}) => addPlayerToRoster(seasonId, teamId, playerData),
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
		mutationFn: searchPlayers,
		onError: (error) => {
			toast.error('Failed to search players. Please try again.');
			console.error('Failed to search players:', error);
		},
	});
};
