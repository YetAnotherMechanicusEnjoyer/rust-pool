pub fn task06(nb: i32) -> u8 {
    if nb < 2 {
        return 0;
    }
    for n in 2..nb {
        if nb % n == 0 {
            return 0;
        }
    }
    1
}
