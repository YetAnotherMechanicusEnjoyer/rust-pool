pub fn task03<'a>(dest: &'a mut String, src: &str, n: usize) -> &'a mut String {
    dest.push_str(src);
    dest.truncate(n);
    dest
}
