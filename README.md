```
cargo new project-euler --lib
touch src/main.rs
```

```
cargo doc --open
```

| No | Title                      | Source code                             | Benchmarks |
|----|----------------------------|-----------------------------------------|------------|
| 1  | Multiples of 3 and 5       | [Multiples of 3 and 5](src/m1.rs)       | 1.8744 us -> 1.9213 ns |
| 2  | Even Fibonacci numbers     | [Even Fibonacci numbers](src/m2.rs)     | 45.805 ns -> 32.516 ns |
| 3  | Largest prime factor       | [Largest prime factor](src/m3.rs)       | 88.581 us -> 34.048 us |
| 4  | Largest palindrome product | [Largest palindrome product](src/m4.rs) | 90.374 ms -> 7.9640 ms -> 62.194 us -> 11.306 us |
| 5  | Smallest multiple          | [Smallest multiple](src/m5.rs)          | 1.7742 s -> 4.6613 us -> 570.55 ns |
| 6 | Sum square difference | [Sum square difference](src/m6.rs) | 5.5783 ns -> 4.0562 ns |
| 7 | 10001st prime         | [10001st prime](src/m7.rs)         | 15.028 ms -> 13.731 ms -> 3.9333 ms |
| 8 | Largest product in a series         | [Largest product in a series](src/m8.rs)         | 6.4163 us, 14.486 us |
| 9 | Special Pythagorean triplet         | [Special Pythagorean triplet](src/m9.rs)         | 70.057 us -> 282.46 ns |
| 10 | Summation of primes         | [Summation of primes](src/m10.rs)         | 10.894 ms -> 8.0042 ms -> 6.8499 ms -> 5.6620 ms |
| 11 | Largest product in a grid         | [Largest product in a grid](src/m11.rs)         | |
| 12 | Highly divisible triangular number         | [Highly divisible triangular number](src/m12.rs)         | 495.54 ms -> 12.663 ms (brute force), 36.857 ms (1000 primes), 2.3902 ms (ten primes) |
| 13 | Large sum         | [Large sum](src/m13.rs)         | 213.29 us -> 8.9126 us  |
| 14 | Longest Collatz sequence         | [Longest Collatz sequence](src/m14.rs)         | 215.79 ms -> 95.136 ms, 343.39 ms -> 586.73 ms (hashmap, deterioration) |
| 15 | Lattice paths         | [Lattice paths](src/m15.rs)         | 865.02 ns -> 1.9170 ns, 254.73 ns, 80.565 ns |
| 16 | Power digit sum         | [Power digit sum](src/m16.rs)         | 284.10 us -> 55.454 us |
| 17 | Number letter counts         | [Number letter counts](src/m17.rs)         |  |
