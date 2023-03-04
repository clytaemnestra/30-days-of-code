use std::io;

fn main(){
    let i = 4;
    let d = 4.0;
    let s = "HackerRank ";

    let mut input_j = String::new();
    io::stdin().read_line(&mut input_j).expect("wrong value");
    let j: i32 = match input_j.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Failed to parse integer.");
            0
        }
    };

    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).expect("wrong value");
    let b: f64 = match input_b.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Failed to parse float.");
            0.0
        }
    };

    let mut st = String::new();
    io::stdin().read_line(&mut st).expect("wrong value");

    let sum = i + j;
    let sum2 = d + b;
    let concat = s.to_owned() + &st;
    println!("{}, {:.1}, {}", sum, sum2, concat);
}