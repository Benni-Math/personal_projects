/**
 * Randomly chooses an element in an array.
 * @author Benedikt Arnarsson
 * @param arr input array, with any generic elements
 * @returns random element of arr
 */
const randomPick = <T>(arr: T[]): T => {
  const choice = Math.floor(Math.random() * arr.length);
  return arr[choice];
};

export default randomPick;
