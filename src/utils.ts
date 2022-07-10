// this is used for better type casting
export const noop = <T>(val: T) => val;

export type PositionRange = {
  from: [line: number, col: number];
  to: [line: number, col: number];
};