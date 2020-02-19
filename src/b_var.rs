// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block scoped language

pub fn run() {
  // Using let
  let name = "Brad";
  let age = 23; // without mut, age cannot be mutated
  let mut experience = 12; // adding mut allows experience to be mutable

  println!(
    "My name is {} and I am {} and I have {} years of experience",
    name, age, experience
  );

  experience = 20;

  println!(
    "My name is {} and I am {} and I have {} years of experience",
    name, age, experience
  );

  // Using const
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple variables
  let (my_name, my_age) = ("Jerry", 23);
  println!("{} is {}", my_name, my_age)
}
