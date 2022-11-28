#![allow(unused)]
fn main() {
    let name = "Sam Smith";
    println!("Hello, {name}! We are going to learn about Tuple");
    let  x: (u32, i8, f64) = (112, -1, 0.001);
    let (a, b, c) = x;

    // to access the values from the tuple 
    println!("{} {} {}",x.0, x.1, x.2);

}
