extern crate core;

use std::collections:: HashMap;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::Write;

fn main(){
    //vector_test();
    //hashmap_test();
    count_common_words_challenge();
}

//Vector Collection
//Vec<T> Data Type
// Collection of elements with the same data types
// Elements are stored in order
// Items can be dynamically added and removed
// Stored on the heap

fn vector_test(){
    let mut astronauts: Vec<String>= Vec::new();
    astronauts.push(String::from("Shepard"));
    astronauts.push(String::from("Grissom"));
    astronauts.push(String::from("Glenn"));
    println!(" astronauts is {:?}", astronauts);
    let last = astronauts.pop();
    println!("last is {:?}", last);
    let third = &astronauts[2];
    println!("third is {:?}", third);
    let countdown = vec![5,4,3,2,1];
}

//Hashmap Collection
//HashMap<k,V> Data Type
// Stores data in key -> value pairs
// Use keys to lookup corresponding values
// Key -> value mapping is one way
// Uses a hash function to determine how to store data
// Keys and value can be different types but all key are the same type and values must have the same data types
// Each key can only have one value associated with it at a time

fn hashmap_test(){
    let mut missions_flown = HashMap::new();
    missions_flown.insert("Hadfield", 3);
    missions_flown.insert("hurley", 3);
    missions_flown.insert("Barron", 0);

    missions_flown.insert("Barron", 1); // Overwrite
    missions_flown.entry("Stone").or_insert(2); // insert if the key dont exist
    let kayla = missions_flown.entry("Barron").or_insert(0);
    *kayla += 1;

    println!("mission_flown is {:?}", missions_flown);
    let barron_missions = missions_flown.get("Barron");
    println!("barron_missions is {:?}", barron_missions);

    // 3 common way to update hashmap
    // 1. Overwrite an existing key-value pair.
    // 2. Insert a new entry if a key does not exist.
    // 3. Modify a value based on its existing value.
}


fn count_common_words_challenge(){
    // challenge goals
    // 1. Read in a text file.
    //  Accept the path to a text file via command-line arguement
    //  implement error handling as needed
    let mut file_string: String = String::from(read_text_from_file());

    // 2. Count the number of time each word occurs.
    //  Parse text into individual words ( use split_whitespace() method)
    //  Ignore Capitalisation
    let mut word_count = text_array_modification(&file_string);

    // 3. Print a message with the most common words and how many time they appeared.
    //  Keep track of how many times each unique word occurs
    //  Print most common words and how many time they occurred
    //  Keep in mind, there could be multiple "most common" words

    let mut top_count = 0u32;
    let mut top_words : Vec<&str> = Vec::new();
    for (&key, &val) in word_count.iter(){
        if val > top_count{
            top_count = val;
            top_words.clear();
            top_words.push(key);
        }else if val == top_count{
            top_words.push(key);
        }
    }

    println!("Top word(s) occurred {top_count} times:");
    for word in top_words.iter() {
        println!("{word}");
    }


}

fn read_text_from_file() -> String {
    if env::args().len() != 1{
        println! ("program requires path to file.")
    }
    let path = match env::args().nth(1) {
        Some(s) => s,
        None => panic!("path string cannot be found.")
    };
    let file_content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => {
            println!("file cannot be converted to string");
            String::from("")
        }
    };
    return file_content.to_lowercase();
}

fn text_array_modification( base_str: &str) -> HashMap<&str,u32>{
    let mut word_count = HashMap::new();
    let word_coll = base_str.split_whitespace().collect::<Vec<&str>>();
    for &item  in word_coll.iter(){
        //println!("{item}");
        let count = word_count.entry(item).or_insert(0);
        *count += 1;
    }
    return word_count;
}
