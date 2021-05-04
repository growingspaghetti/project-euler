#[allow(clippy::zero_prefixed_literal)]
const TWENTY_GRID: [[u32; 20]; 20] = [
    [
        08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08,
    ],
    [
        49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00,
    ],
    [
        81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65,
    ],
    [
        52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91,
    ],
    [
        22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
    ],
    [
        24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
    ],
    [
        32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
    ],
    [
        67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21,
    ],
    [
        24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
    ],
    [
        21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95,
    ],
    [
        78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92,
    ],
    [
        16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57,
    ],
    [
        86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
    ],
    [
        19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40,
    ],
    [
        04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
    ],
    [
        88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
    ],
    [
        04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36,
    ],
    [
        20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16,
    ],
    [
        20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54,
    ],
    [
        01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48,
    ],
];

// 1.316 us
/// In the 20×20 grid below, four numbers along a diagonal line have been marked in red.
///
/// 08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
/// 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
/// 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
/// 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
/// 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
/// 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
/// 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
/// 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
/// 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
/// 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
/// 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
/// 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
/// 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
/// 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
/// 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
/// 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
/// 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
/// 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
/// 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
/// 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
///
/// The product of these numbers is 26 × 63 × 78 × 14 = 1788696.
///
/// What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?
///
///
/// ```rust
/// use self::project_euler::m11::greatest_product_of_four_adjacent_numbers_no_const;
/// assert_eq!(greatest_product_of_four_adjacent_numbers_no_const(), 70600674);
/// ```
pub fn greatest_product_of_four_adjacent_numbers_no_const() -> u32 {
    let mut max = 0u32;
    let matrix = [
        [
            08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08,
        ],
        [
            49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00,
        ],
        [
            81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65,
        ],
        [
            52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91,
        ],
        [
            22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
        ],
        [
            24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
        ],
        [
            32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
        ],
        [
            67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21,
        ],
        [
            24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
        ],
        [
            21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95,
        ],
        [
            78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92,
        ],
        [
            16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57,
        ],
        [
            86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
        ],
        [
            19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40,
        ],
        [
            04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
        ],
        [
            88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
        ],
        [
            04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36,
        ],
        [
            20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16,
        ],
        [
            20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54,
        ],
        [
            01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48,
        ],
    ];

    // →
    for y in matrix.iter() {
        for x in 3..matrix.len() {
            if let (Some(a), Some(b), Some(c), Some(d)) =
                (y.get(x - 3), y.get(x - 2), y.get(x - 1), y.get(x))
            {
                max = std::cmp::max(max, a * b * c * d);
            }
        }
    }

    // ↓
    for x in 0..matrix.len() {
        for y in 3..matrix.len() {
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                matrix.get(y - 3).and_then(|r| r.get(x)),
                matrix.get(y - 2).and_then(|r| r.get(x)),
                matrix.get(y - 1).and_then(|r| r.get(x)),
                matrix.get(y).and_then(|r| r.get(x)),
            ) {
                max = std::cmp::max(max, a * b * c * d);
            }
        }
    }

    // ↘ : x+ then y+
    for x in 3..matrix.len() {
        for y in 3..matrix.len() {
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                matrix.get(y - 3).and_then(|r| r.get(x - 3)),
                matrix.get(y - 2).and_then(|r| r.get(x - 2)),
                matrix.get(y - 1).and_then(|r| r.get(x - 1)),
                matrix.get(y).and_then(|r| r.get(x)),
            ) {
                max = std::cmp::max(max, a * b * c * d);
            }
        }
    }

    // ↙ : y+ then x-
    for x in 3..matrix.len() {
        for y in 0..matrix.len() - 3 {
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                matrix.get(y + 3).and_then(|r| r.get(x - 3)),
                matrix.get(y + 2).and_then(|r| r.get(x - 2)),
                matrix.get(y + 1).and_then(|r| r.get(x - 1)),
                matrix.get(y).and_then(|r| r.get(x)),
            ) {
                max = std::cmp::max(max, a * b * c * d);
            }
        }
    }

    max
}

// 2.6 us
/// In the 20×20 grid below, four numbers along a diagonal line have been marked in red.
///
/// 08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
/// 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
/// 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
/// 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
/// 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
/// 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
/// 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
/// 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
/// 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
/// 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
/// 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
/// 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
/// 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
/// 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
/// 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
/// 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
/// 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
/// 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
/// 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
/// 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
///
/// The product of these numbers is 26 × 63 × 78 × 14 = 1788696.
///
/// What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?
///
///
/// ```rust
/// use self::project_euler::m11::greatest_product_of_four_adjacent_numbers;
/// assert_eq!(greatest_product_of_four_adjacent_numbers(), 70600674);
/// ```
pub fn greatest_product_of_four_adjacent_numbers() -> u32 {
    let mut max = 0u32;

    // →
    for y in TWENTY_GRID.iter() {
        for x in 3..TWENTY_GRID.len() {
            if let (Some(a), Some(b), Some(c), Some(d)) =
                (y.get(x - 3), y.get(x - 2), y.get(x - 1), y.get(x))
            {
                let product = a * b * c * d;
                if max < product {
                    max = product;
                }
            }
        }
    }

    // ↓
    for x in 0..TWENTY_GRID.len() {
        for y in 3..TWENTY_GRID.len() {
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                TWENTY_GRID.get(y - 3).and_then(|r| r.get(x)),
                TWENTY_GRID.get(y - 2).and_then(|r| r.get(x)),
                TWENTY_GRID.get(y - 1).and_then(|r| r.get(x)),
                TWENTY_GRID.get(y).and_then(|r| r.get(x)),
            ) {
                let product = a * b * c * d;
                if max < product {
                    max = product;
                }
            }
        }
    }

    // ↘ : x+ then y+
    for x in 3..TWENTY_GRID.len() {
        for y in 3..TWENTY_GRID.len() {
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                TWENTY_GRID.get(y - 3).and_then(|r| r.get(x - 3)),
                TWENTY_GRID.get(y - 2).and_then(|r| r.get(x - 2)),
                TWENTY_GRID.get(y - 1).and_then(|r| r.get(x - 1)),
                TWENTY_GRID.get(y).and_then(|r| r.get(x)),
            ) {
                let product = a * b * c * d;
                if max < product {
                    max = product;
                }
            }
        }
    }

    // ↙ : y+ then x-
    for x in 3..TWENTY_GRID.len() {
        for y in 0..TWENTY_GRID.len() - 3 {
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                TWENTY_GRID.get(y + 3).and_then(|r| r.get(x - 3)),
                TWENTY_GRID.get(y + 2).and_then(|r| r.get(x - 2)),
                TWENTY_GRID.get(y + 1).and_then(|r| r.get(x - 1)),
                TWENTY_GRID.get(y).and_then(|r| r.get(x)),
            ) {
                let product = a * b * c * d;
                if max < product {
                    max = product;
                }
            }
        }
    }

    max
}

struct SquareVec {
    _matrix: Vec<Vec<u32>>,
    _x: usize,
    _y: usize,
}

impl SquareVec {
    fn new(matrix: Vec<Vec<u32>>) -> SquareVec {
        assert!(matrix.len() == 0 || matrix.len() == matrix[0].len());
        SquareVec {
            _x: 0,
            _y: 0,
            _matrix: matrix,
        }
    }
    fn next(&mut self) -> Option<(usize, usize)> {
        match (
            self._x >= self._matrix.len() - 1,
            self._y >= self._matrix.len() - 1,
        ) {
            (true, true) => None,
            (false, _) => {
                self._x += 1;
                Some((self._x, self._y))
            }
            (true, false) => {
                self._x = 0;
                self._y += 1;
                Some((self._x, self._y))
            }
        }
    }
    fn east_product(&self, terms: usize) -> Result<u32, ()> {
        if self._x + 1 * terms > self._matrix.len() - 1 {
            return Err(());
        }
        let p = (1..=terms)
            .map(|mult| self._matrix[self._y][self._x + 1 * mult])
            .product::<u32>();
        Ok(p)
    }
    fn south_product(&self, terms: usize) -> Result<u32, ()> {
        if self._y + 1 * terms > self._matrix.len() - 1 {
            return Err(());
        }
        let p = (1..=terms)
            .map(|mult| self._matrix[self._y + 1 * mult][self._x])
            .product::<u32>();
        Ok(p)
    }
    fn south_east_product(&self, terms: usize) -> Result<u32, ()> {
        if self._y + 1 * terms > self._matrix.len() - 1 {
            return Err(());
        }
        if self._x + 1 * terms > self._matrix.len() - 1 {
            return Err(());
        }
        let p = (1..=terms)
            .map(|mult| self._matrix[self._y + 1 * mult][self._x + 1 * mult])
            .product::<u32>();
        Ok(p)
    }
    fn south_west_product(&self, terms: usize) -> Result<u32, ()> {
        if self._y + 1 * terms > self._matrix.len() - 1 {
            return Err(());
        }
        if self._x < 1 * terms {
            return Err(());
        }
        let p = (1..=terms)
            .map(|mult| self._matrix[self._y + 1 * mult][self._x - 1 * mult])
            .product::<u32>();
        Ok(p)
    }
}

// 14 us
/// In the 20×20 grid below, four numbers along a diagonal line have been marked in red.
///
/// 08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
/// 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
/// 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
/// 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
/// 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
/// 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
/// 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
/// 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
/// 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
/// 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
/// 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
/// 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
/// 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
/// 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
/// 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
/// 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
/// 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
/// 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
/// 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
/// 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
///
/// The product of these numbers is 26 × 63 × 78 × 14 = 1788696.
///
/// What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?
///
///
/// ```rust
/// use self::project_euler::m11::greatest_product_of_four_adjacent_numbers_2;
/// assert_eq!(greatest_product_of_four_adjacent_numbers_2(), 70600674);
/// ```
pub fn greatest_product_of_four_adjacent_numbers_2() -> u32 {
    let mut square = SquareVec::new(vec![
        vec![
            08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08,
        ],
        vec![
            49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00,
        ],
        vec![
            81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65,
        ],
        vec![
            52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91,
        ],
        vec![
            22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
        ],
        vec![
            24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
        ],
        vec![
            32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
        ],
        vec![
            67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21,
        ],
        vec![
            24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
        ],
        vec![
            21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95,
        ],
        vec![
            78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92,
        ],
        vec![
            16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57,
        ],
        vec![
            86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
        ],
        vec![
            19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40,
        ],
        vec![
            04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
        ],
        vec![
            88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
        ],
        vec![
            04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36,
        ],
        vec![
            20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16,
        ],
        vec![
            20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54,
        ],
        vec![
            01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48,
        ],
    ]);

    let mut max = 0u32;
    while let Some(_) = square.next() {
        if let Ok(p) = square.east_product(4) {
            max = std::cmp::max(max, p);
        }
        if let Ok(p) = square.south_product(4) {
            max = std::cmp::max(max, p);
        }
        if let Ok(p) = square.south_east_product(4) {
            max = std::cmp::max(max, p);
        }
        if let Ok(p) = square.south_west_product(4) {
            max = std::cmp::max(max, p);
        }
    }
    max
}

struct Square {
    _matrix: [[u32; 20]; 20],
    _x: usize,
    _y: usize,
    _len: usize,
}

impl Square {
    fn new(matrix: [[u32; 20]; 20]) -> Self {
        assert!(matrix.len() == 0 || matrix.len() == matrix[0].len());
        Square {
            _x: 0,
            _y: 0,
            _matrix: matrix,
            _len: matrix.len(),
        }
    }
    fn width(&self) -> usize {
        self._len
    }
    fn next(&mut self) -> Option<(usize, usize)> {
        match (
            self._x >= self._matrix.len() - 1,
            self._y >= self._matrix.len() - 1,
        ) {
            (true, true) => None,
            (false, _) => {
                self._x += 1;
                Some((self._x, self._y))
            }
            (true, false) => {
                self._x = 0;
                self._y += 1;
                Some((self._x, self._y))
            }
        }
    }
    fn set_cursor(&mut self, x: usize, y: usize) {
        self._x = x;
        self._y = y;
    }
    fn east_product(&self, adj_len: usize) -> Result<u32, ()> {
        if self._x + adj_len > self._matrix.len() {
            return Err(());
        }
        let p = (0..adj_len)
            .map(|a| self._matrix[self._y][self._x + a])
            .product::<u32>();
        Ok(p)
    }
    fn south_product(&self, adj_len: usize) -> Result<u32, ()> {
        if self._y + adj_len > self._matrix.len() {
            return Err(());
        }
        let p = (0..adj_len)
            .map(|a| self._matrix[self._y + a][self._x])
            .product::<u32>();
        Ok(p)
    }
    fn south_east_product(&self, adj_len: usize) -> Result<u32, ()> {
        if self._y + adj_len > self._matrix.len() {
            return Err(());
        }
        if self._x + adj_len > self._matrix.len() {
            return Err(());
        }
        let p = (0..adj_len)
            .map(|a| self._matrix[self._y + a][self._x + a])
            .product::<u32>();
        Ok(p)
    }
    fn south_west_product(&self, adj_len: usize) -> Result<u32, ()> {
        if self._y + adj_len > self._matrix.len() {
            return Err(());
        }
        if self._x < adj_len - 1{
            return Err(());
        }
        let p = (0..adj_len)
            .map(|a| self._matrix[self._y + a][self._x - a])
            .product::<u32>();
        Ok(p)
    }
}

// 3.13 us
/// In the 20×20 grid below, four numbers along a diagonal line have been marked in red.
///
/// 08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
/// 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
/// 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
/// 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
/// 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
/// 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
/// 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
/// 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
/// 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
/// 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
/// 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
/// 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
/// 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
/// 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
/// 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
/// 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
/// 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
/// 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
/// 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
/// 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
///
/// The product of these numbers is 26 × 63 × 78 × 14 = 1788696.
///
/// What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?
///
///
/// ```rust
/// use self::project_euler::m11::greatest_product_of_four_adjacent_numbers_3;
/// assert_eq!(greatest_product_of_four_adjacent_numbers_3(), 70600674);
/// ```
pub fn greatest_product_of_four_adjacent_numbers_3() -> u32 {
    let mut square = Square::new([
        [
            08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08,
        ],
        [
            49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00,
        ],
        [
            81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65,
        ],
        [
            52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91,
        ],
        [
            22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
        ],
        [
            24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
        ],
        [
            32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
        ],
        [
            67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21,
        ],
        [
            24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
        ],
        [
            21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95,
        ],
        [
            78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92,
        ],
        [
            16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57,
        ],
        [
            86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
        ],
        [
            19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40,
        ],
        [
            04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
        ],
        [
            88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
        ],
        [
            04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36,
        ],
        [
            20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16,
        ],
        [
            20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54,
        ],
        [
            01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48,
        ],
    ]);

    let mut max = 0u32;
    while let Some(_) = square.next() {
        if let Ok(p) = square.east_product(4) {
            max = std::cmp::max(max, p);
        }
        if let Ok(p) = square.south_product(4) {
            max = std::cmp::max(max, p);
        }
        if let Ok(p) = square.south_east_product(4) {
            max = std::cmp::max(max, p);
        }
        if let Ok(p) = square.south_west_product(4) {
            max = std::cmp::max(max, p);
        }
    }
    max
}

// 2.10 us
pub fn greatest_product_of_four_adjacent_numbers_no_next() -> u32 {
    let mut square = Square::new([
        [
            08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08,
        ],
        [
            49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00,
        ],
        [
            81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65,
        ],
        [
            52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91,
        ],
        [
            22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
        ],
        [
            24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
        ],
        [
            32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
        ],
        [
            67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21,
        ],
        [
            24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
        ],
        [
            21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95,
        ],
        [
            78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92,
        ],
        [
            16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57,
        ],
        [
            86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
        ],
        [
            19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40,
        ],
        [
            04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
        ],
        [
            88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
        ],
        [
            04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36,
        ],
        [
            20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16,
        ],
        [
            20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54,
        ],
        [
            01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48,
        ],
    ]);

    let mut max = 0u32;
    for i in 0..square.width() {
        for j in 0..square.width() {
            square.set_cursor(i, j);
            if let Ok(p) = square.east_product(4) {
                max = std::cmp::max(max, p);
            }
            if let Ok(p) = square.south_product(4) {
                max = std::cmp::max(max, p);
            }
            if let Ok(p) = square.south_east_product(4) {
                max = std::cmp::max(max, p);
            }
            if let Ok(p) = square.south_west_product(4) {
                max = std::cmp::max(max, p);
            }
        }
    }
    max
}

// 5.58 us
/// ```rust
/// use self::project_euler::m11::greatest_product_of_four_adjacent_numbers_one_loop;
/// assert_eq!(greatest_product_of_four_adjacent_numbers_one_loop(), 70600674);
/// ```
pub fn greatest_product_of_four_adjacent_numbers_one_loop() -> u32 {
    let mut max = 0u32;

    for x in 0..TWENTY_GRID.len() {
        for y in 0..TWENTY_GRID.len() {
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                TWENTY_GRID.get(y).and_then(|r| r.get(x + 3)),
                TWENTY_GRID.get(y).and_then(|r| r.get(x + 2)),
                TWENTY_GRID.get(y).and_then(|r| r.get(x + 1)),
                TWENTY_GRID.get(y).and_then(|r| r.get(x)),
            ) {
                max = std::cmp::max(max, a * b * c * d);
            }
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                TWENTY_GRID.get(y + 3).and_then(|r| r.get(x + 3)),
                TWENTY_GRID.get(y + 2).and_then(|r| r.get(x + 2)),
                TWENTY_GRID.get(y + 1).and_then(|r| r.get(x + 1)),
                TWENTY_GRID.get(y).and_then(|r| r.get(x)),
            ) {
                max = std::cmp::max(max, a * b * c * d);
            }
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                TWENTY_GRID.get(y + 3).and_then(|r| r.get(x)),
                TWENTY_GRID.get(y + 2).and_then(|r| r.get(x)),
                TWENTY_GRID.get(y + 1).and_then(|r| r.get(x)),
                TWENTY_GRID.get(y).and_then(|r| r.get(x)),
            ) {
                max = std::cmp::max(max, a * b * c * d);
            }
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                TWENTY_GRID
                    .get(y + 3)
                    .and_then(|r| if x < 3 { None } else { r.get(x - 3) }),
                TWENTY_GRID
                    .get(y + 2)
                    .and_then(|r| if x < 2 { None } else { r.get(x - 2) }),
                TWENTY_GRID
                    .get(y + 1)
                    .and_then(|r| if x < 1 { None } else { r.get(x - 1) }),
                TWENTY_GRID.get(y).and_then(|r| r.get(x)),
            ) {
                max = std::cmp::max(max, a * b * c * d);
            }
        }
    }
    max
}
// 7.2 us
pub fn greatest_product_of_four_adjacent_numbers_no_const_vec() -> u32 {
    let mut max = 0u32;
    let matrix = vec![
        vec![
            08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08,
        ],
        vec![
            49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00,
        ],
        vec![
            81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65,
        ],
        vec![
            52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91,
        ],
        vec![
            22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
        ],
        vec![
            24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
        ],
        vec![
            32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
        ],
        vec![
            67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21,
        ],
        vec![
            24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
        ],
        vec![
            21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95,
        ],
        vec![
            78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92,
        ],
        vec![
            16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57,
        ],
        vec![
            86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
        ],
        vec![
            19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40,
        ],
        vec![
            04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
        ],
        vec![
            88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
        ],
        vec![
            04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36,
        ],
        vec![
            20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16,
        ],
        vec![
            20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54,
        ],
        vec![
            01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48,
        ],
    ];

    // →
    for y in matrix.iter() {
        for x in 3..matrix.len() {
            if let (Some(a), Some(b), Some(c), Some(d)) =
                (y.get(x - 3), y.get(x - 2), y.get(x - 1), y.get(x))
            {
                max = std::cmp::max(max, a * b * c * d);
            }
        }
    }

    // ↓
    for x in 0..matrix.len() {
        for y in 3..matrix.len() {
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                matrix.get(y - 3).and_then(|r| r.get(x)),
                matrix.get(y - 2).and_then(|r| r.get(x)),
                matrix.get(y - 1).and_then(|r| r.get(x)),
                matrix.get(y).and_then(|r| r.get(x)),
            ) {
                max = std::cmp::max(max, a * b * c * d);
            }
        }
    }

    // ↘ : x+ then y+
    for x in 3..matrix.len() {
        for y in 3..matrix.len() {
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                matrix.get(y - 3).and_then(|r| r.get(x - 3)),
                matrix.get(y - 2).and_then(|r| r.get(x - 2)),
                matrix.get(y - 1).and_then(|r| r.get(x - 1)),
                matrix.get(y).and_then(|r| r.get(x)),
            ) {
                max = std::cmp::max(max, a * b * c * d);
            }
        }
    }

    // ↙ : y+ then x-
    for x in 3..matrix.len() {
        for y in 0..matrix.len() - 3 {
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                matrix.get(y + 3).and_then(|r| r.get(x - 3)),
                matrix.get(y + 2).and_then(|r| r.get(x - 2)),
                matrix.get(y + 1).and_then(|r| r.get(x - 1)),
                matrix.get(y).and_then(|r| r.get(x)),
            ) {
                max = std::cmp::max(max, a * b * c * d);
            }
        }
    }

    max
}
