#![allow(dead_code)]
fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        return None;
    }
    let t = g as f64 / (v2 as f64 - v1 as f64);
    let hours = t;
    let minutes = hours.fract() * 60.0;
    let seconds = minutes.fract() * 60.0;
    Some([hours as i32, minutes as i32, seconds as i32].to_vec())
}

#[test]
fn basic_tests() {
    assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
    assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
    assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
    assert_eq!(race(820, 81, 550), None);
}
