use std::io;

enum Days{
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

fn main() {
//     const PI: f32 = 3.14;
//     println!("Hola!");
//     let mut x: u32 = 4;
//     println!("x is {}", x);
//     x = 5;
//     println!("x is {}", x);
//     println!("Constant is {}",PI);

    let a: u8=255;
    let b: i8 = -100;
    let c: f32 = 10.2;
    let d: bool = true;
    let e: char = '!';
    let mut tup = (112,true,'@');
    println!("a: {}, b: {}, c: {}, d: {}, e: {}",a,b,c,d,e);
    println!("0: {}, 1: {}, 2:{}",tup.0,tup.1,tup.2);
    tup = (112,false,'*');
    println!("0: {}, 1: {}, 2:{}",tup.0,tup.1,tup.2);

    let arr = [1,2,3,4,5];
    println!("This is the Array: {} {} {} {} {}",arr[0],arr[1],arr[2],arr[3],arr[4]);
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Unable to get Input");
    // println!("{}",input)

    let a = 1_u64;
    let b = 2_u32;
    let z = a + (b as u64);
    println!("{}",z);
    // let mut number = String::new();
    // io::stdin().read_line(&mut number).expect("msg");

    // let num: i32 = number.trim().parse().unwrap();
    // println!("{}",num);
    let res: i32 = add_number(1, 2);
    println!("{}",res);

    let today = Days::Monday;
    match today {
        Days::Sunday=> println!("Happy Weekend!"),
        Days::Monday=> println!("Today is a Long Day, is'nt it!"),
        Days::Tuesday=> println!("3 more to go...!"),
        Days::Wednesday=> println!("Finally it's the Humpday!"),
        Days::Thursday=> println!("1 more my Friend!"),
        Days::Friday=> println!("Movie Night!"),
        Days::Saturday=> println!("Time to catch the Dreams!"),
    }
}


fn add_number(x: i32,y: i32) -> i32{
    x + y 
}