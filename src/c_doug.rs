#[allow(unused_variables)]
#[derive(Debug)]
pub struct Hendry {
  field1: i32,
  field2: f64,
  field3: String,
  field4: bool,
  field5: (i32, i32),
}

impl Hendry {
  //*  Associated Functions
  fn new(arg1: i32, arg2: f64, arg3: String, arg4: bool, arg5: (i32, i32)) -> Self {
    Self {
      field1: arg1,
      field2: arg2,
      field3: arg3,
      field4: arg4,
      field5: arg5,
    }
  }

  //* Methods
  fn see(&self) {
    let sf = self.field1;
    println!("{}", sf)
  }
}

pub fn run() {
  println!("{}", "* * * * * * * * * * * * * * * * * *");
  let mut hendry = Hendry {
    field1: 3,
    field2: 5.,
    field3: "hi".to_string(),
    field4: true,
    field5: (3, 6),
  };
  hendry.field2 = 8.;
  hendry.field3.push_str("pooo");
  println!("{:?}", hendry);
  hendry.see();

  let hendry2 = Hendry::new(6, 3., String::from("Hey"), false, (6, 7));
  println!("{:?}", hendry2);
  hendry2.see();
  println!("{}", "* * * * * * * * * * * * * * * * * *");
  let mut d = String::from("Hello");
  println!("{}", d);
  d.push('x');
  println!("{}", d);
  use_mut_var(&mut d);
  d.push('y');
  use_mut_var(&mut d);
  d.push('y');
  println!("{}", d);
  use_immut_var(&d);
  println!("{}", d);
  println!("{}", "* * * * * * * * * * * * * * * * * *");
  let mut c = String::from("Hi");
  let cb1 = &c;
  println!("{}", cb1); //* OK

  let cr1 = use_mut_var(&mut c);
  c.push_str("1");
  println!("{}", c); //* OK
                     // println!("{}", cb1); //*! Error cannot borrow c as mutable because it is also borrowed as immutable
  let cr2 = use_mut_var(&mut c);
  c.push_str("2");
  let cr3 = use_mut_var_2(&mut c);
  c.push_str("3");

  use_immut_var(&c); //* OK
  use_immut_var(&c); //* OK
                     // use_immut_var(&cb1); //*! Error cannot borrow c as mutable because it is also borrowed as immutable
                     // println!("{}", cb1); //*! Error cannot borrow c as mutable because it is also borrowed as immutable
  println!("back in main {:?} {:?} {:?} {}", cr3, cr1, cr2, c);
  c.push_str("4");

  println!("{}", "* * * * * * * * * * * * * * * * * *");
  let b = String::from("Hi");

  println!("back in main {}", b);

  use_var(&b);

  println!("back in main {}", b);
  println!("back in main {}", b);

  println!("{}", "* * * * * * * * * * * * * * * * * *");
  let mut a = String::from("Hi");
  let al = "mate";

  a.push('x');
  println!("{}", a);
  let ac1 = a + al;
  //* a no longer available
  println!("{} {}", al, ac1); //*! If used a here, will get Error Borrow of moved value
}

fn use_immut_var(arg1: &String) {
  println!("{}", arg1);
}

fn use_mut_var(arg1: &mut String) {
  arg1.push_str("aaa");
}

fn use_mut_var_2(arg1: &mut String) {
  arg1.push_str("xxx");
}

fn use_var(arg1: &str) -> bool {
  println!("inside use_var {}", arg1);
  true
}
