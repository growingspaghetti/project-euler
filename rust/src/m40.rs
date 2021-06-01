// An irrational decimal fraction is created by concatenating the positive integers:

// 0.123456789101112131415161718192021...

// It can be seen that the 12th digit of the fractional part is 1.

// If dn represents the nth digit of the fractional part, find the value of the following expression.

// d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000

struct Container {
    capacity: u32,
    elements: u32,
}

fn digit_at(nth: u32) -> u8 {
    let mut container = Container {
        capacity: 0,
        elements: 0,
    };
    let mut w = 1u32;
    loop {
        let elements = 10u32.pow(w) - 10u32.pow(w - 1);
        let capacity = w * elements;
        if nth < container.capacity + capacity {
            break;
        }
        container.capacity += capacity;
        container.elements += elements;
        w += 1;
    }
    let residue = nth - container.capacity;
    if residue % w == 0 {
        return ((container.elements + residue / w) % 10) as u8;
    }
    let num = container.elements + residue / w + 1;
    ((num / 10u32.pow(w - residue % w)) % 10) as u8
}

mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(digit_at(1), 1);
        assert_eq!(digit_at(17), 3);
        assert_eq!(digit_at(18), 1);
        assert_eq!(digit_at(190), 1);
        assert_eq!(digit_at(194), 0);
        assert_eq!(digit_at(37371), 6);
    }
}

// 183 ns
/// ```rust
/// use self::project_euler::m40::champernownes_constant;
/// assert_eq!(champernownes_constant(), 210);
/// ```
pub fn champernownes_constant() -> u32 {
    (0u32..=6)
        .map(|d| 10u32.pow(d))
        .map(|d| digit_at(d))
        .map(|d| d as u32)
        .product()
}
