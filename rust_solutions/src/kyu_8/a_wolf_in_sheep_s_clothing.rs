#![allow(dead_code)]

use itertools::Itertools;
fn warn_the_sheep(queue: &[&str]) -> String {
    let n = queue.len();
    let wolf_index = queue
        .iter()
        .find_position(|&&item| item == "wolf")
        .unwrap()
        .0;
    if wolf_index == n - 1 {
        return "Pls go away and stop eating my sheep".to_string();
    }
    format!(
        "Oi! Sheep number {}! You are about to be eaten by a wolf!",
        n - wolf_index - 1
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            warn_the_sheep(&[
                "sheep", "sheep", "sheep", "sheep", "sheep", "wolf", "sheep", "sheep"
            ]),
            "Oi! Sheep number 2! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "wolf", "sheep", "sheep", "sheep", "sheep", "sheep"]),
            "Oi! Sheep number 5! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["wolf", "sheep", "sheep", "sheep", "sheep", "sheep", "sheep"]),
            "Oi! Sheep number 6! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "wolf", "sheep"]),
            "Oi! Sheep number 1! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "sheep", "wolf"]),
            "Pls go away and stop eating my sheep",
        );
    }
}
