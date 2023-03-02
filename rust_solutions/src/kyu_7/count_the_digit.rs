#![allow(dead_code)]
fn nb_dig(n: i32, d: i32) -> i32 {
    let mut output = 0_i32;
    let digit = char::from_digit(d as u32, 10).unwrap();
    for k in 0..=n {
        let sq = format!("{}", k * k);
        output += sq.matches(digit).count() as i32
    }
    output
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
