mod task01;
mod task04;

pub fn tasks() {
    println!("tasks01 :");
    let src = String::from("strdup");
    println!("{src} dup => {}", task01::task01(&src));

    println!("task04 :");
    let src = String::from("str to word array");
    println!("{:?}", task04::task04(&src));
}
