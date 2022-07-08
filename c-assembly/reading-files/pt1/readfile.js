// JS code for reading a file (with Node)

// using the built-in synchoronous file reader
const { readFileSync } = require("fs");

// Turns out we *don't* need a string decoder...
// we also need to decode the bytes to strings
// const { StringDecoder } = require("string_decoder");
// 
// const bytes = readFileSync("/etc/hosts");
// const decoder = new StringDecoder("utf8");
// const text = decoder.write(bytes);

const text = readFileSync("/etc/hosts", { encoding: "utf8" });

console.log(text);
