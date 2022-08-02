// General purpose graph cost function for assessing quality of grouping

import GroupGraph from '../types/GroupGraph';

/**
 * Given a weighted (undirected) graph, calculates a 'total weight' or cost.
 * Current calculation is (sum_ext_weights) - (sum_int_weights).
 * @author Benedikt Arnarsson
 */
const groupGraphCost = (g: GroupGraph): number => {
  let int_sum = 0;
  let ext_sum = 0

  // Calculating the sums
  for (const group of g.groups) {
    for (const student of group) {
      for
    }
  }
  
  return (ext_sum - int_sum)/2.0;
};

export default groupGraphCost;