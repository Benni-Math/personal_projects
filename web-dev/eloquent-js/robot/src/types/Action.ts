// Import types
import Place from './Place';

/**
 * Describes the action which a robot decides to take.
 * @author Benedikt Arnarsson
 */
type Action = {
  direction: Place,
  memory?: any,
};

export default Action;
