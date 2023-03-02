#![allow(dead_code)]
fn nb_dig(n: i32, d: i32) -> i32 {
    let digit = char::from_digit(d as u32, 10).unwrap();
    (0..=n)
        .map(|k| k * k)
        .map(|sq| format!("{}", sq))
        .fold(0, |acc, sq| acc + sq.matches(digit).count() as i32)
}
#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i32, d: i32, exp: i32) -> () {
        println!("n: {:?}", n);
        println!("d: {:?}", d);
        let ans = nb_dig(n, d);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(550, 5, 213);
        dotest(5750, 0, 4700);
    }
}
