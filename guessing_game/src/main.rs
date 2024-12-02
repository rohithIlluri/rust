use std::io; //user input 
use rand::Rng;  //Rng from rand to create a random variable
use std::cmp::Ordering;  //Ordering to cmp(compare) value

fn main() {
    println!("Welcome");
    println!("Guess the number between 1 and 100 !");

    //generate a rand number between 1 and 100
    let rand_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };
        println!("you guessed: {}",guess);

    match guess.cmp(&rand_num){
        Ordering::Less => println!("Too small !"),
        Ordering::Greater => println!("Too Big, try a smaller number"),
        Ordering::Equal => {
            println!("You Win");
            break;
        }
    }
}
}
