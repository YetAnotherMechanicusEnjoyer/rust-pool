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
    task01();
    println!("task02 :\r");
    task02();
    println!("task03 :\r");
    task03();
    println!("task04 :\r");
    task04(2);
    task04(-5);
    println!("task05 :\r");
    task05();
    println!("task06 :\r");
    task06();
}
