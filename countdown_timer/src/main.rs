use std::{io, thread, time::Duration};

fn main() {
    println!("Welcome to the countdown Timer!");
    println!("Enter the number of seconds for the countdown");
// creating a mut input and storing the input in a string and read the input. 
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to reathe line");

//Parse the input into a number(u64 is used for larger range of seconds).
// attempt to convert the input string into unsigned 64 bit integer, if Ok then store the value in seconds,
//if Err then  print the error message 
    let seconds: u64 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };
//call the countdown function with 'seconds' as value 
    countdown(seconds);
}

//Countdown function 
fn countdown(seconds: u64){
    //for loop counting from seconds to 1
    for i in (1..=seconds).rev(){
        println!("{}",i);
        //pause for 1 second between each number 
        thread::sleep(Duration::from_secs(1));
    }
    println!("Time's up!");
}
