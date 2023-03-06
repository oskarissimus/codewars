#![allow(dead_code)]
use std::collections::HashSet;

use itertools::Itertools;
fn longest(a1: &str, a2: &str) -> String {
    let s = a1.to_owned() + a2;
    let mut set = HashSet::new();
    for c in s.chars() {
        set.insert(c);
    }
    set.into_iter().sorted().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing(
            "loopingisfunbutdangerous",
            "lessdangerousthancoding",
            "abcdefghilnoprstu",
        );
    }
}
