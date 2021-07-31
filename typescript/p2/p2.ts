export function assert(condition: any, msg?: string): asserts condition {
  if (!condition) {
    throw new Error(msg)
  }
}

class Matrix {
  a: number;
  b: number;
  c: number;
  d: number;
  constructor() {
    this.a = 1;
    this.b = 0;
    this.c = 0;
    this.d = 1;
  }
  multiplyLiteral(o: { a: number; b: number; c: number; d: number }) {
    const a = this.a * o.a + this.b * o.c;
    const b = this.a * o.b + this.b * o.d;
    const c = this.c * o.a + this.d * o.c;
    const d = this.c * o.b + this.d * o.d;
    this.a = a;
    this.b = b;
    this.c = c;
    this.d = d;
  }
  multiplyMatrix(o: Matrix) {
    this.multiplyLiteral(o);
  }
}

function cube(): Matrix {
  let i = new Matrix();
  const fib = { a: 1, b: 1, c: 1, d: 0 };
  i.multiplyLiteral(fib);
  i.multiplyLiteral(fib);
  i.multiplyLiteral(fib);
  return i;
}

let i = new Matrix();
const c = cube();
let sum = 0;
while (i.b <= 4_000_000) {
  sum += i.b;
  i.multiplyMatrix(c);
} 
console.log(sum);
assert(sum === 4613732);
