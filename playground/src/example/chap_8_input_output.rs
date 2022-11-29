// Command-Line Arguments
// Arguments passed to the program when it is invoked
// Common uses
// - File paths
// - Configuration settings

//we use the std::env::args
// returns an iterator over arguments passed to the program
// First argument is traditionally the executable path

use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::Write;


fn main(){
    //cmd_args_test();
    //fs_test();
    //write_test();
    roster_challenge();
}
fn cmd_args_test(){
    //check if argument is at least 2
    if env::args().len() <= 2{
        println!("Program requires at least 2 arguments.");
        return;
    }

    for  (index,arg) in env::args().enumerate(){
        println!("argument {index} is {arg}");
    }

    let arg2= env::args().nth(2).unwrap();
    println!("arg2 is {arg2}");
}


fn fs_test(){
    let dirpath = env::current_dir().unwrap();
    println!("the current dir is {}", dirpath.display());
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("contents is {contents}");

    for (index,line) in contents.lines().enumerate(){
        println!("line {index} has the line \"{line}\"");
    }

    let contents = fs::read("planets.txt").unwrap();
    println!("contents is {:?}", contents);

}
// use std::path module allow for cross platform support

fn write_test(){
    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things.\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");

    fs::write("speech.txt", speech);

    let mut file = fs::OpenOptions::new().append(true).open("planets.txt").unwrap();
    file.write(b"\nPluto");

}

// std::fs::write
// simple to use
// Thing to keep in mind
// - write() will replace contents of existing files
// - Write entire content of the file not chunk by chunk

fn roster_challenge(){
    if env::args().len() != 3{
        println!("program requires 2 args.");
        std::process::exit(1);
    }
    let path = env::args().nth(1).unwrap();
    let name = env::args().nth(2).unwrap();

    let content = fs::read_to_string(path).unwrap();
    for list_name in content.lines(){
        if list_name == name{
            println!("name is in file!");
            return
        }
    }
    println!("name is not in file!");
}