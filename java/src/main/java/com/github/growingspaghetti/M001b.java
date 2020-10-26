package com.github.growingspaghetti;

import org.openjdk.jmh.annotations.Benchmark;

public class M001b {
  @Benchmark
  public void bench() {
    sumOfAllMultiplesOfThreeOrFiveArithmeticSeries(1000);
  }

  private int arithmeticSeries(int first, int last, int times) {
    int doubleOfAverage = first + last;
    return doubleOfAverage * times / 2;
  }

  public int sumOfAllMultiplesOfThreeOrFiveArithmeticSeries(int below) {
    int equalOrLessThan = below - 1;
    int a3 = arithmeticSeries(
        0,
        equalOrLessThan - equalOrLessThan % 3,
        equalOrLessThan / 3 + 1
    );
    int a5 = arithmeticSeries(
        0,
        equalOrLessThan - equalOrLessThan % 5,
        equalOrLessThan / 5 + 1
    );
    int a15 = arithmeticSeries(
        0,
        equalOrLessThan - equalOrLessThan % 15,
        equalOrLessThan / 15 + 1
    );
    return a3 + a5 - a15;
  }
}
