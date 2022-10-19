// Import types
import Action from './Action';
import TownState from './TownState';
/**
 * A general robot-type, which makes decisions based on state and memory.
 * @author Benedikt Arnarsson
 */
type Robot = (state: TownState, memory?: any) => Action;

export default Robot;
