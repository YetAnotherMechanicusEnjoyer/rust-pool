pub fn task04(nb: i32, p: i16) -> i32 {
    if p < 0 {
        return 0;
    }
    if p == 0 {
        return 1;
    }
    nb * task04(nb, p - 1)
}
