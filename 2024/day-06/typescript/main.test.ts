import { describe, it } from "jsr:@std/testing/bdd";
import { assertEquals } from "jsr:@std/assert";
import { Map } from "./main.ts";

const testCases = [
  {
    input: `....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...`,
    startIdx: 64,
    start: [4, 6],
    visited: 41,
  },
  {
    input: `....
....
....
^...`,
    startIdx: 12,
    start: [0, 3],
    visited: 4,
  },
  {
    input: `...v
....
....
....`,
    startIdx: 3,
    start: [3, 0],
    visited: 4,
  },
  {
    input: `....
.>..
....
....`,
    startIdx: 5,
    start: [1, 1],
    visited: 3,
  },
  {
    input: `....
....
.<..
....`,
    startIdx: 9,
    start: [1, 2],
    visited: 2,
  },
  {
    input: `....
.>.#
....
....`,
    startIdx: 5,
    start: [1, 1],
    visited: 4,
  },
  {
    input: `....
.>.#
....
..#.`,
    startIdx: 5,
    start: [1, 1],
    visited: 5,
  },
  {
    input: `....
.>.#
#...
..#.`,
    startIdx: 5,
    start: [1, 1],
    visited: 5,
  },
  {
    input: `>..#
.#..
#...
..#.`,
    startIdx: 0,
    start: [0, 0],
    visited: 7,
  },
];

testCases.forEach(({ input, startIdx, start, visited }, idx) => {
  describe(`Case ${idx + 1}`, () => {
    it(`sets the starting index set correctly`, () => {
      assertEquals(new Map(input).startingIdx, startIdx);
    });

    it(`sets the start case set correctly`, () => {
      assertEquals(new Map(input).startingLocation, start);
    });

    it(`maps the number of visited spots correctly`, async () => {
      const map = new Map(input);
      await map.progressToEnd({ render: false });
      assertEquals(map.visited, visited);
    });
  });
});
