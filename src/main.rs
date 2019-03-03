fn main() {
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);

    let a = String::from("Hello");
    let b = &a;
    println!("{}, {}", a, b);

    let c = check();
    println!("{}", c)
}

fn check() -> String {
    let s = String::from("hey");
    s
}

// fn pass(c: u32) -> u32 {
//     println!("{}", c);
//     c;
// }

// return void?
