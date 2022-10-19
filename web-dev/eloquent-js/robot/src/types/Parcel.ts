// Import types
import Place from './Place';

/**
 * Defines a parcel - place is current location, address is destination.
 * @author Benedikt Arnarsson
 */
type Parcel = {
  place: Place,
  address: Place,
};

export default Parcel;
