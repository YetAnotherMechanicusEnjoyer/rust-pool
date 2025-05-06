use super::task06::task06;

pub fn task07(nb: i32) -> i32 {
    let mut n = nb;

    while task06(n) == 0 {
        n += 1;
    }
    n
}
