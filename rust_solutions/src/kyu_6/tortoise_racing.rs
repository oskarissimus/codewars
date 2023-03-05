#![allow(dead_code)]
fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        return None;
    }
    let v1_fps = (v1 as f64) / 3600.0;
    let v2_fps = (v2 as f64) / 3600.0;
    let t = (g as f64 / (v2_fps - v1_fps)) as i32;
    let h = extract_hours(t);
    let m = extract_minutes(t);
    let s = extract_seconds(t);
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
    assert_eq!(race(820, 81, 550), None);
}
