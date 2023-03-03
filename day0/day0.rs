use std::io;

fn main(){
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("wrong value");
    print!("Hello, World. \n{}", input_string);   
}

