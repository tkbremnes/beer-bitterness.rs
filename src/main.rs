use std::io;

mod tinseth;
pub use tinseth::*;

// hop_weight       grams
// time             minutes
// alpha_acids      fraction
// batch_size       liters
// wort_gravity     SG

fn main() {
    let mut hop_weight = String::new();
    let mut boil_time = String::new();
    let mut alpha_acids = String::new();
    let mut batch_size = String::new();
    let mut wort_gravity = String::new();

    println!("Input hop weight (grams)");
    io::stdin().read_line(&mut hop_weight)
        .expect("Failed");
    let hop_weight: u32 = hop_weight.trim().parse()
        .expect("Failed");

    println!("Input time (minutes)");
    io::stdin().read_line(&mut boil_time)
        .expect("Failed");
    let boil_time: u32 = boil_time.trim().parse()
        .expect("Failed");

    println!("Alpha acids (fraction)");
    io::stdin().read_line(&mut alpha_acids)
        .expect("Failed");
    let alpha_acids: f32 = alpha_acids.trim().parse()
        .expect("Failed");

    println!("Batch size (liters)");
    io::stdin().read_line(&mut batch_size)
        .expect("Failed");
    let batch_size: u32 = batch_size.trim().parse()
        .expect("Failed");

    println!("Special gravity");
    io::stdin().read_line(&mut wort_gravity)
        .expect("Failed");
    let wort_gravity: f32 = wort_gravity.trim().parse()
        .expect("Failed");

    println!("hop_weight: {}", hop_weight);
    println!("time: {}", boil_time);
    println!("alpha_acids: {}", alpha_acids);
    println!("batch_size: {}", batch_size);
    println!("wort_gravity: {}", wort_gravity);

    let result = Tinseth::calculate_ibu(hop_weight, boil_time, alpha_acids, batch_size, wort_gravity);
    println!("result: {}", result);
}
