use std::io;

// hop_weight        grams
// time             minutes
// alpha_acids      fraction
// batch_size       liters
// special_gravity  SG

fn main() {
    let mut hop_weight = String::new();
    let mut time = String::new();
    let mut alpha_acids = String::new();
    let mut batch_size = String::new();
    let mut special_gravity = String::new();

    println!("Input hop weight (grams)");
    io::stdin().read_line(&mut hop_weight)
        .expect("Failed");

    println!("Input time (minutes)");
    io::stdin().read_line(&mut time)
        .expect("Failed");

    println!("Alpha acids (fraction)");
    io::stdin().read_line(&mut alpha_acids)
        .expect("Failed");

    println!("Batch size (liters)");
    io::stdin().read_line(&mut batch_size)
        .expect("Failed");

    println!("Special gravity");
    io::stdin().read_line(&mut special_gravity)
        .expect("Failed");

    println!("hop_weight: {}", hop_weight);
    println!("time: {}", time);
    println!("alpha_acids: {}", alpha_acids);
    println!("batch_size: {}", batch_size);
    println!("special_gravity: {}", special_gravity);
}
