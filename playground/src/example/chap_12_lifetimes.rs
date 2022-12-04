fn main(){
    //borrow_checker_test();
    //lifetime_annotation_test();
    multi_lifetime_annotation_test();
    struct_lifetime_annotations_test();
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


// fn best_fuel2<'a>(x:&'a str, y:&'a str)-> &'a str{
//     if x.len() > y.len() { x } else { x } //why is this still failing -> cos of implemented lifetime annotation in the return
// }

fn best_fuel3<'a,'b>(x:&'a str, y:&'b str)-> &'a str{
    //using multiple timeline annotation, to separate the the timeline restriction
    if x.len() > y.len() { x } else { y }
}

fn multi_lifetime_annotation_test(){
    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNG");
        result = best_fuel3(&propellant1, &propellant2);
    }
    println!("result is {}", result);
}

fn get_first_world<'a>(s: &'a str)-> &'a str{
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }
    &s
}
//before rust updates, rust used to require explicit declaration of lifetime for every references
fn elision_rule_test(){
    //Lifetime Elision Rules
    // - Set of rules for the compiler to analyze reference lifetimes
    // - allows for the omitting of lifetime annotation for certain describe situations that do not require the annotations
    // - explicit annotation will still be required if any ambiguity remains
    // The rules
    // 1. Each input parameter that is a references is assigned its onw lifetimes
    // 2. If there is exactly one input lifetime, assign it to all output lifetimes ( no 2 to 1 )
    // 3. If there is a &self or &mut self input parameter, its lifetime will be assigned to all output lifetimes
}
struct Shuttle<'a>{
    name: &'a str
}
impl<'a, 'b> Shuttle<'a> {
    // third rule of lifetime elision
    fn send_transmission( &'a self, msg: &'b str) -> &'b str {
        println!("Transmitting message:{msg}");
        //&self.name // msg cannot be return if you dont add explicit lifetime annotation
        msg
    }
}

fn struct_lifetime_annotations_test(){
    let vehicle = Shuttle{
        name: "Endeavour"
    };
    let sender = vehicle.send_transmission("Greeting from orbit!");
    println("sender is {sender}");
}

//'static Lifetime
// Indicates references available for the ENTIRE DURATION of the program
//Example : String Literals
// let s: &'static str = "Greeting from Neptune!";
// Can be coerced to be more restrictive lifetime # rarely time
// its also used as a Trait Bound  -> Ensure the data type will only contain ' static elements
// <T : 'static>