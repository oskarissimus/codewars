use itertools::Itertools;

fn meeting(s: &str) -> String {
    s.to_uppercase()
        .split(";")
        .map(|full_name| {
            let mut splitted = full_name.split(":");
            let x = splitted.next().unwrap().to_owned();
            let y = splitted.next().unwrap().to_owned();
            (x, y)
        })
        .sorted_unstable_by(|(a_first, a_last), (b_first, b_last)| {
            if a_last == b_last {
                Ord::cmp(a_first, b_first)
            } else {
                Ord::cmp(a_last, b_last)
            }
        })
        .map(|(first, last)| format!("({}, {})", last, first))
        .join("")
}
#[cfg(test)]

mod tests {
    use super::*;

    fn dotest(s: &str, exp: &str) -> () {
        let ans = meeting(s);
        assert_eq!(ans, exp);
    }

    #[test]
    fn basic_tests() {
        dotest("Alexis:Wahl;John:Bell;Victoria:Schwarz;Abba:Dorny;Grace:Meta;Ann:Arno;Madison:STAN;Alex:Cornwell;Lewis:Kern;Megan:Stan;Alex:Korn",
               "(ARNO, ANN)(BELL, JOHN)(CORNWELL, ALEX)(DORNY, ABBA)(KERN, LEWIS)(KORN, ALEX)(META, GRACE)(SCHWARZ, VICTORIA)(STAN, MADISON)(STAN, MEGAN)(WAHL, ALEXIS)");
        dotest("John:Gates;Michael:Wahl;Megan:Bell;Paul:Dorries;James:Dorny;Lewis:Steve;Alex:Meta;Elizabeth:Russel;Anna:Korn;Ann:Kern;Amber:Cornwell",
               "(BELL, MEGAN)(CORNWELL, AMBER)(DORNY, JAMES)(DORRIES, PAUL)(GATES, JOHN)(KERN, ANN)(KORN, ANNA)(META, ALEX)(RUSSEL, ELIZABETH)(STEVE, LEWIS)(WAHL, MICHAEL)");
        dotest("Alex:Arno;Alissa:Cornwell;Sarah:Bell;Andrew:Dorries;Ann:Kern;Haley:Arno;Paul:Dorny;Madison:Kern",
            "(ARNO, ALEX)(ARNO, HALEY)(BELL, SARAH)(CORNWELL, ALISSA)(DORNY, PAUL)(DORRIES, ANDREW)(KERN, ANN)(KERN, MADISON)");
    }
}
