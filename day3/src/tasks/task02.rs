fn to_ascii(n: u16) -> String {
    match n {
        x@97..123 => format!("{}", x as u8 as char),
        _ => "".into(),
    }
}

pub fn task02() {
    let range = 97..123;

    for i in range.rev() {
        print!("{}", to_ascii(i));
    }
    println!("\r");
}
