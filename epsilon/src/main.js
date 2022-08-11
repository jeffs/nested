const sum = values => [...values].reduce((acc, value) => acc + value);
const epsilon = Number.EPSILON;
const ordered = [epsilon / 3, epsilon / 2, 1];
const unordered = new Set(ordered)
console.log(sum(ordered))   // 1.0000000000000002
console.log(sum(unordered)) // 1.0
