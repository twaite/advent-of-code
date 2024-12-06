import process from "node:process";
import * as fs from "node:fs";

type Location = [number, number];

enum Direction {
  Up = "^",
  Right = ">",
  Down = "v",
  Left = "<",
}

const directions = Object.values(Direction).map(String);

enum Errors {
  OOB = "Out of Bounds",
  Blocked = "Blocked",
}

export class Map extends Array<Array<string>> {
  rowLen: number;
  colLen: number;
  startingIdx: number;
  startingLocation: Location;
  current: Location;
  oob = false;
  printed = false;
  currentSymbol = "X";
  visited = 1;

  constructor(input: string = "") {
    super(...input.split(`\n`).map((line) => line.split("")));

    this.rowLen = this[0].length;
    this.colLen = this.length;
    this.startingIdx = input
      .replace(/\n/g, "")
      .split("")
      .findIndex((c) => directions.includes(c));

    this.startingLocation = [
      this.startingIdx % this.rowLen,
      Math.floor(this.startingIdx / this.rowLen),
    ];

    this.current = this.startingLocation;
  }

  // Overload signatures
  get(x: number, y: number): string;
  get(location: Location): string;

  // Implementation of the method
  get(xOrLocation: number | Location, y?: number): string {
    if (Array.isArray(xOrLocation)) {
      const [x, y] = xOrLocation;
      return this[y][x];
    } else if (typeof xOrLocation === "number" && typeof y === "number") {
      return this[y][xOrLocation];
    }

    throw new Error(Errors.OOB);
  }

  set(location: Location, value: string) {
    this[location[1]][location[0]] = value;
  }

  markVisited(location: Location) {
    this.set(location, "X");
    if (this.currentSymbol !== "X") {
      this.visited++;
    }
  }

  isLocationValid(location: Location) {
    const [x, y] = location;

    return (
      // x axis is valid
      x >= 0 &&
      x < this.rowLen &&
      // y axis is valid
      y >= 0 &&
      y < this.colLen
    );
  }

  get pretty(): string {
    this.printed = true;
    return `Visited Count: ${this.visited}\n${[...this]
      .map((row) => row.join(" "))
      .join("\n")}`;
  }

  get direction() {
    return this.get(this.current);
  }

  async progressToEnd(params: { render: boolean } = { render: false }) {
    while (!this.oob) {
      if (params.render) {
        await this.render();
      }
      this.move();
    }
  }

  async render() {
    console.log(this.pretty);
    await sleep(500);
    process.stdout.write(`\u001b[${this.length + 1}A`);
    process.stdout.write(`\u001b[0J`); // Clear from cursor to the end of the screen
  }

  move() {
    switch (this.direction) {
      case Direction.Up:
        return this.moveUp();
      case Direction.Right:
        return this.moveRight();
      case Direction.Down:
        return this.moveDown();
      case Direction.Left:
        return this.moveLeft();
    }
  }

  private moveIfValid(location: Location, direction: Direction) {
    if (this.isLocationValid(location)) {
      if (this.get(location) === "#") {
        throw new Error(Errors.Blocked);
      }

      this.currentSymbol = this.get(location);
      this.markVisited(this.current);
      this.current = location;
      this.set(this.current, direction);
    } else {
      throw new Error(Errors.OOB);
    }
  }

  private tryMove(next: Location, direction: Direction, fn: () => void) {
    try {
      this.moveIfValid(next, direction);
    } catch (e) {
      if (e instanceof Error && e.message === Errors.Blocked) {
        return fn.bind(this)();
      } else {
        this.oob = true;
      }
    }
  }

  private moveUp() {
    const [x, y] = this.current;
    const up: Location = [x, y - 1];
    this.tryMove(up, Direction.Up, this.moveRight);
  }

  private moveRight() {
    const [x, y] = this.current;
    const right: Location = [x + 1, y];
    this.tryMove(right, Direction.Right, this.moveDown);
  }

  private moveDown() {
    const [x, y] = this.current;
    const down: Location = [x, y + 1];
    this.tryMove(down, Direction.Down, this.moveLeft);
  }

  private moveLeft() {
    const [x, y] = this.current;
    const left: Location = [x - 1, y];
    this.tryMove(left, Direction.Left, this.moveUp);
  }
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function main() {
  const input = fs.readFileSync("input.txt").toString();

  const map = new Map(input);
  map.progressToEnd();

  console.info(`Input has a total of ${map.visited} unique tiles visited.`);
}

main();
