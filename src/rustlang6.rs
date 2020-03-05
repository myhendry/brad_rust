use std::fmt;
use std::ops;
use std::ops::Mul;

//*! CLONE & COPY
/*
Both Copy and Clone do duplicate objects.  With Clone you are using a implementation
that can create a new version of that object explicitly. Where as with Copy,
you are basically telling the compiler that you want to duplicate an object
but you don't want to have it be counted as a "move".  Copy triggers a memcpy;
when you reassign a value or pass a argument by value to a function it is always a memcpy.
With clone you can copy any type but with copy you can only copy a type that takes a memcpy (aka primitives).
Copys are less performant when it comes to more complex objects. This is mainly why you see people passing
around data by reference rather then data by value in Rust code.  Clones are pretty efficient but not as efficient as using references.
*/
#[derive(Debug, Clone, Copy)]
struct A(i32);
#[derive(Debug, Clone, Copy)]
struct D(i32);
// #[derive(Eq, PartialEq, PartialOrd, Ord)]
struct B(f32);

//*! STRUCTS ADD
struct X;
struct Y;
#[derive(Debug)]
struct XY;
#[derive(Debug)]
struct YX;

impl ops::Add<Y> for X {
  type Output = XY;

  fn add(self, _rhs: Y) -> XY {
    XY
  }
}

impl ops::Add<X> for Y {
  type Output = YX;

  fn add(self, _rhs: X) -> YX {
    YX
  }
}

//*! DROP
struct Z {
  z: String,
}

impl Drop for Z {
  fn drop(&mut self) {
    println!("dropped {}", self.z)
  }
}

//*! TRAITS
trait Shape {
  fn area(&self) -> u32;
}

struct Rectangle {
  x: u32,
  y: u32,
}

struct Circle {
  radius: f64,
}

impl Shape for Rectangle {
  fn area(&self) -> u32 {
    self.x * self.y
  }
}

impl Shape for Circle {
  fn area(&self) -> u32 {
    (3.141 * self.radius * self.radius) as u32
  }
}

//*!
struct Fib {
  c: u32,
  n: u32,
}

impl Iterator for Fib {
  type Item = u32;

  fn next(&mut self) -> Option<u32> {
    let n = self.c + self.n;
    self.c = self.n;
    self.n = n;

    Some(self.c)
  }
}

fn fib() -> Fib {
  Fib { c: 1, n: 1 }
}

//*! GENERICS WITH STRUCTS
struct Square<T> {
  x: T,
}

//*! GENERICS WITH FUNCTIONS
fn p<T: fmt::Debug>(x: T) {
  println!("{:?}", x);
}

//*! GENERICS WITH IMPLEMENTATION BLOCK
struct AA<T> {
  x: T,
}

impl<VV> AA<VV> {
  fn item(&self) -> &VV {
    &self.x
  }
}

//*! GENERICS PATTERN MATCHING TYPE
struct DD<U, V> {
  x: U,
  y: V,
}

struct EE<V> {
  x: V,
  y: V,
}

//*! GENERICS WITH TRAITS
trait GShape<T> {
  fn area(&self) -> T;
}

struct GRectangle<T: Mul> {
  x: T,
  y: T,
}

impl<T: Copy> GShape<T> for GRectangle<T>
where
  T: Mul<Output = T>,
{
  fn area(&self) -> T {
    self.x * self.y
  }
}

// Alternative Syntax - same as above
/*
impl<T: Mul<Output = T> + Copy> GShape<T> for GRectangle<T>
{
  fn area(&self) -> T {
    self.x * self.y
  }
}
*/

pub fn run() {
  //*! USING TRAITS
  let c = Circle { radius: 100.132 };
  let r = Rectangle { x: 30, y: 20 };
  println!("{} {}", c.area(), r.area());

  //*! USING CLONE & COPY
  let a = A(32);
  let b = B(12.13);
  let c = a.clone(); // using CLONE trait
  println!("{:?}", a);

  let d = D(25);
  let e = d; // using COPY trait
  println!("{:?}", d);

  //*! USING STRUCTS ADD
  println!("{:?}", X + Y);
  println!("{:?}", Y + X);

  //*! USING DROP
  let zz = Z {
    z: String::from("ZZ"),
  };
  {
    let yy = Z {
      z: String::from("YY"),
    };

    {
      let xx = Z {
        z: String::from("XX"),
      };
      println!("leaving inner scope 2");
    }
    println!("leaving inner scope 1");
  }
  println!("program ending");

  for j in fib().take(10) {
    println!("{}", j);
  }

  for j in fib().skip(14).take(10) {
    println!("{}", j);
  }

  let mut f = fib();

  println!("{:?}", f.next());
  println!("{:?}", f.next());
  println!("{:?}", f.next());
  println!("{:?}", f.next());
  println!("{:?}", f.next());

  //*! USING GENERICS WITH STRUCTS
  let s = Square { x: 10 };
  let s = Square { x: 1.0 };
  let s = Square { x: "Hello" };
  let s = Square { x: 'c' };

  //*! USING GENERICS WITH FUNCTIONS
  p(10);
  p(String::from("String!"));

  //*! USING GENERICS WITH IMPLEMENTATION BLOCK
  let vv = AA { x: "Hello" };

  vv.item();

  //*! USING GENERICS WITH TRAITS
  let r1 = GRectangle { x: 10, y: 20 };
  let r2 = GRectangle { x: 10.10, y: 20.21 };

  println!("{} {}", r1.area(), r2.area())
}
