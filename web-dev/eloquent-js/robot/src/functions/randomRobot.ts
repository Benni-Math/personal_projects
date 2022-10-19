// Import type
import TOWN_MAP from '../constants/TOWN_MAP';
import Action from '../types/Action';
import Robot from '../types/Robot';

// Import function
import randomPick from './randomPick';

/**
 * A Robot which chooses random locations to go to.
 * @author Benedikt Arnarsson
 */
const randomRobot: Robot = (state) => {
  const randomAction: Action = {
    direction: randomPick(TOWN_MAP[state.place]),
  };

  return randomAction;
};

export default randomRobot;
