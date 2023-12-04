import fs from "fs";
import { exit } from "process";
import { sumPartNumbers } from "./parser";

function main() {
  // read filename from args

  const filename = process.argv[2];

  if (!filename) {
    console.error("Please provide a filename as an argument");
    exit(1);
  }

  const input = fs.readFileSync(filename, "utf8");
  const total = sumPartNumbers(input);

  console.log(`The total is ${total}`);
}

main();
