// Import constants
import TOWN_MAP from '../constants/TOWN_MAP';

// Import types
import Place from '../types/Place';
import TownState from '../types/TownState';

/**
 * Function for moving our robot (via updating the TownState).
 */
const move = (townState: TownState, destination: Place): TownState => {
  // Destructure
  const {
    place,
    parcels,
  } = townState;

  if (!TOWN_MAP[place].includes(destination)) {
    return townState;
  }

  const newParcels = parcels
    .map((p) => {
      // Only move parcels that the robot holds
      if (p.place !== place) return p;
      return { place: destination, address: p.address };
    })
    .filter((p) => {
      // Drop off parcels that have reached their destination
      return p.place !== p.address;
    });

  // Create our updated TownState
  const newTownState: TownState = {
    place: destination,
    parcels: newParcels,
  };

  return newTownState;
};

export default move;
