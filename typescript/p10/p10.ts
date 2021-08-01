export function assert(condition: any, msg?: string): asserts condition {
  if (!condition) {
    throw new Error(msg);
  }
}

class Index {
  i: number;
  #f: number;
  constructor() {
    this.i = 5;
    this.#f = 0;
  }
  increment() {
    this.i += 2 << this.#f;
    this.#f ^= 1;
  }
}

function ruleout(sieve: boolean[], p: number) {
  for (let i = p * p; i < sieve.length; i += p) {
    sieve[i] = false;
  }
}

const n = 2_000_000;
let sieve = Array(n + 1).fill(true);
const side = Math.sqrt(n) | 0;
let sum = 2 + 3;
const i = new Index();
while (i.i <= side) {
  if (sieve[i.i]) {
    sum += i.i;
    ruleout(sieve, i.i);
  }
  i.increment();
}
while (i.i < sieve.length) {
  if (sieve[i.i]) {
    sum += i.i;
  }
  i.increment();
}
console.log(sum);
assert(sum === 142913828922);
