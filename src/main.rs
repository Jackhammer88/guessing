extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Игра \"Угадай число\"");

    let secret_number = rand::thread_rng()
        .gen_range(1, 101);

    let mut total_attempts = 1;

    loop {

        println!("Введите ваше предположение:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Не удалось прочитать строку.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Ваша попытка {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Слишком маленькое!"),
            Ordering::Greater   => println!("Слишком большое!"),
            Ordering::Equal     => {
                println!("Вы выиграли с {} попытки!", total_attempts);
                break;
            }
        }

        total_attempts += 1;
    }
}
