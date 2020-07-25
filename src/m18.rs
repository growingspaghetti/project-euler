//! ```txt
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- the_maximum_total_from_top_to_bottom_of_the_triangle --verbose
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!
//!     Finished bench [optimized] target(s) in 13.61s
//!      Running target/release/deps/bench_main-6ef44f4b2c4834b9
//! Benchmarking the_maximum_total_from_top_to_bottom_of_the_triangle
//! Benchmarking the_maximum_total_from_top_to_bottom_of_the_triangle: Warming up for 3.0000 s
//! Benchmarking the_maximum_total_from_top_to_bottom_of_the_triangle: Collecting 100 samples in estimated 5.0074 s (2545200 iterations)
//! Benchmarking the_maximum_total_from_top_to_bottom_of_the_triangle: Analyzing
//! the_maximum_total_from_top_to_bottom_of_the_triangle
//!                         time:   [1.9088 us 1.9201 us 1.9329 us]
//! Found 5 outliers among 100 measurements (5.00%)
//!   5 (5.00%) high severe
//! slope  [1.9088 us 1.9329 us] R^2            [0.8267383 0.8257659]
//! mean   [1.9285 us 2.0155 us] std. dev.      [78.221 ns 354.30 ns]
//! median [1.8974 us 1.9349 us] med. abs. dev. [34.809 ns 66.414 ns]
//!
//! Benchmarking the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up
//! Benchmarking the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up: Warming up for 3.0000 s
//! Benchmarking the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up: Collecting 100 samples in estimated 5.0076 s (2232100 iterations)
//! Benchmarking the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up: Analyzing
//! the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up
//!                         time:   [2.2277 us 2.2361 us 2.2459 us]
//! Found 11 outliers among 100 measurements (11.00%)
//!   4 (4.00%) high mild
//!   7 (7.00%) high severe
//! slope  [2.2277 us 2.2459 us] R^2            [0.9414260 0.9407745]
//! mean   [2.2644 us 2.3919 us] std. dev.      [153.39 ns 459.05 ns]
//! median [2.2238 us 2.2355 us] med. abs. dev. [17.121 ns 39.576 ns]
//! ```

const TRIANGLE: &[&[u8]] = &[
    &[75],
    &[95, 64],
    &[17, 47, 82],
    &[18, 35, 87, 10],
    &[20, 4, 82, 47, 65],
    &[19, 1, 23, 75, 3, 34],
    &[88, 2, 77, 73, 7, 63, 67],
    &[99, 65, 4, 28, 6, 16, 70, 92],
    &[41, 41, 26, 56, 83, 40, 80, 70, 33],
    &[41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
    &[53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
    &[70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
    &[91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
    &[63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
    &[4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
];

///
/// By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.
///
/// 3
/// 7 4
/// 2 4 6
/// 8 5 9 3
///
/// That is, 3 + 7 + 4 + 9 = 23.
///
/// Find the maximum total from top to bottom of the triangle below:
///
/// 75
/// 95 64
/// 17 47 82
/// 18 35 87 10
/// 20 04 82 47 65
/// 19 01 23 75 03 34
/// 88 02 77 73 07 63 67
/// 99 65 04 28 06 16 70 92
/// 41 41 26 56 83 40 80 70 33
/// 41 48 72 33 47 32 37 16 94 29
/// 53 71 44 65 25 43 91 52 97 51 14
/// 70 11 33 28 77 73 17 78 39 68 17 57
/// 91 71 52 38 17 14 91 43 58 50 27 29 48
/// 63 66 04 68 89 53 67 30 73 16 69 87 40 31
/// 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
///
/// NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route. However, Problem 67, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever method! ;o)
///
/// ```rust
/// use self::project_euler::m18::the_maximum_total_from_top_to_bottom_of_the_triangle;
/// assert_eq!(the_maximum_total_from_top_to_bottom_of_the_triangle(), 1074);
/// ```
pub fn the_maximum_total_from_top_to_bottom_of_the_triangle() -> u64 {
    // 3
    // 7 4
    // 2 4 6s
    // 8 5 9 3

    // triangle of single values to triangle of sum
    // 3
    // 7 4
    // ->
    // 3 (init)
    // [3+7] [3+4]

    // 3      [3]
    // 7 4 as [3+7] [3+4]
    // 2 4 6
    // ->
    // [3]
    // [3+7] [3+4]
    // [3+7+2] [3+7+4]|[3+4+4] [3+4+6] # 2 "4" 6, 4 has two options. Sselect the biggest one
    // [3+7+2] [3+7+4] [6+3+4]

    // 2 4 6 as [3+7+2] [3+7+4] [6+3+4]
    // 8 5 9 3
    // ->
    // [3+7+2+8] [3+7+2+5]|[3+7+4+5] [3+7+4+9]|[6+3+4+9] [6+3+4+3]
    // [3+7+2+8] [3+7+4+5] [3+7+4+9] [6+3+4+3]
    // 20 19 23 16

    let mut sum_triangle: Vec<Vec<u64>> = vec![];
    for i in 0..TRIANGLE.len() {
        let value_line = TRIANGLE[i];
        let mut sum_line: Vec<u64> = vec![];
        if i == 0 {
            sum_line.push(value_line[0] as u64);
            sum_triangle.push(sum_line);
            continue;
        }
        for j in 0..value_line.len() {
            let value = value_line[j] as u64;
            let prev_sum_line = &sum_triangle[i - 1];
            let left_cell = if j == 0 {
                None
            } else {
                Some(prev_sum_line[j - 1])
            };
            let right_cell = if j == value_line.len() - 1 {
                None
            } else {
                Some(prev_sum_line[j])
            };
            let sum = match (left_cell, right_cell) {
                (None, Some(d)) => d + value,
                (Some(d), None) => d + value,
                (Some(l), Some(r)) => std::cmp::max(l, r) + value,
                _ => panic!(),
            };
            sum_line.push(sum);
        }
        sum_triangle.push(sum_line);
    }
    sum_triangle
        .last()
        .unwrap()
        .iter()
        .fold(0, |max, &v| max.max(v))
}

///
/// By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.
///
/// 3
/// 7 4
/// 2 4 6
/// 8 5 9 3
///
/// That is, 3 + 7 + 4 + 9 = 23.
///
/// Find the maximum total from top to bottom of the triangle below:
///
/// 75
/// 95 64
/// 17 47 82
/// 18 35 87 10
/// 20 04 82 47 65
/// 19 01 23 75 03 34
/// 88 02 77 73 07 63 67
/// 99 65 04 28 06 16 70 92
/// 41 41 26 56 83 40 80 70 33
/// 41 48 72 33 47 32 37 16 94 29
/// 53 71 44 65 25 43 91 52 97 51 14
/// 70 11 33 28 77 73 17 78 39 68 17 57
/// 91 71 52 38 17 14 91 43 58 50 27 29 48
/// 63 66 04 68 89 53 67 30 73 16 69 87 40 31
/// 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
///
/// NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route. However, Problem 67, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever method! ;o)
///
/// ```rust
/// use self::project_euler::m18::the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up;
/// assert_eq!(the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up(), 1074);
/// ```
pub fn the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up() -> u64 {
    // 3
    // 7 4
    // 2 4 6
    // 8 5 9 3

    // convert triangle of single values to triangle of sum.
    // this time, start from the bottom
    //
    // 8 5 9 3 (init)
    // 2 4 6
    // ->
    // [8+2]|[5+2] [5+4]|[9+4] [9+6]|[3+6] # Select the greatest one
    // [8+2] [9+4] [9+6]

    // 2 4 6 as [8+2] [9+4] [9+6]
    // 7 4
    // ->
    // [8+2+7]|[9+4+7] [9+4+4]|[9+6+4]
    // [9+4+7] [9+6+4]

    // 7 4 as [9+4+7] [9+6+4]
    // 3
    // ->
    // [9+4+7+3] [9+6+4+3]
    // [9+4+7+3] = 23

    let mut sum_triangle: Vec<Vec<u64>> = vec![];
    for i in (0..TRIANGLE.len()).rev() {
        let value_line = TRIANGLE[i];
        let mut sum_line: Vec<u64> = vec![];
        if i == TRIANGLE.len() - 1 {
            value_line.iter().for_each(|&v| sum_line.push(v as u64));
            sum_triangle.push(sum_line);
            continue;
        }
        for j in 0..value_line.len() {
            let value = value_line[j] as u64;
            let prev_sum_line = &sum_triangle.last().unwrap();
            let left_cell = if j == 0 { None } else { Some(prev_sum_line[j]) };
            let right_cell = if j == value_line.len() {
                None
            } else {
                Some(prev_sum_line[j + 1])
            };
            let sum = match (left_cell, right_cell) {
                (None, Some(d)) => d + value,
                (Some(d), None) => d + value,
                (Some(l), Some(r)) => std::cmp::max(l, r) + value,
                _ => panic!(),
            };
            sum_line.push(sum);
        }
        sum_triangle.push(sum_line);
    }
    //println!("{:?}", sum_triangle);
    *sum_triangle.last().unwrap().last().unwrap()
}
