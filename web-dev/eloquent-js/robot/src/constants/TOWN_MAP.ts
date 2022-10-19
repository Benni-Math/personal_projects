// Import constants
import ROADS from './ROADS';

// Import functions
import buildTownMap from '../functions/buildTownMap';

/**
 * The graph representing the town.
 * @author Benedikt Arnarsson
 */
const TOWN_MAP = buildTownMap(ROADS);

export default TOWN_MAP;
