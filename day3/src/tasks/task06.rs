pub fn task06() {
    let mut n = 0;

    for i in 0..100 {
        for j in i..100 {
            if j > i {
                if n > 0 {
                    print!(", ");
                }
                n += 1;
                print!("{:02} {:02}", i, j);
            }
        }
    }
    println!("\r");
}
