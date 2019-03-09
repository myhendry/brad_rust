extern crate rand;
use rand::Rng;

// RANDOM NUMBER L32
pub fn run() {
    let random_number = rand::thread_rng().gen_range(1, 11);
    println!("Random number {}", random_number);

    let random_bool = rand::thread_rng().gen_weighted_bool(5);
    println!("Random bool {}", random_bool);
}
