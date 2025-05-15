mod task02;
mod task03;

pub fn tasks() {
    println!("task02 :");
    let mut dest = String::from("str");
    let src = String::from("cat");
    task02::task02(&mut dest, &src);
    println!("str + {src} = {dest}");

    println!("task03 :");
    let mut dest = String::from("str");
    let src = String::from("ncat");
    let n = 5;
    task03::task03(&mut dest, &src, n);
    println!("str + {src} - {n} = {dest}");
}
