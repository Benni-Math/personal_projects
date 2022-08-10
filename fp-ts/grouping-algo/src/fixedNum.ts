// Basic algorithm (pseudo-code) for fixed group number (showcase?)
//  1. Banned pairs
//  2. Pre-assignment
// Stronger balanced group enforcement
// Taking attendance probability into account?
// Minimum fill?

import { Graph } from './types/SimpleGraph';

const GROUP_NUM = 3;

const students = [
  'Bob',
  'Kim',
  'Violet',
  'Trixie',
  'Katya',
  'Ross',
  'Eric',
  'Ben',
  'Gabe',
  'Jackson',
  'Yuen Ler',
];

// Creating banGraph and adding students
let banGraph = new Graph<string>();
for (const student of students) {
  banGraph.addVert(student);
}

// Adding the banned pairs
banGraph.addEdge('Trixie', 'Katya');
banGraph.addEdge('Kim', 'Bob');
banGraph.addEdge('Violet', 'Ben');
banGraph.addEdge('Violet', 'Gabe');
banGraph.addEdge('Violet', 'Jackson');
banGraph.addEdge('Violet', 'Yuen Ler');

// Initiating groups
let groups: string[][] = [[], []];

// Pre-assignment?
// What if they don't show up?

// No negatives in the cost function -- reframe negatives