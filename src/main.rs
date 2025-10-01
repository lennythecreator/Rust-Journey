use std::io;

fn main() {
    println!("Let's guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fialed ti read line");
    
    println!("You guessed: {guess}");
}
