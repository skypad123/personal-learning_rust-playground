//Tuple Data Type
//Group multiple item of mixed data types
//Elements are ordered
//Struct Data Type
//also Groups multiple items of mixed data types
//but elements are named that can be called using their name
//example -> smth.property1 # now you don't have care about order

// need this to print out debug message -> related to traits
#[derive(Debug)]
#[derive(Clone)]
struct Shuttle {
    name : String,
    crew_size : u8,
    propellant : f64
}

fn main(){
    //def_struct_test();
    //struct_update_test();
    //struct_method_test();
    //associated_fn_test();
    //tuple_struct_test();
    shape_challenge();
}

fn def_struct_test(){
    // only need to specify mutability on the instance not definition
    let mut vehicle = Shuttle{
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 8359958.0
    };
    println!("name is {}", vehicle.name);
    vehicle.name = String::from("Atlantis");
    println!("vehicle is {:?}", vehicle);
}
// where is the struct data stored?
// on the stacks... unless there a array/String then then it will store the ptr and meta in the stack


fn struct_update_test(){
    let mut vehicle = Shuttle{
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 8359958.0
    };
    let mut vehicle2 = Shuttle{
        name: String::from("Discovery"), // String will produce error as there cannot be 2 owners
        ..vehicle // use first vehicle as temple and change only specified fields
    };
    let mut vehicle3 = Shuttle {
        ..vehicle.clone() // use clone to bypass by creating a duplicate of first vehicle
    };
    vehicle.crew_size = 6;
    println!("vehicle is {:?}", vehicle);
    println!("vehicle2 is {:?}", vehicle2);
}

//method
//subroutine associated with a struct
// can have input parameters and a return value
// declared using the fn keyword too
impl Shuttle{
    //method
    fn get_name(&self) -> &str {
        &self.name
    }
    //method
    fn add_fuel(&mut self, gallons:f64){
        self.propellant += gallons;
    }
    //associated function
    fn new(name: &str) -> Shuttle{
        Shuttle{
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0
        }
    }

}
fn struct_method_test(){
    let mut vehicle = Shuttle{
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 0.0
    };

    let vehicle_name = vehicle.get_name();
    println!("vehicle_name is {}", vehicle_name);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is {}", vehicle.propellant);
}

// Associated Function
// Function associated with a struct data type
// Does not have a &self parameter
fn associated_fn_test(){
    let mut vehicle = Shuttle::new("Endeavour");
    let mut vehicle2 = Shuttle::new("Discovery");
    let vehicle_name = vehicle.get_name();
    println!("vehicle_name is {}", vehicle_name);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is {}", vehicle.propellant);
}

// Tuple Structs
// Store a collection of mixed data without named fields
// Distinguishable as a unique data type

struct Color(u8,u8,u8); //RGB
struct Point(u8,u8,u8); //XYZ
// now defined Color cant be used for Point vise visa
fn get_y(p:Point) -> u8{
    p.1
}

fn tuple_struct_test(){
    let red = Color(255,0,0);
    println!("First value is {}", red.0);
    // below will give error
    // let y = get_y(red);
    // println!("y is {}", y);
    let coord = Point(4,5,6);
    let y = get_y(coord);
    println!("y is {}", y);
}

struct Rectangle{
    width: f64,
    height: f64
}

impl Rectangle {
    fn get_area(&self) -> f64{
        self.width * self.height
    }
    fn scale(&mut self, factor:f64) {
        self.width *= factor;
        self.height *= factor;
    }
    fn new(width:f64, height:f64) -> Rectangle{
        Rectangle{
            width,
            height
        }
    }
}

fn shape_challenge(){
    let mut rect = Rectangle::new(1.2,3.4);
    assert_eq!(rect.get_area(),4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(),1.02);
    println!("Tests passed!");
}