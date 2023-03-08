#![allow(dead_code)]
fn duty_free(price: i32, discount: i32, holiday_cost: i32) -> i32 {
    let savings_on_one_bottle = (price * discount) as f64 / 100.0;
    let total_savings = holiday_cost as f64 / savings_on_one_bottle;
    total_savings as i32
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(duty_free(12, 50, 1000), 166);
        assert_eq!(duty_free(17, 10, 500), 294);
    }
}
