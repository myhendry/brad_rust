use std::fs::File;
use std::io::prelude::*;

// MULTIPLE MODS L34
mod b_arrays;
mod b_cli;
mod b_conditionals;
mod b_enums;
mod b_functions;
mod b_loops;
mod b_pointer_ref;
mod b_print;
mod b_strings;
mod b_structs;
mod b_tuples;
mod b_types;
mod b_var;
mod b_vectors;
mod cli;
mod input;
mod my_enum;
mod my_hash;
mod my_http;
mod my_match;
mod my_modules;
mod my_option;
mod my_rand;
mod my_regex;
mod my_str;
mod my_traits;
mod playground;
mod rustlang3;
mod rustlang4;

#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct FavColor(u8, u8, u8);

// Struct with impl
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {} and I am {}", self.name, self.age);
    }
}

fn main() {
    // rustlang4::run();
    // rustlang3::run();
    playground::run();
    // b_print::run();
    // b_var::run();
    // b_types::run();
    // b_strings::run();
    // b_tuples::run();
    // b_arrays::run();
    // b_vectors::run();
    // b_conditionals::run();
    // b_loops::run();
    // b_functions::run();
    // b_pointer_ref::run();
    // b_structs::run();
    // b_enums::run();
    // b_cli::run();
    // my_http::run();
    // my_enum::run();
    // my_option::run();
    // my_modules::run();
    // my_regex::run();
    // my_str::run();
    // my_rand::run();
    // my_hash::run();
    // cli::run();
    // my_traits::run();
    // my_match::run();
    // input::run();
    //  single line comment
    /*
        multi line comment
        multi line comment
    */
    /*
        u8: 0 - 255
    */
    let i: i32 = 5;
    let f: f32 = 3.6;
    let b: bool = false;
    let d: Direction = Direction::Left;
    const MAX_NUMBER: u32 = 9;
    let t: (u32, f32, u32, (i32, i32)) = (1, 2.3, 5, (3, 7));

    // OWNERSHIP, MOVING, BORROWING
    let x: u32 = 32;
    print_primitive(x);
    println!("primitive {}", x);
    let str_x = String::from("x org");
    print_reference(str_x);
    // println!("reference string {}", str_x);  // Error borrow of moved value: `str_x` value borrowed here after move
    let str_y = String::from("y org");
    print_reference_again(&str_y);

    // READ FILE IN RUST L25
    let mut file = File::open("info.txt").expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Oops! Can not read the file...");
    println!("File contents:\n\n{}", contents);

    // VECTORS L24
    let my_vector: Vec<i32> = Vec::new();
    println!("{:?}", my_vector);
    let mut my_vector2 = vec![1, 2, 3, 4];
    my_vector2.push(59);
    my_vector2.remove(1); // use element index
    println!("{}", my_vector2[2]);
    for number in my_vector2.iter() {
        println!("{}", number);
    }

    // IMPLEMENTING TRAITS L23
    let dom = Person {
        name: String::from("Dominic"),
        age: 21,
    };
    println!("{}", dom.to_string());

    // USING STRING L22
    let mut my_string = String::from("Hello World");
    // length
    println!("Length: {}", my_string.len());
    // is Empty?
    println!("String is empty? {}", my_string.is_empty());
    // split string by white space
    for token in my_string.split_whitespace() {
        println!("{}", token);
    }
    // does string contains world
    println!(
        "Does string contains world? {}",
        my_string.contains("World")
    );
    // append string
    my_string.push_str("hey my friend");
    println!("{}", my_string);

    // USING STRUCT IMPL L21
    let rect = Rectangle {
        width: 20,
        height: 50,
    };
    rect.print_description();
    println!("Rectangle is a square: {}", rect.is_square());

    // USING ARRAY L20
    let a: [i32; 5] = [1, 2, 3, 4, 6];
    println!("array {:?}", a[1]);
    for n in a.iter() {
        println!("array items {}", n);
    }
    for i in 0..a.len() {
        println!("arr length item {}", a[i]);
    }
    // Create an array with 50 items of 2 and println!
    // let ab = [2; 50];
    // for i in 0..ab.len() {
    //     println!("ab {}", ab[i]);
    // }

    // USING STRUCTS L17
    let mut bg = Color {
        red: 255,
        green: 70,
        blue: 15,
    };
    // cannot change struct unless make struct mut
    bg.blue = 200;
    println!("{} {} {}", bg.red, bg.green, bg.blue);

    // USING TUPLE STRUCT L18
    let mut red = FavColor(255, 0, 0);
    // cannot change struct unless make struct mut
    red.2 = 60;
    println!("red is {} {} {}", red.0, red.1, red.2);

    // USING REFERENCES T16
    let mut m = 10;
    // mr is a mutable reference of m
    let mr = &mut m;
    *mr += 1;
    println!("m is {}", m);

    // PASS BY REFERENCE STRUCT L19
    print_color(&bg);

    // using enums
    match d {
        Direction::Up => println!("Going Up"),
        Direction::Down => println!("Going Down"),
        Direction::Left => println!("Going Left"),
        Direction::Right => println!("Going Right"),
    }

    // using constants
    for n in 1..MAX_NUMBER {
        println!("{}", n)
    }

    // accessing nested tupple
    println!("{}", (t.3).0);

    // using function with NO return type
    shout(5);

    // using function with return type
    let res: bool = is_even(21);

    // println! primitive values
    println!("{} {} {} {} {}", i, f, b, MAX_NUMBER, res);

    // println! tuple
    println!("{:?}", t);
}

// function with primitive as args
fn shout(num: u32) {
    for n in 1..num {
        println!("{}", n)
    }
}

// using function with return type
fn is_even(num: u32) -> bool {
    num % 2 == 0
}

// PASS BY REFERENCE STRUCT L19
fn print_color(c: &Color) {
    println!("Color - R:{} G:{} B:{}", c.red, c.green, c.blue);
}

fn print_primitive(t: u32) {
    println!("Primitive Copy {}", t)
}

fn print_reference(t: String) {
    println!("Reference {}", t)
}

fn print_reference_again(t: &String) {
    println!("Reference {}", t)
}
