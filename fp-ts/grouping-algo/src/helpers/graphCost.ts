// General purpose graph cost function for assessing quality of grouping

import GroupGraph, { StudentGraph } from '../types/GroupGraph';

/**
 * Calculates the two times the sum of the weights between the groups of a GroupGraph.
 * @author Benedikt Arnarsson
 * @param g The GroupGraph for which we are calculating the 'exterior' weight
 * @returns Two times the sum of the weights between the groups (via the GroupGraph.matrix)
 */
const groupGraphCost = (g: GroupGraph): number => {
  let cost = 0;

  // Calculating the sums
  g.matrix.forEach((row) => {
    row.forEach((weight) => {
      cost += weight;
    })
  })
  
  // WARNING: this cost is NOT the sum of the weights, but two times the sum of the weights
  return cost;
};

const studentGraphCost = (g: StudentGraph): number => {
  let cost = 0;

  for (const student of g.students.values()) {
    for (const weight of student.relations.values()) {
      cost += weight;
    }
  }

  return cost/2.0;
}

const graphCost = (g: GroupGraph): number => {
  return groupGraphCost(g) - studentGraphCost(g.studentGraph);
}

export default graphCost;