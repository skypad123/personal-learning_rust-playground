use std::any;
use std::fmt;
fn main(){
    // implement_traits_test();
    // derivable_traits_test();
    // trait_bound_test();
    // return_with_trait_test();
    impl_display_trait_challenge();
}


//Trait
// - A collection of methods
// - Data types can implement a trait
// - Generics use traits to specify the capabilities of unknown data types
// - similar to interface other languages


trait Description{
    //fn describe(&self)-> String;
    // using a default
    fn describe(&self)-> String{
        String::from("an object flying through space.")
    }
}

struct Satellite {
    name: String,
    velocity: f64
}

struct SpaceStation{
    name:String,
    crew_size: u8,
    altitude: u32
}

impl Description for Satellite{
    fn describe(&self) -> String {
        format!("the {} flying at {} miles per second!",self.name,self.velocity)
    }
}

impl Description for SpaceStation{
    fn describe(&self) -> String {
        format!("the {} flying at {} miles high with {} crew members aboard!",self.name,self.altitude, self.crew_size)
    }
}

fn implement_traits_test(){
    let hubble = Satellite{
        name: String::from("hubble Telescope"),
        velocity: 4.72
    };
    let iss = SpaceStation{
        name: String::from("Internation Space Station"),
        crew_size: 6,
        altitude: 254
    };
    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());
}

// Derivable Traits
// - Provide default implementations for several common traits
// Eq / PartialEq / Ord / PartialOrd / Clone / Copy / Hash / Default / Debug

#[derive(PartialEq, PartialOrd)]
struct Satellite2 {
    name: String,
    velocity: f64
}

fn derivable_traits_test(){
    let hubble = Satellite2{
        name: String::from("hubble Telescope"),
        velocity: 4.72
    };
    let gps = Satellite2{
        name: String::from("GPS"),
        velocity: 2.47
    };
    println!("hubble == gps is {}", hubble == gps);
    println!("hubble > gps is {}", hubble > gps);
    // what is being compared ? the bits in representing the data
}

// Trait Bounds
// - Require a generic type to implement specific traits
// - Guarantees the generic type will have necessary behaviors

// fn print_type<T: fmt::Display>(item:T){
//     println!("{} is {}", item, any::type_name::<T>());
// }

fn print_type<T: fmt::Debug>(item:T){
    println!("{:?} is {}", item, any::type_name::<T>());
}
fn trait_bound_test(){
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    print_type([13]); // will return error as there is not fmt::Display trait for array
}

//fn compare_and_print< T: fmt::Display + PartialEq + from<U>, U: fmt::Display + PartialEq + Copy>(a:T, b:U){
fn compare_and_print< T, U>(a:T, b:U)
    where T: fmt::Display + PartialEq + From<U>,
          U: fmt::Display + PartialEq + Copy
{
    if a == T::from(b){
        println!("{} is equal to {}", a,b);
    }else{
        println!("{} i NOT equal to {}",a,b);
    }
}

fn multi_trait_test(){
    compare_and_print(1.0,1);
    compare_and_print(1.1,1);
}


fn get_displayable() -> impl fmt::Display {
    "thirteen"
}

// fn get_ambi_displayable(choice:bool) -> impl fmt::Display {
//     if choice {
//         13
//     // }else{
//     //     "thirteen"
//     // }
// }

fn return_with_trait_test(){
    println!("output is {}", get_displayable());
}

struct Satellite3{
    name: String,
    velocity:f64
}

impl fmt::Display for Satellite3{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is flying at {} miles per hour", self.name, self.velocity)
    }
}


fn impl_display_trait_challenge(){
    let hubble = Satellite3{
        name : String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("hubble is {}", hubble);
}