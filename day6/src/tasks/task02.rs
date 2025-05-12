pub fn task02<'a>(dest: &'a mut String, src: &str, n: usize) -> &'a mut String {
    let (str, _) = src.split_at(n);
    *dest = str.to_string();
    dest
}
