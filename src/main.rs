use std::io;

fn main() {
    println!("Let's guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    let answer = "Wizard";
    let _ =io::stdin()
        .read_line(&mut guess);
        if guess.trim() == answer.to_string() {
            println!("You win!");
        } else {
            println!("You lose! The answer is {answer}");
        }
        
    
    println!("You guessed:{}", guess);
}
