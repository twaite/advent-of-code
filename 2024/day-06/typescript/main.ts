import process from "node:process";
import * as fs from "node:fs";

type Map = string[][];

export async function mapPatrolDistance(input: string): Promise<number> {
  const map: Map = input.split(`\n`).map((line) => line.split(""));

  const rowLen = map[0].length + 1;
  const startingIdx = input
    .split("")
    .findIndex((c) => ["^", ">", "<", "v"].includes(c));

  let location = [startingIdx % rowLen, Math.floor(startingIdx / rowLen)];
  let iter = 0;

  move: while (true) {
    iter++;

    console.log(map.map((row) => row.join("")).join("\n"), "\n\n");

    if (iter > 0) {
      process.stdout.write(`\u001b[${map.length + 2}A`);
    }

    await sleep(500);

    // Move
    let [x, y] = location;
    const direction = map[y][x];

    map[y][x] = "X"; // Flag as visited

    switch (direction) {
      case "^":
        location = [x, y - 1];
        break;
      case ">":
        location = [x + 1, y];
        break;
      case "v":
        location = [x, y + 1];
        break;
      case "<":
        location = [x - 1, y];
        break;
      default:
        throw new Error(`Didn\'t match a guard position: ${direction}`);
    }

    // Is next location valid?
    [x, y] = location;
    if (
      // x axis is valid
      x >= 0 &&
      x < rowLen - 1 &&
      // y axis is valid
      y >= 0 &&
      y < map.length - 1
    ) {
      map[location[1]][location[0]] = direction;
    } else {
      break move;
    }
  }

  return map.flatMap((row) => row).filter((loc) => loc === "X").length;
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function main() {
  const input = fs.readFileSync("input.txt").toString();
}

main();
