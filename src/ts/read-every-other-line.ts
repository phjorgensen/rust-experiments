import fs from "fs";

function readEveryOtherLine() {
  fs.readFileSync("lines")
    .toString()
    .split("\n")
    .filter((_, i) => i % 2 === 0)
    .forEach((line) => console.log(line));
}

readEveryOtherLine();
