export function assert(condition: any, msg?: string): asserts condition {
  if (!condition) {
    throw new Error(msg);
  }
}

class Divisor {
  d: number;
  #f: number;
  constructor() {
    this.d = 5;
    this.#f = 0;
  }
  increment() {
    this.d += 2 << this.#f;
    this.#f ^= 1;
  }
}

function divideFully(n: number, d: number, side: number): [number, number] {
  if (n % d != 0) {
    return [n, side];
  }
  do {
    n /= d | 0;
  } while (n % d === 0);
  return [n, Math.sqrt(n) | 0];
}

function largestPrimeFactor(n: number): number {
  assert(Number.isInteger(n));
  assert(n > 1);
  let side = Math.sqrt(n) | 0;
  for (const d of [2, 3, 5]) {
    [n, side] = divideFully(n, d, side);
    if (n === 1) {
      return d;
    }
  }
  const d = new Divisor();
  while (true) {
    d.increment();
    if (d.d > side) {
      break;
    }
    [n, side] = divideFully(n, d.d, side);
    if (n === 1) {
      return d.d;
    }
    side = Math.sqrt(n) | 0;
  }
  return n;
}

let a = largestPrimeFactor(600851475143);
console.log(a);
assert(a === 6857);
