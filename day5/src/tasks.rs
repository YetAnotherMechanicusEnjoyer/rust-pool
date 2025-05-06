mod task01;
mod task02;
mod task03;
mod task04;
mod task05;
mod task06;
mod task07;

pub fn tasks() {
    println!("tasks01 :\r");
    let nb = 5;
    println!("{nb}! = {}\r", task01::task01(nb));

    println!("task02 :\r");
    let nb = 5;
    println!("{nb}! = {}\r", task02::task02(nb));

    println!("task03 :\r");
    let nb = 5;
    let p = 3;
    println!("{nb}^{p} = {}\r", task03::task03(nb, p));

    println!("task04 :\r");
    let nb = 5;
    let p = 3;
    println!("{nb}^{p} = {}\r", task04::task04(nb, p));

    println!("task05 :\r");
    let nb = 25;
    println!("sqrt({nb}) = {}\r", task05::task05(nb));

    println!("task06 :\r");
    let nb = 2;
    println!("is_prime({nb}) : {}\r", task06::task06(nb));

    println!("task07 :\r");
    let nb = 0;
    println!("next prime from {nb} : {}\r", task07::task07(nb));
}
