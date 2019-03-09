use std::collections::HashMap;

pub fn run() {
    let mut marks = HashMap::new();

    // HASH MAP L31
    // Add Values
    marks.insert("Soccer", 98);
    marks.insert("Programming", 95);
    marks.insert("UX", 93);

    // Find length of hash map
    println!("Number of subjects? {}", marks.len());

    // Get a single value
    match marks.get("Soccer") {
        Some(mark) => println!("You got {} marks", mark),
        None => println!("You did not study this subject"),
    }

    // Remove a value
    marks.remove("UX");

    // Loop through HashMap
    for (subject, mark) in &marks {
        println!("For {}, you got {} marks", subject, mark);
    }

    // Check for value
    println!("Did you study C++? {}", marks.contains_key("C++"));
}
