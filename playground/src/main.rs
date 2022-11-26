
fn main(){
    // if_test();
    // multiple_if_test();
    // conditional_assignment_test();
    //loop_test();
    //while_loop_test();
    // for_loop_test();
    // nested_loop_test();
    averaging_challange();
}

fn if_test(){
    let x = 3; 
    if x == 3 {
        println!("x is 3!");
    }
    let y = 2; 
    if y == 3 {
        println!("y is 3!");
    }

    // rust don't allow int as boolean
    // if 3 {
    //     println!("x is 3!");
    // } 

    if x + 1 != 3 {
        println!("x+1 is NOT EQUAL to 3!");
    }
}

fn multiple_if_test(){
    let x = 3; 
    let y = 5;

    if x > y{
        println!("x is greater than y");
    } else if x < y{
            println!("x is less than y");
    }else{
        println!("x is equal to y");
    }
}

fn conditional_assignment_test(){
    let x; 

    if true {
        x = 1;
    }else{
        x = 2;
    }

    // both branches of if statement must assign x a value
    //rust compiler will check if statement is possible to be no value
    println!(" x is {}", x);

    // using if statement as an expression;
    let make_y_odd = true; 
    // however need to ensure that the return value are of the same type
    let y = if make_y_odd {1} else {2};
}

//Types of loops
// loop - infinite loop till explicit request to stop
// while loop 
// for loop

fn loop_test(){
    let mut count = 0; 

    let result = loop{
        if count >= 10{
            //exit with value for loop expression;
            break count * 10;
            //exit loop with break;
            //break;
        }
        count += 1; 
        println!("count is {}", count);
    };

    println!("After the loop!");
    println!("result is {}", result);
}

fn while_loop_test(){
    let mut count = 0;
    // while loop allow for loop to never execute if expression is not true
    while count < 10 {
        count += 1; 
        println!("count is {count}");
    }

    count = 0;
    let letters = ['a', 'b', 'c' ];
    while  count < letters.len() {
        println!("letter is {}", letters[count]);
        count += 1;
    }
}

fn for_loop_test(){
    let message = ['h','e','l','l','o'];
    // rust 1.53 allows for collection item to have the IntoIterator trait 
    // that allows it to call message directly instead of using the message.iter() method
    for item in message{
        println!("item is {item}");
    }

    // iterator implements logic to iterate over eact item in a collection
    // next() method returns the next item in the sequence


    // using emumerate to get (index,value) as item
    //we deconstructed the item here
    for (index, &item) in message.iter().enumerate(){
        println!("item {index} is {item}");


        // key to note is that iter gives us a reference rather then the value
        // add & for item to get value
        if item == 'e' {
            break
        }  
    } 

    //range value 0..5 is [0,1,2,3,4]
    for number in 0..5{
        println!("number is {number}");
    }

}


fn nested_loop_test(){
    let matrix = [[1,2,3,],[4,5,6],[7,8,9]];
    for row in matrix{
        for num in row{
            // \t is tab
            print!("{num}\t");
        }
        println!();
    }

    let mut matrix2 = [[1,2,3,],[4,5,6],[7,8,9]];
    for row in matrix2.iter_mut(){
        for num in row.iter_mut(){
            *num += 10;
            // \t is tab
            print!("{num}\t");
        }
        println!();
    }

}


fn averaging_challange(){
    let numbers = [1,9,-2,0,23,20,-7,13,37,20,56,-18,20,3];
    let mut max: i32 = numbers[0]; 
    let mean: f64; 
    let mut min: i32 = numbers[0];
    let mut sum: f64 = 0.0;

    for item in numbers{
        max = if max < item {item} else{max}; 
        min = if min > item {item} else {min};
        sum += item as f64;
    }
    mean = sum / numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Test passed!");
}