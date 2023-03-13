fn product_fib(prod: u64) -> (u64, u64, bool) {
    let (mut prev, mut curr) = (0, 1);
    while prev * curr < prod {
        (prev, curr) = (curr, prev + curr);
    }
    (prev, curr, prev * curr == prod)
}

fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
    assert_eq!(product_fib(prod), exp)
}

#[test]
fn basics_product_fib() {
    dotest(4895, (55, 89, true));
    dotest(5895, (89, 144, false));
}
