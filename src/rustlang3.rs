#![allow(dead_code)]

#[derive(Debug)]
enum Direction {
  Up(Point),
  Down(Point),
  Left(Point),
  Right(Point),
}

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

impl Direction {
  fn match_direction(&self) -> Keys {
    match *self {
      Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
      Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
      Direction::Left(_) => Keys::LeftKey(String::from("Pressed a")),
      Direction::Right(_) => Keys::RightKey(String::from("Pressed d")),
    }
  }
}

#[derive(Debug)]
enum Keys {
  UpKey(String),
  DownKey(String),
  LeftKey(String),
  RightKey(String),
}

impl Keys {
  fn destruct(&self) -> &String {
    match *self {
      Keys::UpKey(ref s) => s,
      Keys::DownKey(ref s) => s,
      Keys::LeftKey(ref s) => s,
      Keys::RightKey(ref s) => s,
    }
  }
}

enum Shape {
  Rectangle { width: u32, height: u32 },
  Square(u32),
  Circle(f64),
}

impl Shape {
  fn area(&self) -> f64 {
    match *self {
      Shape::Rectangle { width, height } => (width * height) as f64,
      Shape::Square(ref s) => (s * s) as f64,
      Shape::Circle(ref r) => 3.14 * (r * r),
    }
  }
}

fn division(x: f64, y: f64) -> Option<f64> {
  if y == 0.0 {
    None
  } else {
    Some(x / y)
  }
}

pub fn run() {
  //TODO Enums & Options: Using Direction and Keys
  let u = Direction::Up(Point { x: 0, y: 1 });
  let k = u.match_direction();
  let x = k.destruct();

  println!("{:?}", x);

  let u = 10;
  let v = &u;
  let ref z = u;

  // THEY ARE EQUAL
  if z == v {
    println!("They are equal")
  }

  //TODO Enums & Options: Using Shape
  let r = Shape::Rectangle {
    width: 10,
    height: 70,
  };
  let s = Shape::Square(10);
  let c = Shape::Circle(4.5);

  let ar = r.area();
  println!("{}", ar);

  let aq = s.area();
  println!("{}", aq);

  let ac = c.area();
  println!("{}", ac);

  // TODO Using Options
  let res = division(5.0, 7.0);
  match res {
    Some(x) => println!("{:.10}", x),
    None => println!("cannot divide by 0"),
  }
}
