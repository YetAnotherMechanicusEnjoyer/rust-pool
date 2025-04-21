fn to_ascii(n: u16) -> String {
    match n {
        x@97..123 => format!("{}", x as u8 as char),
        _ => "".into(),
    }
}

pub fn task01() {
    for i in 97..123 {
        print!("{}", to_ascii(i));
    }
    println!("\r")
}
