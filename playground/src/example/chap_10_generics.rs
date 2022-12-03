use std::mem;
use std::process::Output;

fn main() {
    //generic_struct_def_test();
    //generic_method_test();
    sum_boxes_challenge();
}


//Generic DataTypes
// - Abstract stand-ins for concrete data types or other properties
// - Can be used with structs, functions , methods and more
// - Defined with <T>


#[derive(Debug)]
struct Rectangle<T> {
    width: T,
    height: T
}
#[derive(Debug)]
struct Rectangle2<T,U> {
    width: T,
    height: U
}

fn generic_struct_def_test(){
    let rect = Rectangle{
        width: 1, //compiler replace the type used for T
        height: 3 // but if the type is already sub the type then it cannot change again
    };
    let rect2 = Rectangle2{
        width: 1u8,
        height: 3u16
    };
    // process of MONOMORPHIZATION
    // Compiler replaces generic placeholder with concrete data types (during compile types)
    println!("rect is {:?}", rect);
}

impl<T,U> Rectangle2<T,U>{
    fn get_width(&self) -> &T { // reference is use as it can be used to safely return for pointer type variables
        &self.width
    }
}

impl Rectangle2<u8,u8>{
    // implementation for specific type of Rectangle2
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}

fn generic_method_test(){
    let rect = Rectangle2 {
        width: 1u8,
        height: 3u8
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    println!("perimeter is {}", rect.get_perimeter());
}


// partialOrd Type traits - only for those with this traits
fn get_biggest<T: PartialOrd>(a:T, b:T) -> T{
    if a > b {
        a
    }else{
        b
    }
}

fn generic_function_test(){
    println!("biggest is {}", get_biggest(1,2));
    println!("biggest is {}", get_biggest(1.2,2.3));
}

// The Box<T> DataType -> smart pointer
// store data on the heap instead of on the stack
// Box<T> has ownership of the data it points
// When Box<T> goes out of scope it deallocates the heap memory

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

fn box_data_type_test(){
    let vehicle = Shuttle{
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0
    };
    println!("vehicle size on stack: {} bytes", mem::size_of_val(&vehicle));

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    println!("boxed_vehicle size on stack: {} bytes", mem::size_of_val(&boxed_vehicle));

    // Dereference Operator
    // Represented with * symbol
    // When applied to a pointer it denotes the pointed-to location
    println!("boxed_vehicle size on heap: {} bytes", mem::size_of_val(&*boxed_vehicle));

    let unboxed_vehicle = *boxed_vehicle;
    println!("unboxed_vehicle size on stack: {} bytes", mem::size_of_val(&unboxed_vehicle));

    // Use Cases for Box<T>
    // Store a type whose size cannot be known at compile time
    // Example: Recursive types

    // Transfer onwership of datas rather than copy it on the stack
}

fn sum_boxes<T:std::ops::Add<Output=T>>( a:Box<T> , b:Box<T> ) -> Box<T> {
    let numA = * a;
    let numB = *b;
    Box::new(numA + numB)
}

fn sum_boxes_challenge(){
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one,two),3);
    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi,e),5.85987);
    println!("Tests passed!");
}