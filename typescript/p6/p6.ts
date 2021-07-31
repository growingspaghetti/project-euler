export function assert(condition: any, msg?: string): asserts condition {
  if (!condition) {
    throw new Error(msg);
  }
}

interface Sequence {
  n: number;
}

function sumOfSquares(s: Sequence): number {
  return s.n * (s.n + 1) * (2 * s.n + 1) / 6;
}

function sum(s: Sequence): number {
  return (1 + s.n) * s.n / 2;
}

const s: Sequence = { n: 100 };
const su = sum(s);
const diff = su * su - sumOfSquares(s);
console.log(diff);
assert(diff === 25164150);
