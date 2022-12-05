fn main() {
    //define_enum_test();
    // match_operator_test();
    // match_with_default_test();
    //enum_method_test();
    //option_t_test();
    //match_option_t_test();
    //if_let_test();
    location_challenge();
}

// Enum
// Defines a datatype with multiple possible variants
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64,f64),
    Triangle(f64,f64,f64)
}

//how it enums can be use
enum Command {
    Clear,
    DrawLine(f64,f64),
    DrawShape(Shape)
}

fn define_enum_test(){
    let my_shape = Shape::Rectangle(1.2,3.4);
    println!("my shape is {:?}", my_shape);

}

// Match Operator
// Compares a value to a series of patterns to determine which code to execute

fn match_operator_test(){
    let my_shape = Shape::Rectangle(1.2,3.4);
    println!("my shape is {:?}", my_shape);

    match my_shape{
        Shape::Circle(r) => println!("Circle with radius {r}."),
        Shape::Rectangle(w,h) => println!("{w} x {h} Rectangle."),
        Shape::Triangle(a,b,c)=> println!("Triangle with sides {a},{b},{c}")
    }
}


fn match_with_default_test(){
    let my_number = 4u8;
    let result =  match my_number{
        0 => "Zero",
        1 => "one",
        2 => "two",
        3 => "three",
        // all cases must be covered, so we need to use a wild card value
        //include at the end else all case will get trigger as a catch all
        _ => {
            println!("{my_number} did not match");
            "something else"
        }
    };
    println!("result is {result}");
}

impl Shape {
    fn get_perimeter(&self)-> f64 {
        match *self{
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w,h) => 2.0 * w + 2.0 * h,
            Shape::Triangle(a,b,c) => a+b+c
        }
    }
}

fn enum_method_test(){
    let my_shape = Shape::Rectangle(1.2,3.4);
    println!("my_shape is {:?}", my_shape);
    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {perimeter}");
}

// How Do You Represent Nothing
// many language use null value to indicate "no_value"
// Error often occur when using a null value in a not-null context
// Rust does not have a traditional null value

// Option<T> Enum
// enum Option<T>{
//     Some(T),
//     None
// } // included in the prelude

//let something = Some(13); // Option<i32>
//let something = Some(13.0); // Option<f64>
//let something = Some("thirteen"); // Option<&str>
//let nothing = None

fn option_t_test(){
    let countdown = [5,4,3,2,1];
    let number = countdown.get(4);
    //let number = number.unwrap() + 1; // cannot add integer to the option<integer> -> so we use the unwrap feature (but can cause problems)
    let number = number.unwrap_or(&0) + 1; // unwrap_or allow to specify default value if option was none
    println!("number is {:?}", number);
}

fn match_option_t_test(){
    let countdown = [5,4,3,2,1];
    let number = countdown.get(5);
    //let number = number.unwrap() + 1; // cannot add integer to the option<integer> -> so we use the unwrap feature (but can cause problems)
    let number = match number {
        Some(number) => number + 1,
        None => 0
    };
    println!("number is {:?}", number);
}

fn if_let_test(){
    let number = Some(13);
    // match number {
    //     Some(13) => println!("thirteen"),
    //     _ => ()
    // }
    if let Some(13) = number {
        println!("thirteen");
    } // alternate of the top version
}

enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64) // latitude , longitude
}

impl Location {
    fn display(&self){
        match self {
            Location::Unknown => println!("The location is Unknown."),
            Location::Anonymous=> println!("The location is Anonymous."),
            Location::Known(latitude, longitude) => println!("The location is located at {latitude}, {longitude}.")
        }
    }
}

fn location_challenge(){
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295,-80.604177);
    address.display();
}