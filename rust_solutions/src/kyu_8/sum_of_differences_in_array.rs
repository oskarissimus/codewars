use itertools::Itertools;
use std::ops::Mul;

fn sum_of_differences(arr: &[i8]) -> Option<i8> {
    match arr.len() {
        0 => None,
        1 => None,
        _ => {
            let mut arr_iter = arr.iter().sorted_unstable_by_key(|n| n.mul(-1));
            let mut previous = arr_iter.next().unwrap();
            let mut sum = 0;
            for current in arr_iter {
                sum = sum + previous - current;
                previous = current;
            }
            Some(sum)
        }
    }
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::sum_of_differences;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    #[test]
    fn sample_tests() {
        assert_eq!(sum_of_differences(&[1, 2, 10]), Some(9), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-3, -2, -1]), Some(2), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[1, 1, 1, 1, 1]), Some(0), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-17, 17]), Some(34), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[0]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-1]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[1]), None, "{}", ERR_MSG);
    }
}
