extern crate regex;
use regex::Regex;

//TODO REGULAR EXPRESSIONS (REGEX) L35
pub fn run() {
    // to use regex, must create regex struct using new
    let re = Regex::new(r"\w{5}").unwrap();
    let text = "dcode";

    println!("Found match? {}", re.is_match(text));
}
