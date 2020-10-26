package com.github.growingspaghetti;

import org.openjdk.jmh.annotations.Benchmark;

public class M001a {
  @Benchmark
  public void bench() {
    sumOfAllMultiplesOfThreeOrFive(1000);
  }

  public int sumOfAllMultiplesOfThreeOrFive(int below) {
    int sum = 0;
    for (int i = 0; i < below; i++) {
      if (i % 3 == 0 || i % 5 == 0) {
        sum += i;
      }
    }
    return sum;
  }
}
