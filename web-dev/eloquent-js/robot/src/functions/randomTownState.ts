// Import constants
import PLACES from '../constants/PLACES';

// Import types
import Parcel from '../types/Parcel';
import TownState from '../types/TownState';

// Import functions
import randomPick from './randomPick';

/**
 * Creates a random TownState.
 * @author Benedikt Arnarsson
 * @param parcelCount number of parcels to be generated.
 * @returns a randomly generated TownState
 */
const randomTownState = (parcelCount: number): TownState => {
  const parcels: Parcel[] = [];

  for (let i = 0; i < parcelCount; i++) {
    const address = randomPick(PLACES);
    let place = randomPick(PLACES);
    while (place === address) {
      place = randomPick(PLACES);
    }
    parcels.push({ place, address });
  }

  const place = 'Post Office';

  const newTownState: TownState = {
    place,
    parcels,
  };

  return newTownState;
};

export default randomTownState;
