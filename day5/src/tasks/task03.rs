pub fn task03(nb: i32, p: i16) -> i32 {
    if p < 0 {
        return 0;
    }
    let mut n = 1;
    for _ in 0..p {
        n *= nb;
    }
    n
}
