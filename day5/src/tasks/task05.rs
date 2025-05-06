pub fn task05(nb: i32) -> i32 {
    let mut n = 0;

    while n * n < nb {
        n += 1;
    }
    if n > nb {
        return 0;
    }
    n
}
