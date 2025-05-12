pub fn task01<'a>(dest: &'a mut String, src: &str) -> &'a mut String {
    *dest = src.to_string();
    dest
}
