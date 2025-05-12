pub fn task05<'a>(src: &'a str, to_find: &str) -> Option<&'a str> {
    src.find(to_find).map(|i| &src[i..])
}
