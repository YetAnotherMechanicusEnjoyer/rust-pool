pub fn task01(nb: i32) -> i32 {
    if nb < 0 {
        return 0;
    }
    let mut n = 1;
    for i in 0..nb {
        if nb - i == 0 {
            break;
        }
        n *= nb - i;
    }
    n
}
