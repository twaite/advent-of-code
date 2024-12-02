import { assertEquals } from "jsr:@std/assert";
import { testReport } from "./main.ts";

const testCases = [
  {
    input: "7 6 4 2 1",
    expected: true,
  },
  {
    input: "1 2 7 8 9",
    expected: false,
  },
  {
    input: "9 7 6 2 1",
    expected: false,
  },
  {
    input: "1 3 2 4 5",
    expected: false,
  },
  {
    input: "8 6 4 4 1",
    expected: false,
  },
  {
    input: "1 3 6 7 9",
    expected: true,
  },
];

testCases.forEach(({ input, expected }) => {
  Deno.test(
    `Report ${input} should be ${expected ? "Safe" : "Not Safe"}`,
    () => {
      assertEquals(testReport(input), expected);
    }
  );
});
