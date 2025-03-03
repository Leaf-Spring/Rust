fn main() {
    let mut string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    string1 = longest(string1.as_str(), string2.as_str()).to_string();
    println!("The longest string is {string1}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
