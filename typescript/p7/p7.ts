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

function ruleout_from_position(
  sieve: boolean[],
  prime: number,
  position: number,
) {
  const sq = prime * prime;
  const head = (((position - 1) / prime | 0) + 1) * prime;
  for (let i = Math.max(sq, head); i < sieve.length; i += prime) {
    sieve[i] = false;
  }
}

function ruleout(sieve: boolean[], p: number) {
  for (let i = p * p; i < sieve.length; i += p) {
    sieve[i] = false;
  }
}

function extend(sieve: boolean[], primes: number[]) {
  const previousLen = sieve.length;
  sieve.push(...Array(previousLen).fill(true));
  for (const p of primes) {
    ruleout_from_position(sieve, p, previousLen);
  }
}

let counter = 2;
let sieve = Array(10000).fill(true);
let primes = [];
const i = new Index();
exploration:
while (true) {
  while (i.i < sieve.length) {
    if (!sieve[i.i]) {
      i.increment();
      continue;
    }
    counter++;
    if (counter === 10001) {
      break exploration;
    }
    primes.push(i.i);
    ruleout(sieve, i.i);
    i.increment();
  }
  extend(sieve, primes);
}
console.log(i.i);
assert(i.i === 104743);
