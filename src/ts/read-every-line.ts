import fs from "fs";

function readEveryLine() {
  console.log("read every line");
  fs.readFileSync("lines")
    .toString()
    .split("\n")
    .forEach((line) => console.log(line));
  console.log("");
}

readEveryLine();
