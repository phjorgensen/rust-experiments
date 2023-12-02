import fs from "fs";

function readNumbers() {
  const filepath = process.argv[2];
  if (!filepath) return;

  const lines = fs.readFileSync(filepath).toString().split("\n");
  lines.forEach((line) => {
    const print = parseInt(line);

    if (isNaN(print)) {
      console.log("Line not a number");
    } else {
      console.log(line);
    }
  });
}

readNumbers();
