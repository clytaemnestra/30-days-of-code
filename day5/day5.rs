use std::io;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");
    let n: i32 = n.trim().parse().expect("failed to parse");

    for i in 1..11{
        println!("{} x {} = {}", n, i, n*i);
    }

}