pub fn run() {
  let mut a = 10;
  let _b = a;
  a = 12;
  println!("a {}", a);

  /*
    let i = String::from("hey");
    let j = i;
    println!("i {}", i); //*! */ <- Borrowing error
  */

  let mut c = String::from("hello");
  println!("c1 {}", c);

  /*
    let d = c;
    println!("{}", c); //*! */ <- Borrowing error
  */

  c = String::from("hello2");
  println!("c2 {}", c);

  let f = String::from("anywhere");
  println!("f1 {}", f);

  let g = &f;
  println!("g1 {}", g);

  let e = &mut c;
  println!("e1 {}", e);

  e.pop();
  println!("e2 {}", e);
  // println!("c {}", c); //*! <- Error Immutable Borrow

  {
    e.pop();
    e.pop();

    println!("e3 {}", e);
    //println!("c3 {}", c); //*? Error Immutable Borrow when e4 is NOT commented out
  }

  println!("e4 {}", e); //*? Error Immutable Borrow when c3 is NOT commented out
  println!("c4 {}", c);

  println!("f2 {}", f);
  println!("g2 {}", g);

  let h = &f;
  println!("h1 {}", h);

  let mut aa = vec![1, 2, 3];
  {
    let bb = &mut aa;
    bb.push(5);

    println!("{:?}", bb);
  }
  aa.push(9);

  push_to_vec(&mut aa);
  println!("{:?}", aa);

  println!("area of rectangle {}", calc_rectangle(20, 40));

  let cc = Some(5);
  println!("{:?}", cc.unwrap());

  let s = Some(6);

  match s {
    Some(i) => println!("{}", i),
    None => println!("None"),
  }

  let mut dd = vec![1, 3, 5];
  let ee = test1(&mut dd);
  println!("ee {}", ee);

  test2(&mut dd);
  println!("dd {:?}", dd);

  let ff = &dd;
  println!("ff {:?}", ff);

  let hh = &mut dd;
  hh.pop();
  println!("ee {:?}", hh);
  println!("dd {:?}", dd);

  let gg = &mut dd;
  println!("gg {:?}", gg);
}

fn push_to_vec(v: &mut Vec<i32>) {
  v.push(100);
}

fn calc_rectangle(width: u32, height: u32) -> u32 {
  width * height
}

fn test1(v: &mut Vec<i32>) -> i32 {
  let n = v.pop();
  n.unwrap()
}

fn test2(v: &mut Vec<i32>) -> bool {
  v.push(100);
  true
}

// structs
// tuples
// struct tuple
// enums - options
// enum methods
