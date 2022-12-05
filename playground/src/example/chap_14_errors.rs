use std::{fs, io};
use rand::prelude::*;

fn main(){
    //unrecoverable_test();
    //recoverable_test();
    //propagate_error();
    higher_lower_challenge();
}

// Errors are part of life in software
// rust has several features to handle runtime errors
// Errors are grouped into two categories: recoverable and un recoverable
// Types of Errors
// - Recoverable
//      Example file not found error
//      Handled with Result<T,E>
// - Unrecoverable
//      Example index beyond array bounds
//      Handled with panic!
//      Immediately terminate the program and provide feedback

fn unrecoverable_test(){
    //panic("Houston, we've had a problem.");

    let countdown =[5,4,3,2,1,0];
    for count in countdown.iter(){
        println!("T-minus {count}");
        let x = 1/count; // this error from the arith.rs file
    }
}
// Rust backtrace env variable $Env:RUST_BACKTRACE=1 // this may differ between operating system

//Recoverable Errors are errors that don not cause the program to fail and can be corrected
//Example: File Not Found error
//Solution ot Recover: prompt to user to select a different file or create the file and file path

//enum Result<T,E> {
// Ok(T),
// Err(E)
//} // included in the prelude

fn recoverable_test(){
    let contents = fs::read_to_string("the_ultimate_question.txt").expect("Nobody know the ultimate question"); // there are better way to handle tho
    println!("content is: {:?}", contents);
}

fn match_result_test(){
    let result = fs::read_to_string("the_ultimate_question.txt");
    let content = match result {
        Ok(message) => message,
        //Err(error) => String::from("Nobody knows the ultimate question!")
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found."),
            io::ErrorKind::PermissionDenied => String::from("Permission denied."),
            _ => panic!("Another type of error: {:?}", error)
        }
    };
    println!("content is: {:?}", content);
}

fn read_and_combine(f1:&str, f2:&str) -> Result<String, io::Error>{
    //the shorthand veirson
    let mut s1 = fs::read_to_string(f1)?;
    let mut s2 = match fs::read_to_string(f2){
        Ok(s) => s,
        Err(e)=> return Err(e)
    };
    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}
fn propagate_error(){
    let result = read_and_combine("plants.txt", "dwarf_planets.txt");
    match result {
        Ok(s) => println!("result is ... \n {}",s),
        Err(e) => println!("there was an error: {e}")
    }
}


fn higher_lower_challenge(){
    let number = thread_rng().gen_range(1..101);
    let mut guess: u32;
    loop{
        let mut buffer = String::new();
        println!("guess my number!");
        let mut error : bool;

        let guess = match io::stdin().read_line(&mut buffer){
            Ok(_) => match buffer.trim().parse::<u32>() {
                Ok(s) => s,
                Err(_) => {
                    println!("answer returned has some input problem! try again!");
                    continue
                }
            },
            Err(_) => {
                println!("answer returned has some parsing problem! try again!");
                continue
            }
        };


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