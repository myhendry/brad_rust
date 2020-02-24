#![allow(dead_code)]

struct Book {
  title: String,
  price: f32,
}

impl Book {
  fn new(title: &str, price: f32) -> Book {
    Book {
      title: title.to_string(),
      price,
    }
  }

  fn get_details(&self) {
    format!("{} {}", self.title, self.price.to_string());
  }

  fn change_price(&mut self, price: f32) {
    self.price = price
  }
}

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
  println!("Hello");

  let d = Day::Friday;
  // println!("Is d a weekday? {}", d.is_weekday());

  let circle = Shape::Circle(3.2);
  // println!("circle area is {}", circle.area());

  let rect = Shape::Rectangle {
    width: 5,
    height: 7,
  };
  // println!("rectangle area is {}", rect.area());

  //TODO string literals
  let name: &str = "hi. my name is Hendry";
  mover_str(name);
  println!("name: {}", name);

  //TODO String
  let name2 = "hello".to_string();
  mover_string(&name2);
  println!("name2: {}", name2);

  //TODO Numbers
  let num: i32 = 32;
  mover_num(32);
  println!("{}", num);

  //TODO Vectors
  let vec: Vec<i32> = vec![1, 3, 8, 12];
  mover_vec(&vec);
  println!("vec {:?}", vec);

  //TODO Tuples
  let tup = (2, 3, 6, 8);
  mover_tup(tup);
  println!("tup {:?}", tup);

  //TODO Booleans
  let bool1 = true;
  mover_bool(bool1);
  println!("bool {}", bool1);

  //TODO Arrays
  let arr1: [i32; 4] = [22, 55, 77, 88];
  // mover_arr[arr1];
  println!("arr is {:?}", arr1);

  let book = Book {
    title: "Harry Potter".to_string(),
    price: 32.50,
  };
  println!("Book is {} and {}", book.title, book.price);

  let mut book2 = Book::new("Mary has a little lamb", 22.80);
  book2.get_details(); //TODO Not working
  println!("Book 2 is {} and {}", book2.title, book2.price);
  book2.change_price(88.70);
  println!("Book 2 is {} and {}", book2.title, book2.price);
}

// fn mover_arr(arr1: [i32]) {
//   println!("arr is {}", arr1[2]);
// }

fn mover_bool(bool1: bool) {
  println!("bool is {}", bool1);
}

fn mover_tup(tup: (i8, i8, i8, i8)) {
  println!("tup is {}", tup.1);
}

fn mover_vec(vec: &Vec<i32>) {
  println!("vec is {}", vec[1]);
}

fn mover_string(name: &String) {
  println!("name is {}", name);
}

fn mover_str(name: &str) {
  println!("name is {}", name);
}

fn mover_num(num: i32) {
  println!("num is {}", num);
}
