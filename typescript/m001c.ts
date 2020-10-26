import { assertEquals } from "https://deno.land/std@0.74.0/testing/asserts.ts";

function arithmeticSeries(first: number, last: number, n: number): number {
  const doubleOfAverage = first + last;
  return doubleOfAverage * n / 2;
}

function sumOfAllMultiplesOfThreeOrFiveArithmeticSeries(below: number): number {
  const equalOrLessThan = below - 1;
  const a3 = arithmeticSeries(
    0,
    equalOrLessThan - equalOrLessThan % 3,
    Math.floor(equalOrLessThan / 3) + 1,
  );
  const a5 = arithmeticSeries(
    0,
    equalOrLessThan - equalOrLessThan % 5,
    Math.floor(equalOrLessThan / 5) + 1,
  );
  const a15 = arithmeticSeries(
    0,
    equalOrLessThan - equalOrLessThan % 15,
    Math.floor(equalOrLessThan / 15) + 1,
  );
  return a3 + a5 - a15;
}

Deno.test("sumOfAllMultiplesOfThreeOrFiveArithmeticSeries", () => {
  assertEquals(sumOfAllMultiplesOfThreeOrFiveArithmeticSeries(1000), 233168);
});
