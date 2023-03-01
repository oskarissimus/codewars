fn gimme(input_array: [i32; 3]) -> usize {
    let index_of_max = input_array
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index)
        .unwrap();
    let index_of_min = input_array
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index)
        .unwrap();
    input_array
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != index_of_max && *index != index_of_min)
        .map(|(index, _)| index)
        .nth(0)
        .unwrap()
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gimme() {
        assert_eq!(gimme([2, 3, 1]), 0);
        assert_eq!(gimme([-2, -3, -1]), 0);
        // assert_eq!(gimme([5, 10, 14]), 1);
    }
}
