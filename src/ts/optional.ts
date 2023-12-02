function ensureNumber(num: number | undefined): number {
  return (num ?? 0) * 5;
}

function handleNumber(num: number | undefined): number | undefined {
  return num === undefined ? undefined : num * 5;
}

console.log(ensureNumber(0));
console.log(ensureNumber(10));
console.log(ensureNumber(undefined));

console.log(handleNumber(10));
console.log(handleNumber(0));
console.log(handleNumber(undefined));
