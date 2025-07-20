const API_URL = 'http://localhost:8080';

const teamNames = [
  'Lightning Bolts', 'Ice Dragons', 'Thunder Hawks', 'Frost Giants', 'Storm Eagles',
  'Fire Phoenix', 'Silver Wolves', 'Golden Bears', 'Steel Panthers', 'Iron Lions',
  'Crystal Sharks', 'Diamond Tigers', 'Crimson Falcons', 'Azure Leopards', 'Emerald Cobras',
  'Midnight Ravens', 'Solar Flares', 'Lunar Shadows', 'Cosmic Comets', 'Stellar Blazers',
  'Arctic Foxes', 'Desert Scorpions', 'Mountain Lions', 'Ocean Waves', 'Forest Rangers',
  'City Warriors', 'Metro Knights', 'Urban Legends', 'Capital Crushers', 'Downtown Devils',
  'Highland Chiefs', 'Lowland Lords', 'Riverside Raptors', 'Valley Vipers', 'Peak Predators',
  'Elite Force', 'Supreme Squad', 'Ultimate Team', 'Prime Players', 'Alpha Athletes',
  'Beta Brigade', 'Gamma Guards', 'Delta Dynamos', 'Epsilon Eagles', 'Zeta Zebras',
  'Omega Owls', 'Sigma Stallions', 'Theta Thunders', 'Lambda Legends', 'Kappa Kings'
];

const countryIds = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // Assuming these country IDs exist

async function createRandomTeams() {
  console.log('Starting to create 50 random teams...');
  
  for (let i = 0; i < 50; i++) {
    const randomName = teamNames[Math.floor(Math.random() * teamNames.length)];
    const randomCountryId = countryIds[Math.floor(Math.random() * countryIds.length)];
    
    // Add some variation - some teams might have null names (like national teams)
    const teamName = Math.random() > 0.1 ? randomName : null;
    
    try {
      const response = await fetch(`${API_URL}/team`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          name: teamName,
          country_id: randomCountryId,
        }),
      });
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      
      const result = await response.json();
      console.log(`✅ Created team ${i + 1}/50: ${teamName || 'National Team'} (Country: ${randomCountryId}) - ID: ${result.id}`);
      
      // Small delay to avoid overwhelming the server
      await new Promise(resolve => setTimeout(resolve, 50));
      
    } catch (error) {
      console.error(`❌ Failed to create team ${i + 1}:`, error.message);
    }
  }
  
  console.log('🎉 Finished creating teams!');
}

// Run the script
createRandomTeams().catch(console.error);