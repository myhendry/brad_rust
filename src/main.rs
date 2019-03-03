#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    //  single line comment
    /*
        multi line comment
        multi line comment
    */
    let i: i32 = 5;
    let f: f32 = 3.6;
    let b: bool = false;
    let d: Direction = Direction::Left;
    const MAX_NUMBER: u32 = 9;
    let t: (u32, f32, u32, (i32, i32)) = (1, 2.3, 5, (3, 7));

    // USING REFERENCES T16
    let mut m = 10;
    // mr is a mutable reference of m
    let mr = &mut m;
    *mr += 1;
    println!("m is {}", m);

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

fn is_even(num: u32) -> bool {
    num % 2 == 0
}
