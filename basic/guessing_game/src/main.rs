use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = get_secret_number();
    show_start_messages(secret_number);
    start_game(secret_number)
}

fn get_secret_number() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    return secret_number;
}

fn show_start_messages(secret_number: u32) {
    println!("Guess the number!");
    println!("The secret number is: {}", secret_number);
}

fn start_game(secret_number: u32) {
    let mut is_win = false;
    let mut attempts = 0;
    loop {
        if is_win {
            println!("Attempts: {}", attempts);
            break;
        }

        attempts += 1;

        let guess = read_console_input();
        println!("You guessed: {}", guess);

        let result_data = get_result(guess, secret_number);
        is_win = result_data.is_win;
        println!("{}", result_data.message)
    }
}

fn read_console_input() -> u32 {
    println!("Please input your guess:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    return guess;
}

fn get_result(guess: u32, secret_number: u32) -> ResultData {
    return match guess.cmp(&secret_number) {
        Ordering::Less => ResultData {
            message: "Too small!".to_string(),
            is_win: false,
        },
        Ordering::Greater => ResultData {
            message: "Too big!".to_string(),
            is_win: false,
        },
        Ordering::Equal => ResultData {
            message: "You win!".to_string(),
            is_win: true,
        },
    };
}

struct ResultData {
    message: String,
    is_win: bool,
}
