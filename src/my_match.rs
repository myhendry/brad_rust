pub fn run() {
    let number = 11;

    match number {
        1 => println!("number is one"),
        2 => println!("number is two"),
        2...9 => println!("number is between two to nine"),
        10 | 11 => println!("number is either ten or eleven"),
        _ => println!("some other number"),
    }
}
