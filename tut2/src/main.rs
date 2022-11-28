#![allow(unused)]

fn main() {
    println!("Hello, world!");
    let mut a = 12;
    let mut b = 14;
    if(a>b){
        println!("Value of A is grater than B: {}",a);
    }else{
        println!("Value of B is grater than A: {}",b);
    }
    let can_vote = if a>b {
        true
    }else{
        false
    };
    println!("is A grater than B: {}",can_vote);
    

    let age = 28;
    match age{
        1..=16=> println!("Time to enjoy your life"),
        17..=27=> println!("Time to make money"),
        _ => println!("Rest my friend")
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1
    };
    println!("{}",binary)
}
