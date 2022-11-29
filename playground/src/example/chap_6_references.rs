fn main(){
    // borrowing_reference_test();
    // mutable_reference_test();
    // dangling_reference_test();
    //slice_test();
    //slice_parameter_test();
    trim_challenge();
}

fn borrowing_reference_test(){
    let rocket_fuel = String::from("RP-1");
    let  length = process_fuel_string_1(&rocket_fuel);
    println!("rocket_fuel is {rocket_fuel} and length is {length}");
}
// Borrowing
// Access data without taking ownership of it
// Create references using the borrow operator &
// now propellant is returning a reference to String in the parent function
fn process_fuel_string_1(propellant: &String) -> usize {
    println!("processing propellant {propellant}...");
    let length: usize = propellant.len();
    length
}

// Changing value of a a borrowed variable (use reference)
// When using a mutable reference, you cannot create any other references -> even not immutable
// this prevents data races
fn mutable_reference_test(){
    let mut rocket_fuel = String::from("RP-1");
    let  length = process_fuel_string_2(&mut rocket_fuel);
    println!("rocket_fuel is {rocket_fuel} and length is {length}");
}

fn process_fuel_string_2(propellant: &mut String) -> usize {
    println!("processing propellant {propellant}...");
    propellant.push_str(" is highly flammable!");
    let length: usize = propellant.len();
    length
}

fn dangling_reference_test(){
    let rocket_fuel = produce_fuel();
    //println!("rocket_fuel is {rocket_fuel}");
}

fn produce_fuel() -> String{
    let new_fuel = String::from("RP-1");
    // new_fuel string lifespan is shorter than the return references -> error
    //&new_fuel
    //correct way -> no dangling references
    new_fuel
}

// Slice
// Reference to a contiguous section of a collection
// Commonly encountered as the string slice  data type: &str
// String literals are slices

fn slice_test(){
    let message = String::from("Greetings from Earth!");
    println!(" message is {message}");

    //allow the reference to just to indexed elements in the array
    let last_word = &message[15..15+5];
    println!("last word is : {last_word}");
    // when specifying len, if len specified is longer, runtime will exit with panic as memory accessed is out of bound
    // len can be dynamically adjusted to the end of the array; Do the following
    let last_word = &message[15..];
    println!("last word is : {last_word}");
    // Additional Info String Slices
    // the len in the space specify the number of bytes not chars ( and chars in the string are encoded in utf-8 so they can span multiple bytes)
    // remember to specify range indices that occur at valid utf-8 char boundaries, else program will panic

    let planets = [1,2,3,4,5,6,7,8];
    let inner_planets = &planets[..4];
    println!("inner_planets are {:?}", inner_planets);

}

fn slice_parameter_test(){
    let message = String::from("Greetings from Earth!");
    //let first_word = get_first_word(&message); //&String
    let first_word = get_first_word(&message[10..]); //&str
    println!(" first_word is {first_word}");
}
//&String vs &str
//&String's ptr -> String's ptr -> Address in Heap
//&str's ptr -> Address in heap

// &String can be used in &str place due to Deref Coercion as the String that it position to is implements str
// But &str cannot be used in &String place

//use &str type for parameter instead of &String to use Deref Coercion
fn get_first_word(s: &str)-> &str {
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space
        }
    }
    &s // if no space is found
}

fn trim_challenge(){
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");

    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

fn trim_spaces(s:&str)-> &str {
    let bytes = s.as_bytes();
    let mut start: usize = 0;
    let mut end: usize = if bytes.len() != 0 { bytes.len() - 1} else {0} ;
    while start < bytes.len() && bytes[start] == b' ' {
        start += 1;
    }
    while end > 0 && bytes[end] == b' ' {
        end -= 1;
    }
    return if start>= end {&s[0..0]} else {&s[start..end+1]};

}