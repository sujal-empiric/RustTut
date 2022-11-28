fn main() {
    println!("Hello, world!");
    let a = [1,2,3,4,5,6];
    let mut b: [i32; 6] = [1,2,3,4,5,6];
    println!("{}",b[1]);
    println!("{}",a[2]);
    b[1]=100;
    println!("{}",b[1]);
    
    
}
