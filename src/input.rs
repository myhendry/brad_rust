use std::io;

// READ INPUT L30
pub fn run() {
    let mut input = String::new();

    println!("Hey mate! say something:");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Success! You said {}", input.to_uppercase());
        }
        Err(e) => println!("Oops! Something went wrong {}", e),
    }
}
