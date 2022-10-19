// Import types
import Graph from '../types/Graph';
import Place from '../types/Place';
import Road from '../types/Road';

const buildTownMap = (edges: Road[]) => {
  const graph: Graph<Place> = Object.create(null);

  const addEdge = (from: Place, to: Place) => {
    if (!graph[from]) {
      graph[from] = [to];
    } else {
      graph[from].push(to);
    }
  };

  edges.forEach(([from, to]) => {
    addEdge(from, to);
    addEdge(to, from);
  });

  return graph;
};

export default buildTownMap;
