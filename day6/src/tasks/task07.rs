pub fn task07(s1: &str, s2: &str, n: usize) -> i8 {
    let mut ch1 = s1.chars();
    let mut ch2 = s2.chars();

    for _ in 0..n {
        match (ch1.next(), ch2.next()) {
            (Some(ch1), Some(ch2)) => {
                if ch1 != ch2 {
                    return (ch1 as i8).wrapping_sub(ch2 as i8);
                }
            }
            (Some(ch1), None) => return ch1 as i8,
            (None, Some(ch2)) => return -(ch2 as i8),
            (None, None) => return 0,
        }
    }
    0
}
