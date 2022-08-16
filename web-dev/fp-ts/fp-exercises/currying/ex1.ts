import curry from '../support';

// Refactor:
// const words = str => split(' ', str);
const splitter = curry((arg: string, str: string) => str.split(arg));