import fs from "fs";

function readEveryOtherLineSkip() {
  console.log("read every other line skip the first two and the last");
  fs.readFileSync("lines")
    .toString()
    .split("\n")
    .filter((_, i) => i % 2 === 0)
    .filter((_, i) => i > 1 && i < 4)
    .forEach((line) => console.log(line));
  console.log("");
}

readEveryOtherLineSkip();
