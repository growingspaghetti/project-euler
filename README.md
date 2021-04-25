# Rust

```
cargo new project-euler --lib
touch rust/src/main.rs
```

```
cargo doc --open
```

| No | Title                      | Source code                             | Benchmarks |
|----|----------------------------|-----------------------------------------|------------|
| 1  | Multiples of 3 and 5       | [Multiples of 3 and 5](rust/src/m1.rs)       | 1.8744 us -> 1.9213 ns https://docs.google.com/presentation/d/e/2PACX-1vR7HA2byHmvIaxWXOhdLJMXgsyf7M5vx7ZWRU2ZV4qGgIOFrLaehDqONZVPYB_tLZdrCRFmUgMAagf9/pub?start=false&loop=false&delayms=60000 |
| 2  | Even Fibonacci numbers     | [Even Fibonacci numbers](rust/src/m2.rs)     | 67 ms -> 13 ns, 14 ns https://docs.google.com/presentation/d/e/2PACX-1vSaHP7TcUI71uiwljBVfn6M7yFRxgXos451Bcb_q-y0h__0CLFzDZn916HYRaj-y_hfgcS-MUbDsEh6/pub?start=false&loop=false&delayms=60000 |
| 3  | Largest prime factor       | [Largest prime factor](rust/src/m3.rs)       | 73 us -> 14.6 us https://docs.google.com/presentation/d/e/2PACX-1vSTrpjghGiCjA0R9p38sLYptZYTiHKxk_Xljri6TMzeMCjf7lyjq1WSWaIKIJxBDG_sc7dpzfyhbaqz/pub?start=false&loop=false&delayms=60000 |
| 4  | Largest palindrome product | [Largest palindrome product](rust/src/m4.rs) | 90.374 ms -> 7.9640 ms -> 62.194 us -> 3.0 us https://docs.google.com/presentation/d/e/2PACX-1vTTJxJFxMM7KkbNVNMuhPlLG4EJdhHkaJKgibWTKEeKjuzUlvoNJ-qDBOC9qDxd9FbIS4y-Zf76P5F8/pub?start=false&loop=false&delayms=60000 |
| 5  | Smallest multiple          | [Smallest multiple](rust/src/m5.rs)          | 1.7742 s -> 2.0 us -> 440 ns https://docs.google.com/presentation/d/e/2PACX-1vS0vhb4qKuLM62w4AUrn4DQfhzn9hKKS3NmOkhPK3t30mMhf7wB_KqFh5fz0vmGPZcpjP12FT7tlxZT/pub?start=false&loop=false&delayms=60000 |
| 6 | Sum square difference | [Sum square difference](rust/src/m6.rs) | 5.5783 ns -> 4.0562 ns |
| 7 | 10001st prime         | [10001st prime](rust/src/m7.rs)         | 15.028 ms -> 13.731 ms -> 3.9333 ms |
| 8 | Largest product in a series         | [Largest product in a series](rust/src/m8.rs)         | 6.4163 us, 14.486 us |
| 9 | Special Pythagorean triplet         | [Special Pythagorean triplet](rust/src/m9.rs)         | 70.057 us -> 282.46 ns |
| 10 | Summation of primes         | [Summation of primes](rust/src/m10.rs)         | 10.894 ms -> 8.0042 ms -> 6.8499 ms -> 5.6620 ms |
| 11 | Largest product in a grid         | [Largest product in a grid](rust/src/m11.rs)         | |
| 12 | Highly divisible triangular number         | [Highly divisible triangular number](rust/src/m12.rs)         | 495.54 ms -> 12.663 ms (brute force), 36.857 ms (1000 primes), 2.3902 ms (ten primes) |
| 13 | Large sum         | [Large sum](rust/src/m13.rs)         | 213.29 us -> 8.9126 us  |
| 14 | Longest Collatz sequence         | [Longest Collatz sequence](rust/src/m14.rs)         | 215.79 ms -> 95.136 ms, 343.39 ms -> 586.73 ms (hashmap, deterioration) |
| 15 | Lattice paths         | [Lattice paths](rust/src/m15.rs)         | 865.02 ns -> 1.9170 ns, 254.73 ns, 80.565 ns |
| 16 | Power digit sum         | [Power digit sum](rust/src/m16.rs)         | 284.10 us -> 55.454 us |
| 17 | Number letter counts         | [Number letter counts](rust/src/m17.rs)         |  |
| 18 | Maximum path sum I         | [Maximum path sum I](rust/src/m18.rs)         | 1.9201 us |
| 19 | Counting Sundays        | [Counting Sundays](rust/src/m19.rs)         | |
| 20 | Factorial digit sum        | [Factorial digit sum](rust/src/m20.rs)         | |
| 21 |  Amicable numbers       | [Amicable numbers](rust/src/m21.rs)         | 333.42 ms -> 233.36 ms -> 3.7326 ms |
| 22 |  Names scores       | [Names scores](rust/src/m22.rs)         | 1.0419 ms (std), 210.01 ms -> 170.67 ms -> 2.1485 ms |
| 23 |  Non-abundant sums      | [Non-abundant sums](rust/src/m23.rs)         |  |
| 24 |  Lexicographic permutations      | [Lexicographic permutations](rust/src/m24.rs)         | 39.383 ms -> 182.03 ns |
| 25 | 1000-digit Fibonacci number | [1000-digit Fibonacci number](rust/src/m25.rs) | 2.2420 ms -> 17.320 us |
| 26 | Reciprocal cycles | [Reciprocal cycles](rust/src/m26.rs) | |
| 27 | Quadratic primes | [Quadratic primes](rust/src/m27.rs) | 93.446 ms -> 21.558 ms -> 8.0667 ms |
| 28 | Number spiral diagonals | [Number spiral diagonals](rust/src/m28.rs) | 3.7760 ms -> 7.3503 us -> 2.2267 ns |
| 29 | Distinct powers | [Distinct powers](rust/src/m29.rs) | 15.813 ms -> 2.0975 ms -> 1.1515 ms |
| 30 | Digit fifth powers | [Digit fifth powers](rust/src/m30.rs) | 22.395 ms -> 10.193 ms |
| 31 | Coin sums | [Coin sums](rust/src/m31.rs) | 3.4913 ms -> 3.7455 us, 16.639 ms -> 25.304 us |
|  |  |  |  |
| 81 | Path sum: two ways | [Path sum: two ways](rust/src/m81.rs) | |

# Go

| No | Title                      | Source code                             | Benchmarks |
|----|----------------------------|-----------------------------------------|------------|
| 1  | Multiples of 3 and 5       | [Multiples of 3 and 5](go/internal/answer)       | 2.089 us -> 9.27 ns |

# TypeScript

```
deno test https://raw.githubusercontent.com/growingspaghetti/project-euler/master/typescript/m001b.ts
```

| No | Title                      | Source code                             |
|----|----------------------------|-----------------------------------------|
| 1  | Multiples of 3 and 5       | [Multiples of 3 and 5](typescript)       |

# Java

| No | Title                      | Source code                             | Benchmarks |
|----|----------------------------|-----------------------------------------|------------|
| 1  | Multiples of 3 and 5       | [Multiples of 3 and 5](java/src/main/java/com/github/growingspaghetti/) | 1.836 us -> 0.673 ns (not much meaning to measure) |
