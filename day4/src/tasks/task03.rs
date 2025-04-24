pub fn task03(str: &String) -> u16 {
    let mut len: u16 = 0;
    for _ in str.chars() {
        len += 1;
    }
    len
}
