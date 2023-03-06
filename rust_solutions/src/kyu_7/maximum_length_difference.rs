#![allow(dead_code)]

fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    if a1.is_empty() || a2.is_empty() {
        return -1;
    }
    let a1_min = a1.iter().map(|s| s.len()).min().unwrap() as i32;
    let a1_max = a1.iter().map(|s| s.len()).max().unwrap() as i32;

    let a2_min = a2.iter().map(|s| s.len()).min().unwrap() as i32;
    let a2_max = a2.iter().map(|s| s.len()).max().unwrap() as i32;
    (a2_max - a1_min).max(a1_max - a2_min)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(a1: Vec<&str>, a2: Vec<&str>, exp: i32) -> () {
        println!("a1: {:?};", a1);
        println!("a2: {:?};", a2);
        let ans = mx_dif_lg(a1, a2);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut s1 = vec![
            "hoqq",
            "bbllkw",
            "oox",
            "ejjuyyy",
            "plmiis",
            "xxxzgpsssa",
            "xxwwkktt",
            "znnnnfqknaz",
            "qqquuhii",
            "dvvvwz",
        ];
        let mut s2 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
        dotest(s1, s2, 13);
        s1 = vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ];
        s2 = vec!["bbbaaayddqbbrrrv"];
        dotest(s1, s2, 10);
    }
}
