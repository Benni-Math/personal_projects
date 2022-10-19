/* eslint-disable no-console */

// Import types
import TownState from '../types/TownState';
import Robot from '../types/Robot';

// Import functions
import move from './move';

const runRobot = (opts: {
  state: TownState,
  robot: Robot,
  memory?: any,
}) => {
  const { robot } = opts;
  let {
    state,
    memory,
  } = opts;

  state = move(state, state.place);
  for (let turn = 0; ; turn++) {
    if (state.parcels.length === 0) {
      console.log(`Done in ${turn} turns.`);
      break;
    }
    const action = robot(state, memory);
    state = move(state, action.direction);
    memory = action.memory;
    console.log(`Moved to ${action.direction}`);
  }
};

export default runRobot;
