export function assert(condition: any, msg?: string): asserts condition {
  if (!condition) {
    throw new Error(msg);
  }
}

function gcd(a: number, b: number): number {
  assert(Number.isInteger(a) && Number.isInteger(b));
  assert(Math.sign(a) === 1 && Math.sign(b) === 1);
  let t: number;
  while (b != 0) {
    t = Number(a);
    a = b;
    b = t % b;
  }
  return a;
}

function lcm(a: number, b: number): number {
  return a * b / gcd(a, b);
}

let acc = 2;
for (let n = 3; n <= 20; n++) {
  acc = lcm(acc, n);
}
console.log(acc);
assert(acc === 232792560);
