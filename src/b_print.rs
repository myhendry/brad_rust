pub fn run() {
  // print to console
  println!("Hello from basics");

  // Basic formatting
  println!("{} is  from {}", "Hendry", "Lim");

  // Positional Arguments
  println!(
    "{0} is from {1} and {0} likes to {2}",
    "Hendry", "Singapore", "code"
  );

  // Named Arguments
  println!(
    "{name} likes to play {activity}",
    name = "Hendry",
    activity = "Soccer"
  );

  // Placeholder Traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // Placeholder for Debug trait
  println!("{:?}", (12, true, "hello"));

  // Basic Math
  println!("10 + 10 = {}", 10 + 10);
}
