use std::io;
use rand::{Rng};

fn main() {
    println!("Let's guess the number!");
    println!("Please input your guess.");


    
    let mut chances = 3; 
    let answers = ["Wizard","Witch","Warlock","Warrior","Warrior King"];

    let mut rng = rand::rng();
    let round_answer = answers.len();
    let answer_index = rng.random_range(0..round_answer);
    let answer:&'static str  = answers[answer_index];

    loop{
        let mut guess = String::new();
        if chances == 0 {
            println!("You lose! The answer is {answer}");
            break;
        }
         let _ =io::stdin()
        .read_line(&mut guess);
        if guess.trim() == answer.to_string() {
            println!("You win!");
            break;
        } if guess.trim() != answer.to_string() && chances > 0 {
            chances -= 1;
            println!("Wrong! Try again.");
            println!("You have {chances} chances left.");
        } else {
            println!("You lose! The answer is {answer}");
        }
        
    
        println!("You guessed:{}", guess);
    }
   
}
