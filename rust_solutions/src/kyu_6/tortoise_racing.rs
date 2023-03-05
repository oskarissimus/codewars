#![allow(dead_code)]
fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        return None;
    }
    let t = g * 3600 / (v2 - v1);
    let h = extract_hours(t);
    let m = extract_minutes(t);
    let s = extract_seconds(t);
    // println!("{}, {}, {}",v1,v2,g);
    Some(vec![h, m, s])
}
fn extract_hours(t: i32) -> i32 {
    t / 3600
}
fn extract_minutes(t: i32) -> i32 {
    (t / 60) % 60
}
fn extract_seconds(t: i32) -> i32 {
    t % 60
}

#[test]
fn basic_tests() {
    assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
    assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
    assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
    assert_eq!(race(820, 850, 550), Some(vec![18, 20, 0]));
    assert_eq!(race(141, 228, 87), Some(vec![1, 0, 0]));
    assert_eq!(race(820, 81, 550), None);
}
