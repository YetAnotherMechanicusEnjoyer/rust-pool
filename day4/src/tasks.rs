mod task01;
use task01::task01;
mod task02;
use task02::task02;
mod task03;
use task03::task03;
mod task04;
use task04::task04;
mod task05;
use task05::task05;
mod task06;
use task06::task06;

pub fn tasks() {
    println!("task01 :\r");
    let mut a: i32 = 10;
    let mut b: i32 = 5;
    print!("a: {a} b: {b} => ");
    task01(&mut a, &mut b);
    println!("a: {a} b: {b}\r");

    println!("task02 :\r");
    let string: String = "This is a test for my_putstr() 2042 !\r".to_string();
    task02(&string);

    println!("task03 :\r");
    println!("String length : {}\r", task03(&string));

    println!("task04 :\r");
    task04(&string);
    println!("\r");

    println!("task05 :\r");
    println!("nb: {}\r", task05(&string));

    println!("task06 :\r");
    let mut arr: Vec<i32> = vec![40, 25, 1, 16, 32];
    print!("{:?} => ", arr);
    task06(&mut arr, 5);
    println!("{:?}\r", arr);
}
