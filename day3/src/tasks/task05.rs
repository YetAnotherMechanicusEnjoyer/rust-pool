pub fn task05() {
    for n in 0..1000 {
        if n / 100 >= n / 10 % 10 || n / 10 % 10 >= n % 10 {
            continue;
        }
        print!("{:03} ", n);
    }
    println!("\r");
}
