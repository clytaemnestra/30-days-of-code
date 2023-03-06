use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");
    let n: i32 = n.trim().parse().expect("failed to parse");

    if (n % 2 == 0 && n > 20 ) || (n % 2 == 0 && n >= 2 && n <= 5) {
        println!("Not Weird");
    } else {
        println!("Weird");
    }
}