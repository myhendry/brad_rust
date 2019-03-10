pub fn run() {
    // L33 STRING METHODS
    // Replace
    {
        let my_string = String::from("Rust is fantastic!");
        println!("After replace: {}", my_string.replace("fantastic", "great"));
    }

    // Lines
    {
        let my_string = String::from("Hello world \nrust! \noutside");
        for line in my_string.lines() {
            println!("[ {} ]", line);
        }
    }

    // Split
    {
        let my_string = String::from("Leave+a+like+if+you+enjoyed");
        let tokens: Vec<&str> = my_string.split("+").collect();

        println!("{}", my_string);
        println!("At index 2: {}", tokens[2]);
    }

    // Trim
    {
        let my_string = String::from("   My name is Hendry    \n\r");

        println!("Before trim: {}", my_string);
        println!("After trim: {}", my_string.trim());
    }

    // Chars
    {
        let my_string = String::from("hendry on youtube");

        match my_string.chars().nth(4) {
            Some(c) => println!("Character at index 4: {}", c),
            None => println!("No character at index 4"),
        }
    }
}
