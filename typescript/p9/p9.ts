export function assert(condition: any, msg?: string): asserts condition {
  if (!condition) {
    throw new Error(msg);
  }
}

let ans = 0;
exploration:
for (let m = 2; m <= (Math.sqrt(499) | 0); m++) {
  for (let n = 1; n < m; n++) {
    const a = m * m - n * n;
    const b = 2 * m * n;
    const c = m * m + n * n;
    if (a + b + c === 1000) {
      ans = a * b * c;
      break exploration;
    }
  }
}
console.log(ans);
assert(ans === 31875000);
