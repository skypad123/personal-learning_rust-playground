fn main(){
    //scope_test();
    //shadowing_test();
    //string_test();
    //ownership_string_test();
    //ownership_integer_test();
    transferring_ownership_test();
}

// variable scope - where a var is usable
// variable are valid / in scope with in its block of code {}
fn scope_test(){
    let planet = "Earth";
    if true {
        //let planet = "Earth";
        println!("planet is {planet}");
    }
    //planet cannot be used here if declared in the function
    println!("planet is {planet}");
}

// variable Mutability
// var are immutable by default
// but same var can be re declared - process is called shadowing/mask
fn shadowing_test(){
    let planet = "Earth";
    println!("planet is {planet}");
    {
        // this creates an new variable not reassign the variable
        let planet = "Mars";
        println!("planet is {planet}");
        // type is now changeable so all preceding type handling are changed as well
        let mut planet = 4;
        println!("planet is {planet}");
    }
    // shadowing only last as the code block its shadow is declared is present
    // now original value is present.
    println!("planet is {planet}");
}

// Memory -> stack and heap
// stack -> FILO
//    Push and Pop data are very quick and Access data is very quick
//    by very small data space ( known fixed size -> cannot resiz )
// heap -> warehouse
//    less organised, can be anywhere in memory
//    use pointer to locate memory space from stack pointing to heap
//    adding and accessing data is slower than the stack
//    dynamically add and remove data


fn string_test(){
    // string are stored in the heap
    // String Literal
    // "some text" - string literal ( hard-coded into the executable ) -> immutable | must be know at compilation
    // string literal cannot be used to store dynamically changing text like user input
    // String Type
    // Allocated on the heap -> mutable | dynamically generated at runtime

    // path operator :: that lets us used the from function from String Type
    let  mut message = String::from("Earth");
    // field of var message -> ptr; len; cap -> actual array size;
    println!("message is {message}");
    message.push_str(" is home.");
    // if new string append make it more than cap, then new location in heap is used and cap is increased.
    println!("message is {message}");

}

// The heap has plenty of space, but not infinite space.
// there is a need to clean up allocated memory blocks that are no longer needed.

// one way -> Explicit Alloc {malloc()} and Dealloc {free()} c,c++
// Programmer is responsible for memory management
// Pros -> Programmer has lots of control
// Cons -> Memory leaks | Invalid memory access

// 2nd way -> Garbage collector automatically cleans up memory
// Pros -> easy
// Cons -> wasteful memory allocation | collector can run at inconvenient times

// Rust Way -> Ownership
// Variables are responsible for freeing their own resources
// Rules
// 1. Every value is "owned" by one, and only one, variable at a time.
// 2. When the owning variable goes out of scope, value will be dropped and memory freed.
// there can be ownership transfer
// Pros -> Safe | Efficient - determined when value are dropped
// Cons -> Require understanding of ownership

fn ownership_string_test(){
    let outer_planet : String;
    {
        let mut inner_planet = String::from("Mercury");

        // under the hood
        // outer planet's pointer points the same memory space inner planet as pointer # basically a shallow copy
        // but in rust the ptr; len; cap; of inner_planet are "moved" to out planet and cant be used.
        //outer_planet = inner_planet; // move
        //println!("inner_planet is {inner_planet}"); // error -> value borrowed after move

        // to do a deep copy where we duplicate the heap data. to avoid rule of ownership
        outer_planet = inner_planet.clone();
        inner_planet.clear(); // show that inner_planet points to a different heap space
        println!("inner_planet is {inner_planet}");
    }
    println!("outer_planet is {outer_planet}");
}

fn ownership_integer_test(){
    //show the difference between stack and heap
    //here there is no need to clone the data as the equal operator assigns the inner_planet the value of 1 and not a reference
    let outer_planet : i32;
    {
        let mut inner_planet : i32 = 1;
        outer_planet = inner_planet;
        inner_planet += 1;
        println!("inner_planet is {inner_planet}");
    }
    println!("outer_planet is {outer_planet}");
    // Data types with known size that are stored on the stack -> will implement the trait of copy value instead of move when using equal operator =
    // only cloning must be done explicitly.
}

fn transferring_ownership_test(){
    let rocket_fuel = 1;
    // here the integer is pass by value
    process_fuel_integer(rocket_fuel);
    println!("rocket_fuel is {rocket_fuel}");

    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel_string(rocket_fuel);
    println!("rocket_fuel is {rocket_fuel}");
}

fn process_fuel_integer(mut propellant: i32){
    // the value copied is modified not the integer from the parent function
    propellant += 1;
    println!("processing propellant {propellant}...");
}
// propellant here took ownership of the "RP-1" string and will need to transfer back the ownership of the string
//either deep copy with clone() or return the string
fn process_fuel_string(propellant: String) -> String{
    println!("processing propellant {propellant}...");
    //propellant
    let new_fuel = String::from("LNG");
    new_fuel
}