/**
 * Given a 2D array and a pair of numbers (x, y), checks whether it is a valid point.
 * NOTE: this function assumes that arr is a *matrix*, so please keep a uniform length of subarrays.
 * @author Benedikt Arnarsson
 * @param arr 2D matrix
 * @param x x-axis position aka column number 
 * @param y y-axis position aka row number 
 */
const isValidPoint2D = (arr: any[][], x: number, y: number): boolean => {
  return (x >= 0 && y >= 0 && x < arr[0].length && y < arr.length);
};

export default isValidPoint2D;
