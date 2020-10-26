package com.github.growingspaghetti;

import org.junit.Assert;
import org.junit.Test;
import org.openjdk.jmh.annotations.Mode;
import org.openjdk.jmh.runner.Runner;
import org.openjdk.jmh.runner.RunnerException;
import org.openjdk.jmh.runner.options.Options;
import org.openjdk.jmh.runner.options.OptionsBuilder;

import java.util.concurrent.TimeUnit;

public class M001 {

  @Test
  public void testSumOfAllMultiplesOfThreeOrFive() {
    Assert.assertEquals("sum of all multiples of 3 and 5 under 1000 is 233168",
        new M001a().sumOfAllMultiplesOfThreeOrFive(1000), 233168);
  }

  // M001a.bench  avgt   20  1836.392 ± 14.222  ns/op
  @Test
  public void benchA() throws RunnerException {
    Options opt = new OptionsBuilder()
        .include(M001a.class.getSimpleName() + ".bench")
        .warmupIterations(15)
        .forks(1)
        .mode(Mode.AverageTime)
        .timeUnit(TimeUnit.NANOSECONDS)
        .build();
    new Runner(opt).run();
  }

  @Test
  public void testSumOfAllMultiplesOfThreeOrFiveArithmeticSeries() {
    Assert.assertEquals("sum of all multiples of 3 and 5 under 1000 is 233168",
        new M001b().sumOfAllMultiplesOfThreeOrFiveArithmeticSeries(1000), 233168);
  }

  // M001b.bench  avgt   20  0.673 ± 0.005  ns/op
  @Test
  public void benchB() throws RunnerException {
    Options opt = new OptionsBuilder()
        .include(M001b.class.getSimpleName() + ".bench")
        .warmupIterations(15)
        .forks(1)
        .mode(Mode.AverageTime)
        .timeUnit(TimeUnit.NANOSECONDS)
        .build();
    new Runner(opt).run();
  }
}
