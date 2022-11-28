use std::io;

fn main() {
    println!("Hello, world! in this tutorial we are gonna learn about input output in rust");
    let a = [1,2,3,4,5,6];

    let mut index = String::new();
    let mut name = String::new();
    // io::stdin().read_line(&mut index).expect("Failed to read input");
    io::stdin().read_line(&mut name).expect("sadad");
    // let index: usize = index.trim().parse().expect("NO");
    // println!("{}",a[index]);
    println!("Hello {}",name);
}
