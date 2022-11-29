use std::io;
//use rand;
//use rand::random;
use rand::prelude::*;
fn main(){
    //io_test();
    //parse_test();
    //use_crate_test();
    higher_lower_challenge();
}

// use Statement
// Bring a module path into scope
// usually located at the top of rust file

//RUST Stand library (std)
// available to all rust programs by default
// the prelude allow developer to use all the most common part of the std library with the use function
// but it does not include the entire Rust Standard Library

//FILE START
// use std::thread;
// thread::spawn(move|| {

//});
//FILE END

// User interaction in cli -> std::io #see top of file

fn io_test(){
    let mut buffer = String::new();
    println!("Enter a message:");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {}", buffer);
}
fn parse_test(){
    let mut buffer = String::new();
    println!("Enter a message:");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {}", buffer);

    //let number = buffer.trim().parse::<i32>(); //remove newline char at the end
    //or use variable type
    let number : i32 = buffer.trim().parse().unwrap(); //remove newline char at the end
    println!("number + 1 is {}",number + 1 );
}

// Crates - collection of rust source code
// 2 Types of crates
// Binary crates compile to produce an executable program
// Library crates contain code for other programs to use
// crates.io -> open source "packages"
// to add open source crates from crates.io
// go to Cargo.toml file
// under the line [dependencies], add crate name and its version example -> rand = "0.8.0"

fn use_crate_test(){
    //let number = rand::random::<f64>();
    let number = random::<f64>();
    println!("number is {number}");

    let number = thread_rng().gen_range(1..11);
    println!("number is {number}");
}
// if random function in import in specifically, it will clash with the local declaration of random
// fn random() -> f64 { 1.0 }

// use rand::prelude::*; can be use to import everything in the rand::prelude:: path; (we can invoking them direct);

fn higher_lower_challenge(){
    let number = thread_rng().gen_range(1..101);
    let mut guess: u32;
    loop{
        let mut buffer = String::new();
        println!("guess my number!");
        io::stdin().read_line(&mut buffer);
        guess = buffer.trim().parse::<u32>().unwrap();
        if guess == number {
            println!("you are correct !");
            break;
        }else if guess > number{
            println!("too high. try again.");
        }else {
            println!("too low. try again.");
        }
    }
}