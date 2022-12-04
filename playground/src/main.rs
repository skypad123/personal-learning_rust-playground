fn main(){
    //borrow_checker_test();
    lifetime_annotation_test();
}

// Borrow Checker - Compiler 's tool to check borrowing
fn borrow_checker_test(){
    let propellant;
    let rp1 = String::from("RP-1"); // this is acceptable as lifetime is now the same as line 13
    {
        //let rp1 = String::from("RP-1"); // this disappear after the code block
        propellant = &rp1;
    }
    println!("propellant is {}", propellant);
}

fn best_fuel<'a>(x:&'a str, y:&'a str)-> &'a str{
    //returning str borrow check freaks out cos it does not know which the borrowed values will be used

    //'a is a lifetime Annotation
    // explicitly defines a generic lifetime for parameters
    // must begin with an apostrophe
    // tell the relation of the lifetime between the parameters ( will always use the lesser of all compared)
    if x.len() > y.len() { x } else { y }
}

fn lifetime_annotation_test(){
    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNG");
        result = best_fuel(&propellant1, &propellant2);
    }
    println!("result is {}", result);
}