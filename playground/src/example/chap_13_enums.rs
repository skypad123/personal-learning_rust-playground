use core::panicking::panic;

fn main(){

}

// Errors are part of life in software
// rust has several features to handle runtime errors
// Errors are grouped into two categories: recoverable and un recoverable
// Types of Errors
// - Recoverable
//      Example file not found error
//      Handled with Result<T,E>
// - Unrecoverable
//      Example index beyond array bounds
//      Handled with panic!
//      Immediately terminate the program and provide feedback

fn unrecoverable_test(){
    panic("Houston, we've had a problem.");
}