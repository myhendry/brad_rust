pub fn run() {
  // PRIMITIVE TYPES - implement COPY TRAITS
  // integer: i8, u8, i16, u16, i32, u32, i64, u64, isize, usize
  // float: f32, f64
  // bool: true, false
  // char
  // slices
  // tuple containing primitive: (i32, f64, char) - cannot loop over tuple because possibility of different element types inside
  // fixed size array: [1,2,3,4,5,6] - can loop over an array
  // functional pointers

  // OWNERSHIP RULES
  // 1 Each value has a variables which is its owner
  // 2 There can only be one owner at any given time
  // 3. When the owner goes out of scope the value will be dropped out of memory

  // BORROWING RULES
  // 1 Allow infinite borrows for readonly access
  // 2 Readonly borrows make the original data immutable for their duration
  // 3 Only allowed to pass one borrow at a time for write access/mutability

  let x: u32 = 5;
  println!("{}", x);

  let t: (i32, f64, char) = (42, 6.12, 'j');
  let (_, _, x) = t;
  println!("{}", x); // returns 'j'

  let arr = [1, 2, 3, 4, 5, 6];
  let a1 = arr[2];
  println!("{}", a1); // returns 3

  for item in arr.iter() {
    println!("{}", item);
  }

  // Ownership & Borrowing
  // Now rust supports non-lexical lifetimes
  let mut a = String::from("A String");

  let b = &a; // readonly borrow can be borrowed multiple times
  let c = &a;
  let d = &a;
  println!("b: {}, c: {}, d: {}", b, c, d);

  a.pop();

  let x = &mut a; // only can borrow mutable data once
  x.pop();
  println!("a: {}", a);

  let mut aa = 10;

  let dd = &mut aa;
  *dd += 1;

  println!("aa {}", aa);
}
