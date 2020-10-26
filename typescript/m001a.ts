import { assertEquals } from "https://deno.land/std@0.74.0/testing/asserts.ts";

function sumOfAllMultiplesOfThreeOrFive(below: number): number {
  let sum: number = 0;
  for (let i = 0; i < below; i++) {
    if (i % 3 == 0 || i % 5 == 0) {
      sum += i;
    }
  }
  return sum;
}

Deno.test("sumOfAllMultiplesOfThreeOrFive", () => {
  assertEquals(sumOfAllMultiplesOfThreeOrFive(1000), 233168);
});
