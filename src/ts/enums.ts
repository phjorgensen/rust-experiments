enum Colour {
  Red,
  Green,
  Blue,
  Yellow,
}

function printColour(color: Colour) {
  switch (color) {
    case Colour.Red:
      console.log("red");
      break;
    case Colour.Green:
      console.log("green");
      break;
    case Colour.Blue:
      console.log("blue");
      break;
  }
}

printColour(Colour.Red);
printColour(Colour.Green);
printColour(Colour.Blue);
printColour(Colour.Yellow);
