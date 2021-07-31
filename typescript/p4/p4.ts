export function assert(condition: any, msg?: string): asserts condition {
  if (!condition) {
    throw new Error(msg);
  }
}

function isPalindrome(p: number): boolean {
  let t = Number(p);
  let q = 0;
  while (t > 0) {
    q = q * 10 + t % 10;
    t = t / 10 | 0;
  }
  return p === q;
}

function scanB(a: number, lpp: number): number {
  for (let b = 999; b >= a; b--) {
    const p = a * b;
    if (p <= lpp) {
      return lpp;
    }
    if (isPalindrome(p)) {
      lpp = Math.max(lpp, p);
    }
  }
  return lpp;
}

let lpp = 0;
for (let a = 990; a >= 110; a -= 11) {
  lpp = scanB(a, lpp);
}
console.log(lpp);
assert(lpp === 906609);
