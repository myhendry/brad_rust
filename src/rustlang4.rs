#![allow(dead_code)]

/*
  Struct
  Struct Methods
  Enum
  Enum Methods
*/

//*! ENUM
#[derive(Debug)]
enum Status {
  On,
  Off,
}

//*! 1. ENUMS: USING DIRECTION & KEYS
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

//*! 1. ENUMS: USING SHAPE
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

//*! 1. OPTION - is a basic enum type
/*
  enum Option<T> {
    Some(T),
    None,
  }
*/

fn division(x: f64, y: f64) -> Option<f64> {
  if y == 0.0 {
    None
  } else {
    Some(x / y)
  }
}

pub fn run() {
  let status = Status::Off;
  println!("status {:?}", status);

  //*! 2. ENUMS: USING DIRECTION & KEYS
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

  //*! 2. ENUMS: USING SHAPE
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

  //*! 2. OPTION - is a basic enum type
  let res = division(5.0, 7.0);
  match res {
    Some(x) => println!("{:.10}", x), //{:10} limited to 10 decimal points
    None => println!("cannot divide by 0"),
  }
}
