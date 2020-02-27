pub fn run() {
  let c = true;

  //*! IF-ELSE
  let n = if c { 50 } else { 76 }; // types must be compatible and c must resolve to a boolean

  println!("n: {}", n);

  //*! LOOPS
  let mut d = 0;

  loop {
    println!("finite");
    d += 1;

    if d >= 3 {
      break;
    }
  }

  /*
  'a: loop {
    println!("loop a");
    'b: loop {
      println!("loop b");
      'c: loop {
        println!("loop c");

        break 'b;

        // if true {
        //   continue;
        // } else {
        //   break;
        // }
      }
    }
    continue 'a;
  }
  */

  //*! WHILE
  let mut n = 5;

  while n != 0 {
    println!("while {}", n);

    n = n - 1;
  }

  //*! FOR LOOPS
  let a = vec![10, 20, 30];

  for i in a {
    println!("for loop: {}", i);
  }

  for i in 1..6 {
    println!("for loop: {}", i);
  }

  //*! MATCH & PATTERN MATCHING
  // Example 1
  let x = 5;

  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("others"),
  }

  // Example 2
  let z = 15;

  match z {
    1 => println!("One"),
    2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    13..=19 => println!("A teen"),
    _ => println!("Ain't special"),
  }

  // Example 3
  let pair = (0, -2);

  match pair {
    (0, y) => println!("y: {}", y),
    (x, 0) => println!("x: {}", x),
    _ => println!("No match"),
  }

  // Example 4
  let pair2 = (5, -5);

  match pair2 {
    (x, y) if x == y => println!("Equal"),
    (x, y) if x + y == 0 => println!("Equal zero"),
    (x, _) if x % 2 == 0 => println!("X is even"),
    _ => println!("No match"),
  }

  // Example 5
  let pp = 5;

  match pp {
    u @ 1..=12 => println!("u: {}", u), // n @ allows binding result to variable
    u @ 13..=19 => println!("u: {}", u),
    _ => println!("no match"),
  };

  // Example 6
  let p = 8;

  let w = match p {
    w @ 1..=12 => w, // n @ allows binding result to variable
    w @ 13..=19 => w,
    _ => 0,
  };

  println!("w: {}", w);
}
