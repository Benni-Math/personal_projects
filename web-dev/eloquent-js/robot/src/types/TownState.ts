// Import types
import Parcel from './Parcel';
import Place from './Place';

/**
 * Main type defining the state of the town.
 * @author Benedikt Arnarsson
 */
type TownState = {
  place: Place,
  parcels: Parcel[],
};

export default TownState;
