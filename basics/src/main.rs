use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Hello, world!");
    
    // Generate a number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    
    loop {
        // Read user input
        println!("Now, enter an integer guess...");
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    
        // Shadowing allows redefinitions; they are scoped
        // let input :u32 = input.trim().parse().expect("You did not enter an integer.");
        let input : u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("This is not a number. Try again.");
                continue;
            },
        };
        
        println!("You entered: {}", input);
    
        // Switch case on input.cmp evaluated value
        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Just nice!");
                break;
            }
        }

    }


}
