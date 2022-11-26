fn main() {
    //function_parameter_test();
    //function_return_test();
    convert_temperature_challenge()

}

fn function_parameter_test(){
    say_hello();
    say_hello();
    //if not specified, compilier determine the type
    let (x,y) = (1,2);
    say_the_sum(x,y);

}

fn say_hello(){
    println!("hello!");
    // this is an example of the usage of a argurement;
    say_a_number(12);
}

// this is example of a parameter
fn say_a_number(number:i32){
    println!("number is {}",number);
}

fn say_the_sum(a:u8, b:u8){
    println!("the sum is {}.", a+b);
}

// Statement vs Expression
// statement - performs an actions with out return a value ( uses ;) let a = 1 + 2;
// expression = evaluates to a resulting value ( does not uses ;) 1 + 2;

fn function_return_test(){
    let result = square(13);
    println!{"result is {}", result};
    let result2 = square_ret_tuple(13);
    // using special formatter to print out tuple
    println!{"result is {:?}", result2};
}

fn square(x: i32) -> i32 {
    println!("squaring {}", x);

    return x*x;
    println!("End of Function");

    // we can just use an expression without semicolon to return;
    x*x
    // but we can't have statement after this statement; 
}

fn square_ret_tuple(x: i32) -> (i32, i32){
    println!("squaring {}", x);

    return (x,x*x);
    println!("End of Function");

    // we can just use an expression without semicolon to return;
    (x,x*x)
    // but we can't have statement after this statement;  
}

//the unit datatype, () - used by compiler for function with no return

fn convert_temperature_challenge(){
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);
    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

fn celsius_to_fahrenheit(celsius:f64) -> f64{
    (celsius * (9.0/5.0)) + 32.0
}