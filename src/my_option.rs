// OPTION ENUM L37

pub fn run() {
    let name = String::from("Hendry");

    println!(
        "Character at index 3 is {}",
        match name.chars().nth(3) {
            Some(c) => c.to_string(),
            None => "No character at index 8".to_string(),
        }
    );

    println!(
        "Occupation is {}",
        match get_occupation("Dominic") {
            Some(o) => o,
            None => "No Occupation found",
        }
    )
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Dominic" => Some("Software Developer"),
        "Michael" => Some("Dentist"),
        _ => None,
    }
}
