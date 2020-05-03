#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;

//*! A. ENUMS
#[derive(Debug)]
enum Direction {
  Up(Point),
  Down(Point),
  Left(Point),
  Right(Point),
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

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

#[derive(Debug)]
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

pub fn run() {
  //*! OPTIONS
  let so = Some('c');

  match so {
    Some(i) => println!("i {}", i),
    _ => {}
  }

  //** Using if let
  if let Some(i) = so {
    println!("i {}", i);
  } else {
    {}
  }

  let mut ss = Some(0);

  loop {
    match ss {
      Some(i) => {
        if i > 19 {
          println!("Quit");
          ss = None;
        } else {
          println!("{}", i);
          ss = Some(i + 2);
        }
      }
      _ => {
        break;
      }
    }
  }

  //* While Let Loop
  while let Some(i) = ss {
    if i > 19 {
      println!("Quit");
      ss = None;
    } else {
      println!("{}", i);
      ss = Some(i + 2);
    }
  }

  //* Casting
  let ff = 24.32_f32;
  let ii = ff as u8;
  let cc = ii as char;

  //* Result Enum
  enum Result<T, E> {
    Ok(T),
    Err(E),
  }

  let fl = File::open("info.txt");

  let _fl = match fl {
    Ok(file) => file,
    Err(error) => panic!("There was a problem opening the file: {:?}", error),
  };

  println!("{} {} {}", ff, ii, cc);
  println!("{} {} {}", 12 as char, 14 as char, 255 as char);

  //*! HASHMAPS
  let mut hm = HashMap::new();

  hm.insert(String::from("random"), 12);
  hm.insert(String::from("strings"), 49);
  hm.insert(String::from("task"), 50);
  hm.remove(&String::from("strings"));

  for (k, v) in &hm {
    println!("{} : {}", k, v);
  }

  match hm.get(&String::from("random")) {
    Some(&n) => println!("n {}", n),
    _ => println!("no match"),
  }

  //*! VECTORS
  let mut v1: Vec<i32> = vec![1, 2, 3, 4, 5];
  v1.push(8);
  let mut v2 = Vec::new();
  v2.push(12);
  v2.push(20);
  v2.push(30);
  let vs1 = v2.pop();

  if let Some(s) = vs1 {
    println!("vs1 s {}", s);
  } else {
    println!("vs1 s is None")
  }

  println!("vs1 {:#?}", vs1);

  //** Iterate through Vector
  for v in &v2 {
    println!("v {:#?}", v);
  }

  println!("v2 {:#?}", v2);

  //** Vector with Enum Type
  let r = vec![
    Shape::Circle(12.2),
    Shape::Rectangle {
      width: 10,
      height: 20,
    },
    Shape::Square(3),
  ];
  println!("r {:?}", &r);

  //*! B. ENUMS
  let u: Direction = Direction::Left(Point { x: 0, y: 1 });
  let k = u.match_direction();
  let x = k.destruct();
  println!("{:?}", x);

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

  //*! CONCANTENATE STRINGS
  // String + string literal
  let mut ssc1: String = String::from("cool");
  let s1 = "apple";
  ssc1.push_str(s1);
  ssc1.pop();
  println!("ssc1 {}", ssc1);

  // String + string literal
  let mut ssc2: String = "best".to_owned();
  let s2: &str = "friend";
  ssc2.push_str(s2);
  println!("ssc2 {}", ssc2);
  println!("s2 {}", s2);

  // String + string ref
  let mut ssc3: String = "fashanu".to_owned();
  let ssc4: String = "wimbledon".to_owned();
  ssc3.push_str(&ssc4); //  coercion dereference
  println!("ssc4 {}", ssc4);
  println!("ssc3 {}", ssc3);

  //*! OWNERSHIP
  let mut ssb1 = String::from("liverpool");
  let ssb2 = &mut ssb1;
  ssb2.pop();
  println!("ssb2 {}", ssb2); // OK
  println!("ssb1 {}", ssb1);
  // println!("ssb2 {}", ssb2); // Error: cannot borrow `ssb1` as immutable because it is also borrowed as mutable

  let mut ssa1 = String::from("hellow");
  ssa1.pop();
  let ssa2 = &ssa1;
  println!("ssa2 {}", ssa2);
  println!("ssa1 {}", ssa1);

  let ssa3 = &ssa1;
  println!("ssa3 {}", ssa3);
}
