#![allow(dead_code)]
fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if h > 0.0 && bounce > 0.0 && bounce < 1.0 && window < h {
        let mut times = -1;
        let mut height_after_bounce = h;
        while height_after_bounce > window {
            height_after_bounce *= bounce;
            times += 2;
        }
        times
    } else {
        -1
    }
}

fn testequal(h: f64, bounce: f64, window: f64, exp: i32) -> () {
    assert_eq!(bouncing_ball(h, bounce, window), exp)
}

#[test]
fn tests_bouncing_ball() {
    testequal(3.0, 0.66, 1.5, 3);
    testequal(30.0, 0.66, 1.5, 15);
    testequal(40.0, 0.4, 10.0, 3);
    testequal(10.0, 0.6, 10.0, -1);
}
