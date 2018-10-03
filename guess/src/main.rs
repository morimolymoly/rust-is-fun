extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) ->  Guess {
        if value < 1 || value > 100 {
            panic!("数字は1から100までの整数にしてください！！")
        }
        Guess {
            value
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[should_panic]
    fn new_invalid_guess() {
        Guess::new(400);
    }
}

fn main(){
    println!("数あてゲーム!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("秘密の数字: {}", secret_number);
    loop {
        println!("予想を入力して！");

        let mut guess_str = String::new();

        io::stdin().read_line(&mut guess_str)
            .expect("読み込み失敗!");

        let guess: Guess = match guess_str.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("数字を入力してください！！");
                continue
            },
        };

        println!("予想した数字: {}", guess.value);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("小さスギィッ!!"),
            Ordering::Greater => println!("イキスギィッ!!"),
            Ordering::Equal => {
                println!("正解!!");
                break;
            }
        }
    }
    println!("完")
}
