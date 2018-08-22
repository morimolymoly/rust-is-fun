fn main() {
    let _vec: Vec<u32> = Vec::new();
    //vec.push(40);
    let mut vec = vec![10, 20, 30];
    vec.push(40);
    vec.push(50);
    // 下のはパニックが起こる(境界チェック)
    //let non = &vec[100];
    {
        let non: Option<&u32> = vec.get(100);
        if let Option::None = non {
            println!("vec[100] is None!");
        }
    }

    for v in &mut vec {
        *v += 10;
    }

    for v in &vec {
        println!("{}", v);
    }
}
