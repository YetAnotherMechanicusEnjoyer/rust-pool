fn to_ascii(c: char) -> u8 {
    let nb: u8 = c.to_ascii_lowercase() as u8;
    if nb < 58 && nb > 47 {
        return nb;
    }
    0
}

fn to_int(str: String) -> i32 {
    let mut len = str.len();
    let mut nb: i32 = 0;
    for c in str.chars() {
        let mut p: i32 = 1;
        let n = (c as u8 - 48) as i32;
        for _ in 1..len {
            p *= 10;
        }
        nb += n * p;
        len -= 1;
    }
    nb
}

pub fn task05(str: &String) -> i32 {
    let mut nb_str = String::new();
    let mut check = 0;
    for c in str.chars() {
        if to_ascii(c) != 0 {
            nb_str.push(to_ascii(c) as char);
            check = 1;
            continue;
        }
        if check == 1 {
            break;
        }
    }
    to_int(nb_str)
}
