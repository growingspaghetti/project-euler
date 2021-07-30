function assert(condition: any, msg?: string): asserts condition {
  if (!condition) {
    throw new Error(msg);
  }
}

function arithmeticSeries(n: number): number {
  const d = 999 / n | 0;
  return n * d * (d + 1) / 2 | 0;
}

const ans = arithmeticSeries(3) + arithmeticSeries(5) - arithmeticSeries(15);
console.log(ans);
assert(ans === 233168);
