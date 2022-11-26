fn main() {
    array_test();
    multidimensional_array_test();
    tuple_test();
}

fn array_test(){
    // all element must be a single type of datatype
    let mut letters = ['a', 'b' , 'c' ];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    let numbers: [i32; 5];
    // will errorr cus compiler will check if element is initialise before access.
    //println!("last number is {}", numbers[4]);

    let zero_numbers: [i32; 5] = [0;5];
    let index = zero_numbers.len();
    println!("last number is {}", zero_numbers[index]);
}


fn multidimensional_array_test(){
    //when working with multidimensional Array all inter array must be the same length.
    let parking_lot = [[1,2,3], [4,5,6]];
    let number = parking_lot[0][1];
    println!("number is {}", number);
    // 3-D Array
    let garage = [[[0;100]; 20]; 5];
}

fn tuple_test(){
    // can be mixed with different elements ( ordered )
    let mut stuff : (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!(" first_item is {}", first_item);
    // destructuring
    let (a,b,c) = stuff;
    println!(" b is {}", b);
}