use std::fs;

fn remove_dotslash(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next();
    chars.next();
    chars.as_str()
}

pub fn task03() {
    let paths = fs::read_dir(".").unwrap();
    let mut i = 0;

    for path in paths {
        let str = path.unwrap().path();

        if i > 0 {
            print!(", ");
        }
        i += 1;
        print!("{}", remove_dotslash(str.to_str().unwrap()));
        if str.is_dir() {
            print!("/");
        }
    }
    print!("\r\n");
}
