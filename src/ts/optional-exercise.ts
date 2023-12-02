function practice(list: number[], index: number): number {
  return (list[index] ?? index) * 5;
}

console.log(practice([1, 2, 3], 0));
console.log(practice([1, 2, 3], 1));
console.log(practice([1, 2, 3], 2));
console.log(practice([1, 2, 3], 3));
console.log(practice([1, 2, 3], 4));
