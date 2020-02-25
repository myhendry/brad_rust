use std::fmt;

#[derive(Debug)]

struct Object {
  width: u32,
  height: u32,
}

// Related Function
impl Object {
  fn new(width: u32, height: u32) -> Object {
    Object { width, height }
  }
}

// Methods
impl Object {
  fn area(&self) -> u32 {
    return self.width * self.height;
  }

  fn show(&self) {
    println!(
      "object width {} height {} area {}",
      self.width,
      self.height,
      self.area()
    );
  }
}

impl fmt::Display for Object {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(
      f,
      "({}, {} and area {})",
      self.width,
      self.height,
      self.area()
    );
  }
}

pub fn run() {
  //*! STRUCTS
  let o1 = Object {
    width: 20,
    height: 10,
  };
  println!(
    "object width {} height {} area {}",
    o1.width,
    o1.height,
    o1.area()
  );

  let o2 = Object::new(27, 20);
  println!(
    "object width {} height {} area {}",
    o2.width,
    o2.height,
    o2.area()
  );

  let o3 = Object::new(570, 200);
  o3.show();

  println!("{:?}", o3);
  println!("{:#?}", o3);
  println!("{}", o3);
}
