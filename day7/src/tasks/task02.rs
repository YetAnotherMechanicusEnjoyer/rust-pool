pub fn task02<'a>(dest: &'a mut String, src: &str) -> &'a mut String {
    dest.push_str(src);
    dest
}
