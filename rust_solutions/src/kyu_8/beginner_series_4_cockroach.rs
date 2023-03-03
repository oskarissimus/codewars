#![allow(dead_code)]
fn cockroach_speed(speed_in_kmph: f64) -> i64 {
    // To convert kilometers per hour (km/h) to centimeters per second (cm/s), you need to use the following formula:
    // 1 km/h = (1000 meters / 1 kilometer) / (3600 seconds / 1 hour) = 0.27778 meters/second
    // 1 meter/second = 100 centimeters/second
    // Therefore, 1 km/h = 0.27778 meters/second * 100 cm/meter = 27.778 cm/s
    // So to convert kmph to cm/s, multiply the speed in km/h by 27.778.
    let cmps_to_kmph_ratio = 1000.0 * 100.0 / 3600.0;
    (speed_in_kmph * (cmps_to_kmph_ratio)) as i64
}
#[test]
fn basic_tests() {
    assert_eq!(cockroach_speed(1.08), 30);
    assert_eq!(cockroach_speed(1.09), 30);
    assert_eq!(cockroach_speed(0.0), 0);
}
