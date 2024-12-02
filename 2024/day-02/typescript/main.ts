import * as fs from "node:fs";

export function testReport(report: string) {
  const values = report.split(" ").map(Number);

  const isIncreasing = values[1] < values[2];

  return values.every((val, idx, arr) => {
    if (idx + 1 === values.length) {
      return true;
    }

    const variance = Math.abs(val - arr[idx + 1]);

    const isMonotonic = isIncreasing ? val < arr[idx + 1] : val > arr[idx + 1];
    const varianceIsWithinTolerance = variance < 4 && variance > 0;

    return isMonotonic && varianceIsWithinTolerance;
  });
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
