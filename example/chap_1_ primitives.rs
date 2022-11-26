fn main() {
    //var_test();
    //int_test();
    //float_test();
    //arithmetic_test();
    //println_test();
    // bitwise_test();
    //boolean_test();
    //comparison_test();
    //char_test();
    averaging_challenge();
}


fn averaging_challenge(){
    let a = 13; 
    let b = 2.3; 
    let c: f32 = 120.0;

    let sum: f64 = a as f64 + b + c as f64;
    let avg = sum/3.0 ;
    assert_eq!(avg,45.1);
    println!("Test passed!");
}

fn char_test(){
    //represents a single character
    //Unicode scalar value
    //Stored using 4 bytes more than c++
    let letter = 'a';
    let finger = '1';
    //specify char using unicode
    let number = '\u{261D}'; 
    println!("{letter}\n{number}\n{finger}");

}

fn comparison_test(){
   let a = 1; 
   let b = 2; 
   // cannot compare between different types
   println!(" a is {a} and b is {b}"); 
   println!(" a EQUAL TO b is {}", a == b);
   println!(" a NOT EQUAL TO b is {}", a != b);
   println!(" a GREATER THAN b is {}", a > b);
   println!(" a GREATER THAN OR EQUAL TO b is {}", a >= b);
   println!(" a LESS THAN b is {}", a<b); 
   println!(" a LESS THAN OR EQUAL TO b is {}", a <= b);
}

fn boolean_test(){
    let a = true; 
    let b = false;
    println!("a is {a} and b is {b}");
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a|b);
    println!("a XOR b {}", a^b);

    //shortcurcuit with && and || 
    // use panic!() for exiting to out of program with or operator 
}

fn bitwise_test(){
    // allows for underscore to spread bits out
    let mut value = 0b1111_0101u8;
    println!("value is {value}"); 
    println!("value is {value:08b}");
    // bitwise operation

    // Not
    value = !value;
    println!("not value is {value:08b}");

    // And (filter out specific bit)
    value = value & 0b1111_1011;
    println!("and value is {value:08b}");

    // or (set value in specific bit)
    value = value | 0b0100_0000;
    println!("or value is {value:08b}");

    // xor (check difference)
    value = value ^ 0b0101_0101;
    println!("xor value is {value:08b}");

    //shift left  (shift bit operation) zero padding
    value = value << 4;
    println!("lsft value is {value:08b}");

    //shift right (shift bit operation) zero padding
    value = value >> 4;
    println!("rsft value is {value:08b}");

}


fn println_test(){
    //formatting println
    //default
    let a = 10.0;
    let c = 10.000002;
    println!("c is {}", c); 
    //formatted for 8 (white spaces) char ahead and 3 dp
    println!("c is {:8.3}", c);

    //formatted for 8 (zero padded) char ahead and 3 dp
    println!("c is {:08.3}", c);  
    //newline char
    println!("c is {:08.3}\na is {}", c,a);  
    // no auto new line before sentence
    print!("c is {:08.3}\na is {}", c,a); 
    //position parameter
    println!("c is {0:08.3}\na is {1}\nc is {0}", c,a); 
    // 1.58ver named parameter / can just used variable in bracket too 
    println!("c is {sum:08.3}\na is {num1}", sum = c, num1 = a);  
}

fn arithmetic_test(){
    // integers
    let a = 10; 
    let b = 3; 
    let intc = a+b;
    println!("ic is {}", intc);
    let intd = a-b;
    println!("id is {}", intd); 
    let inte = a*b;
    println!("ie is {}", inte);
    let intf = a/b;
    println!("if is {}", intf);
    let intg = a%b;
    println!("ig is {}", intg);

    //float
    let floata = 10.0; 
    let floatb = 3.0; 

    //let dividebyfloat = a/floatb; // cannot divide int/float
    /* rust does not export a value if operations are done with mixed types */

    // cast a to float using as keyword
    let operationbyfloat = a as f64 /floatb;
    println!("operationbyfloat is {}", operationbyfloat); 

    // float to int - remove the reminding decimal value (truncation)
    // i64 to i32, etc - ingore signficent half of integer (truncation)

    //use parathesis to modify order of operation

    let orderofoperation = a as f64 / (floatb + 1.0);
    println!("orderofoperation is {}", orderofoperation); 
}

fn float_test(){
    // f32 - single percision
    // f64 - double percision (default)
    let mut  x = 10.0; //now its a float
    println!("x is {}", x);
    x = 10.1234567890123456234;
    println!("x is {}", x);
    let y:f32 = 10.1234567890123456234; 
    println!("y is {}", y);
}

fn int_test(){
    // 0 to 255 unsigned 8bits -u8
    // -128 to 127 signed 8bits -i8
    //default integer - i32
    let mut x :i8 = 127;
    println!("x is {}", x);
    /* overflow should happen */
    x += 1 ;
    println!("x is {}", x);
}

fn var_test(){
    //variables are immutables by default
    //use mut keyword to allow mutability
    //only can begin with letter or underscore
    //see RPC 430 naming convension
    let mut x = 10;
    println!("x is {}", x);
    x= 20;
    println!("x is {}", x);
}
