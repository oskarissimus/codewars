#![allow(dead_code)]
fn number(bus_stops: &[(i32, i32)]) -> i32 {
    let mut people_in_bus = 0;
    for bus_stop in bus_stops {
        people_in_bus += bus_stop.0;
        people_in_bus -= bus_stop.1;
    }
    people_in_bus
}
#[test]
fn returns_expected() {
    assert_eq!(number(&[(10, 0), (3, 5), (5, 8)]), 5);
    assert_eq!(
        number(&[(3, 0), (9, 1), (4, 10), (12, 2), (6, 1), (7, 10)]),
        17
    );
    assert_eq!(
        number(&[(3, 0), (9, 1), (4, 8), (12, 2), (6, 1), (7, 8)]),
        21
    );
}
