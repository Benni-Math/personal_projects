import randomRobot from './functions/randomRobot';
import randomTownState from './functions/randomTownState';
import runRobot from './functions/runRobot';

const randomOpts = {
  state: randomTownState(5),
  robot: randomRobot,
};

runRobot(randomOpts);
