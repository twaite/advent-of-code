import * as fs from "node:fs";

export function testReport(report: string) {
  const values = report.split(" ").map(Number);

  const isIncreasing = values.every((val, idx) =>
    idx + 1 < values.length ? val < values[idx + 1] : true
  );

  const isDecreasing = values.every((val, idx) =>
    idx + 1 < values.length ? val > values[idx + 1] : true
  );

  const onlyVariesByOneToThree = values.every((val, idx) => {
    if (idx + 1 < values.length) {
      const varies = Math.abs(val - values[idx + 1]);

      return varies > 0 && varies < 4;
    }

    return true;
  });

  return (isIncreasing || isDecreasing) && onlyVariesByOneToThree;
}

function main() {
  const safeReports = fs
    .readFileSync("input.txt")
    .toString()
    .split("\n")
    .reduce((acc, next) => (testReport(next) ? acc + 1 : acc), 0);

  console.log(`Number of safe reports: ${safeReports}`);
}

main();
