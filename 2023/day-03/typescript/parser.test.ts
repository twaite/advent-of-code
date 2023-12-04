import { describe, expect, it } from "bun:test";
import { getValidNumbersFromLine, sumPartNumbers } from "./parser";

describe("sumPartNumbers", () => {
  it.each([
    [
      `467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..`,
      4361,
    ],
    [
      `................909.......180.............100.*....#...574..*............52...................................451............764.517....*...
........701*..........127*............117../...787...........67.....903..........273....*900................................=..........112..
............430............423.67.......*..............*572...........@...199.........94..........686.....679.....................921*......
`,
      6775,
    ],
    [
      `...342*468....&.....573....................*.................................%850............*..465................192.....*.......&115.....
............988.......*...731.............789.673....*256..............677..................621...........#....716......910..297........22..
204&....69............905...=.....641.472.......#.524............127....*..714.........*62..........622..324..&.................*...........`,
      2569 + 3910 + 3055,
    ],
  ])("should sum correctly", (input, expected) => {
    expect(sumPartNumbers(input)).toEqual(expected);
  });
});

describe("getValidNumbersFromLine", () => {
  it.each([
    [[".....", "12345", "....."], 1, [0]],
    [["....*", "12345", "....."], 1, [12345]],
    [[".&...", "..1..", "....."], 1, [1]], // top left
    [["..&..", "..1..", "....."], 1, [1]], // top
    [["...&.", "..1..", "....."], 1, [1]], // top right
    [[".....", "..1..", ".&..."], 1, [1]], // bottom left
    [[".....", "..1..", "..&.."], 1, [1]], // bottom
    [[".....", "..1..", "...&."], 1, [1]], // bottom right
    [[".....", "..1..", "....."], 1, [0]], // no surrounding symbols
    [[".....", ".*1..", "....."], 1, [1]], // left
    [[".....", "..1@.", "....."], 1, [1]], // right
    [[".*...", "1....", "....."], 1, [1]], // left align valid
    [[".....", "1....", "./..."], 1, [1]], // left align valid
    [[".....", "....1", ".../."], 1, [1]], // right align valid
    [[".../.", "....1", "....."], 1, [1]], // right align valid
    [
      [
        ".......12.......935............184.720...243........589.652..........435..........483.............6...........................904...........",
        "......*.....968*.....$............*........=..348...*..........986....*...................459....*........422................#......%482....",
      ],
      0,
      [12, 935, 184, 720, 243, 589, 0, 435, 0, 6, 904],
    ],

    [
      [
        "...342*468....&.....573....................*.................................%850............*..465................192.....*.......&115.....",
        "............988.......*...731.............789.673....*256..............677..................621...........#....716......910..297........22..",
        "204&....69............905...=.....641.472.......#.524............127....*..714.........*62..........622..324..&.................*...........",
      ],
      1,
      [988, 731, 789, 673, 256, 677, 621, 716, 910, 297, 0],
    ],
  ])(
    "Should return the correct value",
    (lines: string[], idx: number, expected: number[]) => {
      expect(getValidNumbersFromLine(lines[idx], idx, lines)).toEqual(expected);
    }
  );
});
