#![allow(dead_code)]

fn get_sum(a: i64, b: i64) -> i64 {
    let double_mean = a + b;
    let elements = (a - b).abs() + 1;
    double_mean * elements / 2
}

// See: https://doc.rust-lang.org/book/testing.html

#[cfg(test)]
mod tests {
    use super::get_sum;

    #[test]
    fn sample_tests() {
        assert_eq!(get_sum(0, 1), 1);
        assert_eq!(get_sum(1, 2), 3);
        assert_eq!(get_sum(5, -1), 14);
        assert_eq!(get_sum(505, 4), 127759);
    }
}
