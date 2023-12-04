import fs from "fs";
export function sumPartNumbers(input: string): number {
  fs.writeFileSync(
    "output.txt",
    input
      .split("\n")
      .map(getValidNumbersFromLine)
      .flatMap((l) => l.reduce((a, n) => a + n, 0))
      .join("\n")
  );

  return input
    .split("\n")
    .flatMap(getValidNumbersFromLine)
    .reduce((acc, num) => acc + num, 0);
}

export function getValidNumbersFromLine(
  line: string,
  lineIndex: number,
  lines: string[]
): number[] {
  const regex = /\d+/g;
  const matches = line.match(regex) || [];
  let offset = 0;

  return matches.map((match) => {
    offset = line.indexOf(match, offset);
    let shouldParse = hasSurroundingSymbols(lines, lineIndex, match, offset);
    return shouldParse ? parseInt(match) : 0;
  });
}

function hasSurroundingSymbols(
  lines: string[],
  lineIndex: number,
  match: string,
  matchIdx: number
): boolean {
  /* Lines */
  const prevLine = lines[lineIndex - 1] || "";
  const nextLine = lines[lineIndex + 1] || "";
  const currentLine = lines[lineIndex];

  console.log(`${prevLine}\n${currentLine}\n${nextLine}`);

  /* Indexes */
  const startIdx = Math.max(0, matchIdx - 1);
  const endIdx = matchIdx + match.length;

  const surroundingChars = [
    currentLine[startIdx], // left
    prevLine.substring(startIdx, endIdx + 1), // above
    currentLine[endIdx], // right
    nextLine.substring(startIdx, endIdx + 1), // below
  ];

  console.log([
    `currentLine[${startIdx}]`,
    `prevLine.substring(${startIdx}, ${endIdx + 1})`,
    `currentLine[${endIdx}]`,
    `nextLine.substring(${startIdx}, ${endIdx + 1})`,
  ]);

  console.log({
    match,
    chars: surroundingChars.join("|"),
    valid: /[^0-9.]/.test(surroundingChars.join("")),
  });

  return /[^0-9.]/.test(surroundingChars.join(""));
}
