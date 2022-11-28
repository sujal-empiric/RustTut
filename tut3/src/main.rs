fn main() {
    println!("Hello, world!");
    let mut i = 0;
    let mut j = 0;
    while i<10 {
        while j<10 {
            if j==i||i==9||j==0 {
                print!(" #");
            }else{
                print!(" ");
            }
            j+=1;
        }
        println!("");
        i+=1;
        j=0;
    }    
}
