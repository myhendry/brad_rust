#![allow(dead_code)]

// 42A1 Parsing JSON
extern crate serde;
extern crate serde_json;
use serde_json::Value as JsonValue;

// 40A Running/Executing Commands (CLI)
use std::process::Command;
// 25A Reading a File
// 27A Writing to a File
use std::fs::File;
use std::io::prelude::*;
// 26A Command Line Arguments
use std::env;
// 30A Reading User Input
use std::collections::HashMap;
// 31A Hash Maps (HashMap)
use std::io;
// 32A Random number and Booleans
extern crate rand;
use rand::Rng;
// 35A Regular Expressions (Regex)
extern crate regex;
use regex::Regex;
// 38A HTTP Get Request (Reqwest crate)
extern crate reqwest;

// 42C Parsing JSON
#[derive(Serialize, Deserialize)]
struct Human {
  name: String,
  age: u8,
  is_male: bool,
}

// 17A Structs
struct Color {
  red: u8, // u8: 0 - 255
  green: u8,
  blue: u8,
}

// 18A Tuple Structs
struct TColor(u8, u8, u8);

// 21A Impl Keyword (Implementation)
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn print_description(&self) {
    println!("Rectangle {} x {}", self.width, self.height);
  }

  fn is_square(&self) -> bool {
    return self.width == self.height;
  }
}

// 22A Implmenting Traits
// similar to interfaces

struct Person {
  name: String,
  age: u8,
}

impl ToString for Person {
  fn to_string(&self) -> String {
    return format!("My name is {} and I am {}", self.name, self.age);
  }
}

// 28A Defining Traits
struct Employee {
  name: String,
  age: u8,
}

trait HasVoiceBox {
  // Speak
  fn speak(&self);
  // Check if can speak
  fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Employee {
  fn speak(&self) {
    println!("Hello, my name is {}", self.name);
  }

  fn can_speak(&self) -> bool {
    if self.age > 0 {
      return true;
    }
    return false;
  }
}

// 36A Modules (Mod keyword)
mod dcode {
  fn chicken() {
    println!("Chicken");
  }

  pub fn print_msg() {
    chicken();
    println!("How is it going?");
  }

  pub mod water {
    pub fn print_msg() {
      println!("I'm water");
    }
  }
}

// 39A Enum Methods
enum Day {
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
  Sunday,
}

impl Day {
  fn is_weekday(&self) -> bool {
    match self {
      &Day::Saturday | &Day::Sunday => return false,
      _ => return true,
    }
  }
}

pub fn run() {
  println!("- - - - - - - - - - - - PLAYGROUND BEGIN - - - - - - - - - - - - - - - - ");
  // 3 Comments
  // single line comment
  /*
      multi line comment
      multi line comment
  */

  // 4. Variables
  // By default all variables is immutable unless use keyword mut
  let mut a = String::from("cucumber");
  println!("a is {}", a);
  a = String::from("papaya");
  println!("a is {}", a);

  // 5. Data Types
  /*
  Primitive Types--
  Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
  Floats: f32, f64
  Boolean (bool)
  Characters (char)
  Tuples
  Arrays  - fixed length
  Vectors - growable length
  */

  //? value assigned to b is never used
  let mut b = "banana";
  b = "orange";
  println!("b is {}", b);

  let x: i32 = 70;
  let mut y: i32 = 40;
  println!("value of x is {}", x);
  println!("value of y before mutate is {}", y);

  y = 50;
  println!("value of y after mutate is {}", y);

  // 6 If-Else
  if x > 20 && x < 40 {
    println!("x is greater than 20 and less than 40");
  } else if x >= 40 && x < 60 {
    println!("x is greater than 40 and less than 60");
  } else {
    println!("x is greater than 60");
  }

  // 7  Loop
  let mut count = 0;
  loop {
    count += 1;

    if count == 2 {
      continue;
    }

    if count == 5 {
      break;
    }

    println!("loop {}", count);
  }

  // 8 While Loop
  let mut num = 0;

  while num <= 3 {
    println!("while {}", num);

    num += 1;
  }

  // 9 For Loop
  for i in 1..5 {
    println!("for i is {}", i);
  }

  // 10 Enum
  enum Direction {
    Up,
    Down,
    Left,
    Right,
  }

  let player_direction = Direction::Left;

  match player_direction {
    Direction::Up => println!("Going Up"),
    Direction::Down => println!("Going Down"),
    Direction::Left => println!("Going Left"),
    Direction::Right => println!("Going Right"),
  }

  // 11 Constants
  const MAXIMUM_NUMBER: u8 = 3;

  for n in 1..MAXIMUM_NUMBER {
    println!("n {}", n);
  }

  // 12 Tuples
  // Tuples group together values of different types
  // Max 12 elements
  let tup = (1, 3, "computer", (2, false));

  let (tup_item1, tup_item2, tup_item3, tup_item4) = tup;
  println!("tupItem1 {}", tup_item1);
  println!("tupItem4 {:?}", tup_item4);

  let tup_selected = (tup.3).1;
  println!("tup selected {}", tup_selected);

  // 13B Functions
  let result = print_result(10);
  println!("{}", multiply_by_100(result));

  // 14 Code Block
  let block_item1 = "global";

  {
    let block_item2 = "inside";
    println!("code block {} {}", block_item1, block_item2);
  }

  /*
    //ERROR  block_item2 cannot be found
    println!("code block {} | {}", block_item1, block_item2);
  */

  // 15 Shadowing
  let mut shadow1 = 10;
  let mut shadow2 = 100;

  {
    shadow1 = 15;
    let shadow2 = "orange";
    println!("shadow2 {}", shadow2);
  }

  let shadow1 = "Who is that?";
  let shadow1 = true;

  // What is shadow? values?
  println!("shadow1 {}", shadow1);
  println!("shadow2 {}", shadow2);

  // 16  References
  let mut ref1 = 30;
  let ref1r = &ref1; // simple reference

  let mut ref2 = 300;

  /*
    {
      let ref2r = &mut ref2;
      *ref2r += 1;
    }

    println!("ref2 {}", ref2);
  */

  let ref2r = &mut ref2;
  *ref2r += 1;

  // println!("ref2r {}", ref2r);
  println!("ref2 {}", ref2);

  // 17B Structs
  let mut bg = Color {
    red: 255,
    green: 70,
    blue: 15,
  };

  bg.green = 200;

  println!("{} {} {}", bg.red, bg.green, bg.blue);

  // 18B Tuple Structs
  let mut red = TColor(255, 0, 0);

  red.2 = 100;

  println!("red is {}, {}, {}", red.0, red.1, red.2);

  // 19A Pass by Reference
  let blue = Color {
    red: 0,
    green: 0,
    blue: 255,
  };

  print_color(&blue);

  // 20 Array
  let numbers = [1, 2, 3];
  let t_numbers: [i32; 3] = [4, 5, 6];
  let r_numbers = [2; 5];

  for n in numbers.iter() {
    println!("numbers {}", n);
  }

  for i in 0..t_numbers.len() {
    println!("t numbers {}", t_numbers[i]);
  }

  for x in 0..r_numbers.len() {
    println!("r numbers {}", r_numbers[x]);
  }

  // 21B Impl Keyword (Implementation)
  let my_rect = Rectangle {
    width: 10,
    height: 5,
  };
  my_rect.print_description();
  println!("Rectangle is a square {}", my_rect.is_square());

  // 22 Strings
  let mut my_string = String::from("Hey, how are you?");

  println!("Length {}", my_string.len());
  println!("String is Empty {}", my_string.is_empty());

  for token in my_string.split_whitespace() {
    println!("{}", token);
  }

  println!(
    "Does the string contain word how {}",
    my_string.contains("how")
  );

  my_string.push_str("Welcome to our tutorial");

  println!("{}", my_string);

  // 22B Implmenting Traits
  let hendry = Person {
    name: String::from("Hendry"),
    age: 22,
  };

  println!("{}", hendry.to_string());

  // 24 Vectors
  // Vectors - Resizable arrays

  let mut my_vector = vec![1, 2, 3, 4];

  println!("{}", my_vector[2]);

  my_vector.push(49);
  my_vector.remove(1); // remove '2'

  for number in my_vector.iter() {
    println!("{}", number);
  }

  // 25B Reading a File
  let mut file = File::open("info.txt").expect("Can't open file");

  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .expect("Oops! Cannot read the file");

  println!("File Contents:\n\n{}", contents);

  // 26B Command Line Arguments
  let args: Vec<String> = env::args().collect();

  println!("{}", args[1]); // args[0] is target/debug/lab, arg[1] is <entry 1>, arg[2] is <entry 2>

  for argument in args.iter() {
    println!("{}", argument);
  }

  // 27B Writing to a File
  let mut file = File::create("output.txt").expect("Cannot create file!");

  file
    .write_all(b"Welcome to hendrylim!")
    .expect("Cannot write to the file, sorry pal");

  // 28B Defining Traits
  let employee1 = Employee {
    name: String::from("Hendry"),
    age: 22,
  };

  println!("Can {} speak? {}", employee1.name, employee1.can_speak());

  // 29 Pattern Matching (Switch Statement)
  let expected_number = 8;

  match expected_number {
    1 => println!("It is one"),
    2 => println!("It is two"),
    3..=9 => println!("It is between 3 to 9"),
    10 | 11 => println!("It is 10 or 11"),

    _ => println!("It doesn't match"),
  }

  let expected_string = "hendry";

  match expected_string {
    "chris" => println!("your name is chris"),
    "hendry" => println!("your name is hendry"),
    _ => println!("I don't know your name"),
  }

  // 30 Reading User Input
  let mut input = String::new();

  println!("Hey mate, say something");

  match io::stdin().read_line(&mut input) {
    Ok(_) => {
      println!("Success! You said: {}", input.to_uppercase());
    }
    Err(e) => println!("Oops! Something went wrong! {}", e),
  }

  // 31B Hash Maps (HashMap)
  let mut marks = HashMap::new();

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

  // 32B Random number and Booleans
  let random_number = rand::thread_rng().gen_range(1, 11);
  println!("Random number {}", random_number);

  let random_bool = rand::thread_rng().gen_weighted_bool(5);
  println!("Random bool {}", random_bool);

  // 33 String Methods
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

    // Get character at index 4
    match my_string.chars().nth(4) {
      Some(c) => println!("Character at index 4: {}", c),
      None => println!("No character at index 4"),
    }
  }

  //? 35A Regular Expressions (Regex)
  // to use regex, must create regex struct using new
  let re = Regex::new(r"\w{5}").unwrap();
  let text = "dcode";

  println!("Found match? {}", re.is_match(text));

  // 36B Modules (Mod keyword)
  dcode::print_msg();
  dcode::water::print_msg();

  // 37A Option (Enum)
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
  );

  // 38A HTTP Get Request (Reqwest crate)
  // Long Way
  match reqwest::get("http://youtube.local/hello") {
    Ok(mut response) => {
      // Check if 200 OK
      if response.status() == reqwest::StatusCode::Ok {
        match response.text() {
          Ok(text) => println!("Response text: {}", text),
          Err(_) => println!("Could not read response text"),
        }
      } else {
        println!("Response was not 200 OK");
      }
    }
    Err(_) => println!("Could not make the request!"),
  }

  // Shorter Way
  /*
    let response_text = reqwest::get("http://youtube.local/hello")
      .expect("Couldn't make request!")
      .text()
      .expect("Couldn't read response text");

    println!("Response Text: {}", response_text);
  */

  // 39A Enum Methods
  let d = Day::Tuesday;

  println!("Is d a weekday? {}", d.is_weekday());

  // 40B Running/Executing Commands (CLI)
  // python dcode.py
  let mut cmd = Command::new("python");
  cmd.arg("dcode.py");

  // Execute the command
  match cmd.output() {
    Ok(o) => unsafe {
      println!("Output: {}", String::from_utf8_unchecked(o.stdout));
    },
    Err(e) => {
      println!("There was an error! {}", e);
    }
  }

  // 42B Parsing JSON
  let json_str = r#" 
    {
      "name": "Hendry",
      "age": 22,
      "is_male": true
    }
  "#;

  let res = serde_json::from_str(json_str);

  if res.is_ok() {
    let p: JsonValue = res.unwrap();
    println!("The name is {}", p["name"].as_str().unwrap());
  } else {
    println!("Could not pass JSON")
  }

  let res1 = serde_json::from_str(json_str);

  if res1.is_ok() {
    let p: Human = res1.unwrap();
    println!("The name is {}", p.name);
  } else {
    println!("Could not pass JSON")
  }

  println!("- - - - - - - - - - - - PLAYGROUND END - - - - - - - - - - - - - - - - ");
}

// 37B Option (Enum)
fn get_occupation(name: &str) -> Option<&str> {
  match name {
    "Dominic" => Some("Software Developer"),
    "Michael" => Some("Dentist"),
    _ => None,
  }
}

// 19B Pass by Reference
fn print_color(c: &Color) {
  println!("Color - Red: {} Green: {} Blue: {}", c.red, c.green, c.blue);
}

// 13A Functions
fn print_result(num: u32) -> u32 {
  return num * 5;
}

fn multiply_by_100(num: u32) -> u32 {
  return num * 100;
}
