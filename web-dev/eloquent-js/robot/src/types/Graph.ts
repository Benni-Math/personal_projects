/**
 * General graph type.
 * @author Benedikt Arnarsson
 */
type Graph<T extends string> = {
  [k: string]: T[],
};

export default Graph;
