fn rps(p1: &str, p2: &str) -> &'static str {
    if p1wins(p1, p2) {
        return "Player 1 won!";
    }
    if p1wins(p2, p1) {
        return "Player 2 won!";
    }
    "Draw!"
}

fn p1wins(p1: &str, p2: &str) -> bool {
    p1 == "scissors" && p2 == "paper"
        || p1 == "paper" && p2 == "rock"
        || p1 == "rock" && p2 == "scissors"
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::rps;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(p1: &str, p2: &str, expected: &str) {
        assert_eq!(
            rps(p1, p2),
            expected,
            "{ERR_MSG} with p1 = \"{p1}\", p2 = \"{p2}\""
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("rock", "scissors", "Player 1 won!");
        dotest("scissors", "rock", "Player 2 won!");
        dotest("rock", "rock", "Draw!");
    }
}
