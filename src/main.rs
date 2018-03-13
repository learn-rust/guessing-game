extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    let secret = rand::thread_rng().gen_range(1, 201);

    loop {
        let mut guess = String::new();

        println!("请输入数字：");

        io::stdin().read_line(&mut guess)
            .expect("读取输入错误！");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("输入的数字是：{}", guess);

        match secret.cmp(&guess) {
            Ordering::Greater => println!("再大一点！"),
            Ordering::Less => println!("再小一点！"),
            Ordering::Equal => {
                println!("恭喜你猜中了！");
                break;
            }
        }
    }
}
