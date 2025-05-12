mod task01;
mod task02;
mod task03;
mod task05;
mod task06;
mod task07;
mod task08;
mod task09;

pub fn tasks() {
    println!("task01 :");
    let mut dest = String::new();
    let src = String::from("strcpy");
    task01::task01(&mut dest, &src);
    println!("copy {src} => {dest}\n");

    println!("task02 :");
    let mut dest = String::new();
    let src = String::from("0123456879");
    let n = 5;
    task02::task02(&mut dest, &src, n);
    println!("copy to index {n} {src} => {dest}\n");

    println!("task03 :");
    let src = String::from("reverse_str");
    let rev = task03::task03(&src);
    println!("{src} => {rev}\n");

    println!("task05 :");
    let src = String::from("RustForRustlers");
    let to_find = String::from("For");
    if let Some(str) = task05::task05(&src, &to_find) {
        println!("{to_find} in {src} => {str}\n");
    } else {
        println!("String not found.\n");
    }

    println!("task06 :");
    let s1 = String::from("RustCmp");
    let s2 = String::from("RustCpy");
    println!("{s1} strcmp {s2} => {}\n", task06::task06(&s1, &s2));

    println!("task07 :");
    let n = 4;
    println!(
        "{s1} strncmp to {n} {s2} => {}\n",
        task07::task07(&s1, &s2, n)
    );

    println!("task08 :");
    let src = String::from("i Am nOt iN uPpErCasE");
    println!("{src} => {}\n", task08::task08(&src));

    println!("task09 :");
    let src = String::from("i Am nOt iN lOwErCasE");
    println!("{src} => {}\n", task09::task09(&src));
}
