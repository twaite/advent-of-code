import { assertEquals } from "jsr:@std/assert";
import { mapPatrolDistance } from "./main.ts";

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
    expected: 41,
  },
  {
    input: `....
....
....
^...`,
    expected: 4,
  },
  {
    input: `...v
....
....
....`,
    expected: 4,
  },
  {
    input: `....
.>..
....
....`,
    expected: 3,
  },
  {
    input: `....
....
.<..
....`,
    expected: 2,
  },
  {
    input: `....
.>.#
....
....`,
    expected: 4,
  },
  {
    input: `....
.>.#
....
..#.`,
    expected: 5,
  },
  {
    input: `....
.>.#
#...
..#.`,
    expected: 6,
  },
];

testCases.splice(0, testCases.length).forEach(({ input, expected }) => {
  Deno.test(
    `Guard patrolled the expected number of locations: ${expected}`,
    async () => {
      const distance = await mapPatrolDistance(input);
      assertEquals(distance, expected);
    }
  );
});
