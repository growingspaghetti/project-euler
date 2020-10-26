import { assertEquals } from "https://deno.land/std@0.74.0/testing/asserts.ts";
import * as _ from "https://cdn.skypack.dev/lodash-es@^4.17.15";

function sumOfAllMultiplesOfThreeOrFiveLodash(below: number): number {
  return _.range(0, below, 1)
    .filter((i: number) => i % 3 == 0 || i % 5 == 0)
    .reduce((acc: number, v: number) => acc + v, 0);
}

Deno.test("sumOfAllMultiplesOfThreeOrFiveLodash", () => {
  assertEquals(sumOfAllMultiplesOfThreeOrFiveLodash(1000), 233168);
});
