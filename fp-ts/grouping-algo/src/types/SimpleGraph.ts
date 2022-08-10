export class Graph<T> {
  private _adjList: Map<T, Set<T>>;

  constructor() {
    this._adjList = new Map<T, Set<T>>();
  }

  /**
   * Add a Vert to the Graph
   * @param v Vert to add
   */
  addVert(v: T) {
    this._adjList.set(v, new Set());
    return this;
  }

  /**
   * Add a new two-way edge between two verts.
   * @param v Src Vert
   * @param w Dest Vert
   */
  addEdge(v: T, w: T) {
    this._adjList?.get(v)?.add(w);
    return this;
  }
  /**
   * Clear the _adjList.
   */
  clear() {
    this._adjList = new Map<T, Set<T>>();
  }

  /**
   * Check to see if a vertex exists.
   * @param v The vertex to check for.
   */
  hasVert(v: T) {
    return this._adjList.has(v);
  }

  linked(v: T, w: T) {
    return this._adjList.get(v)?.has(w);
  }

  /**
   * Get the size, or number of vertices in the graph.
   */
  get size() {
    return this._adjList.size;
  }
}