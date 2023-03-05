use std::io;

fn main(){
    let mut meal_cost = String::new();
    io::stdin()
        .read_line(&mut meal_cost)
        .expect("failed to read input.");
    let meal_cost: f32 = meal_cost.trim().parse().expect("invalid input");

    let mut tip_percent = String::new();
    io::stdin()
        .read_line(&mut tip_percent)
        .expect("failed to read input.");
    let tip_percent: f32 = tip_percent.trim().parse().expect("invalid input");

    let mut tax_percent = String::new();
    io::stdin()
        .read_line(&mut tax_percent)
        .expect("failed to read input.");
    let tax_percent: f32 = tax_percent.trim().parse().expect("invalid input");

    let total_price: f32 = meal_cost + (meal_cost * (tip_percent / 100.0)) + (meal_cost * (tax_percent / 100.0));
    println!("{}", total_price as i32);
}