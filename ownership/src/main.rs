fn main() {
    let s1: String = String::from("hello");
    println!("s1 is {}", s1);
    let s2 = s1;
    println!("s1 moved to s2!");
    //println!("s1 is {}", s1);
    // printstr(String::from("s1 is"), s2);
    printstr_by_ref(String::from("s2 is"), &s2);
    let s3 = takes_and_gives_back(s2);
    printstr(String::from("s3 is"), s3);

    let mut s1: String = String::from("striingggg");
    let _r1 = &mut s1;
    _r1.push_str("fuck");
    println!("r1 is {}", _r1);
    // 同一スコープ内で複数の可変な借用はむり
    //let _r2 = &mut s1;
    // 同一スコープ内で可変と不変を組み合わせた借用はむり
    // 不変で参照しているのに変更されても困る
    //let _r2 = &s1;
}

// 所有権はうつる
fn printstr(msg: String, s: String) {
    println!("{} {}", msg, s);
}

// 参照するので所有権はうつらん
fn printstr_by_ref(msg: String, s: &String) {
    println!("{} {}", msg, s);
    // sはimmutableだから改変はできない
    // s.push_str(" uooooo!");
}

fn takes_and_gives_back(s: String) -> String {
    s
}

// 参照している値はスコープを抜けたらDropされて無効な値になる！！
/*
fn dangle() -> &String {
    let s1: String = String::from("ssss");
    &s1
}*/

// 所有権をmoveさせてやりゃいい
fn no_dangle() -> String {
    let s1: String = String::from("sss");
    s1
}
