use std::ops;

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
}
