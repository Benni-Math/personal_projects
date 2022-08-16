import isValidPoint2D from './isValidPoint2D';
/**
 * Given a 2D boolean array and a point, calculates the number of neighbors which are true.
 * Checks all **8** neighbors, meaning diagonal neighbors as well.
 * Will return -1 if (x, y) is not a valid point
 * NOTE: assumes that arr is a matrix, so maintain a uniform length of subarrays.
 * @author Benedikt Arnarsson
 * @param arr 2D boolean array to be checked
 * @param x the x-axis position of the point we are checking a.k.a. column number
 * @param y the y-axis position of the point we are checking a.k.a. row number
 * @returns the sum of the number of true neighbors
 */
const numNeighbors2D = (arr: boolean[][], x: number, y: number): number => {
  // Checking whether (x, y) is a valid point in arr
  if (isValidPoint2D(arr, x, y)) { return -1; }

  // The direction of neighbors
  const DIRS = [
    [0, 1],
    [1, 1],
    [1, 0],
    [1, -1],
    [0, -1],
    [-1, -1],
    [-1, 0],
    [-1, 1],
  ];

  let sum = 0;

  for (const dir of DIRS) {
    const xPosition = x + dir[0];
    const yPosition = y + dir[1];

    if (isValidPoint2D(arr, xPosition, yPosition) && arr[yPosition][xPosition]) {
      sum += 1;
    }
  }

  return sum;
};

export default numNeighbors2D;
