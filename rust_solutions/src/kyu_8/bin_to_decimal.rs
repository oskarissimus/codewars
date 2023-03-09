#![allow(dead_code)]
fn bin_to_decimal(inp: &str) -> i32 {
    inp.chars()
        .map(|c| c.to_digit(2).unwrap())
        .rev()
        .enumerate()
        .fold(0, |sum, (index, digit)| {
            sum + (digit as i32) * 2_i32.pow(index as u32)
        })
}

#[test]
fn test_bin_to_decimal() {
    assert_eq!(bin_to_decimal("0"), 0);
    assert_eq!(bin_to_decimal("1"), 1);
    assert_eq!(bin_to_decimal("1001001"), 73);
}
