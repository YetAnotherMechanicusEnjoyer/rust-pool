pub fn task03(src: &str) -> String {
    let mut rev = String::new();
    for c in src.chars().rev() {
        rev.push(c);
    }
    rev
}
