fn say_hello() -> String {
    "Hello, World".to_string()
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn print_hello(name: String) {
    println!("Hello, {}", name);
}

fn main() {
    println!("Hello, world!");
    let cha = '👌';
    println!("{}", cha);
    let shit: (String, char, u32) = ("あああ".to_string(), '👌', 229);
    let (_, b, _) = shit;
    println!("{}", b);
    println!("{}", shit.1);

    let msg = say_hello();
    println!("{}", msg);
    println!("5+10={}", add(5, 10));

    let x = {
        let y = 20;
        println!("value y is {}", y);
        40
    };
    println!("value x is {}", x);

    {
        let block = "blobloblo".to_string();
        println!("inside block {}", block);
    }
    //println!("{}", block);

    let tup: (String, i32) = ("MMM".to_string(), 20);
    println!("{}",tup.0);

    print_hello("melo".to_string());

    for num in 0..5 {
        println!("{}", num);
    }
}
