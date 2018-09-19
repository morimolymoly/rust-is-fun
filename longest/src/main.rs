fn main() {
    let s = String::from("long");
    let s2 = String::from("longlong");
    println!("{}", longest(s.as_str(), s2.as_str()));
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
