fn main() {
    let s = String::from("long");
    let s2 = String::from("longlong");
    println!("{}", longest(s.as_str(), s2.as_str(), "fuck you"));
}

fn longest<'a, T>(x: &'a str, y: &'a str, msg: T) -> &'a str
    where T: std::fmt::Display
{
    println!("{}", msg);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
