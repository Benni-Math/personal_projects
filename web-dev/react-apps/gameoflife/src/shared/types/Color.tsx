/**
 * General Color type - loose constraints, but tight enough.
 * @author Benedikt Arnarsson
 */
type Color = RGB | RGBA | HEX;

type RGB = `rgb(${number}, ${number}, ${number})`;
type RGBA = `rgba(${number}, ${number}, ${number}, ${number})`;
type HEX = `#{string}`;

export default Color;
