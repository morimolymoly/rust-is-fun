extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("数あてゲーム!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("秘密の数字: {}", secret_number);

    println!("予想を入力して！");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("読み込み失敗!");

    let guess: u32 = guess.trim().parse()
        .expect("数字を入力してください!");

    println!("予想した数字: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("小さスギィッ!!"),
        Ordering::Greater => println!("イキスギィッ!!"),
        Ordering::Equal => println!("正解!!"),
    }
}
