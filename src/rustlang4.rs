use std::collections::HashMap;
use std::fs::File;

pub fn run() {
  //TODO Vectors
  let x = vec![1, 2, 3, 4];
  let mut v: Vec<i32> = Vec::new();
  let mut w: Vec<i32> = Vec::new();

  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  for i in &v {
    println!("{}", i);
  }

  v.push(10);
  println!("v {:?} {} {}", &v, v.len(), v.capacity());
  println!("v {:?}", v.pop());

  println!("w {:?} {} {}", &w, w.len(), w.capacity());
  println!("w {:?}", w.pop());

  #[derive(Debug)]
  enum Example {
    Float(f64),
    Int(i32),
    Text(String),
  }

  let r = vec![
    Example::Int(142),
    Example::Float(12.32),
    Example::Text(String::from("string")),
  ];

  println!("{:?}", &r);

  //TODO HashMap
  let mut hm = HashMap::new();

  hm.insert(String::from("random"), 12);
  hm.insert(String::from("strings"), 49);
  hm.remove(&String::from("strings"));

  for (k, v) in &hm {
    println!("{} : {}", k, v);
  }

  match hm.get(&String::from("random")) {
    Some(&n) => println!("{}", n),
    _ => println!("no match"),
  }

  let s = Some('c');

  match s {
    Some(i) => println!("{}", i),
    _ => {}
  }

  if let Some(i) = s {
    println!("{}", i);
  } else {
    {}
  }

  let mut g = Some(0);

  loop {
    match g {
      Some(i) => {
        if i > 19 {
          println!("Quit");
          g = None;
        } else {
          println!("{}", i);
          g = Some(i + 2);
        }
      }
      _ => {
        break;
      }
    }
  }

  while let Some(i) = g {
    if i > 19 {
      println!("Quit");
      g = None;
    } else {
      println!("{}", i);
      g = Some(i + 2);
    }
  }

  let f = 24.243243_f32;
  let i = f as u8;
  let c = i as char;

  println!("{} {} {}", f, i, c);
  println!("{}", 255 as char);
  println!("{} {}", 12 as char, 14 as char);

  enum Result<T, E> {
    Ok(T),
    Err(E),
  }

  let ff = File::open("test.txt");

  let ff = match ff {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error);
    }
  };
}
