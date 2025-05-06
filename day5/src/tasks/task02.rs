pub fn task02(nb: i32) -> i32 {
    if nb < 0 {
        return 0;
    }
    if nb == 0 {
        return 1;
    }
    nb * task02(nb - 1)
}
