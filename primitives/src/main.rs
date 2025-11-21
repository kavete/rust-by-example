#![allow(unused)]
use std::mem;
fn main() {
    println!(" 1 + 2  = {}", 1i32 + 2);

    println!(" 1 - 2 = {}", 1i32 - 2); // u32 will not work
    //
    let logical: bool = true;
    let a_float: f64 = 5.6;
    let an_integer = 5i32; // Suffix annotation

    // The type of a variable can't be changed
    let mut mutable = 12;
    // mutable = "mutated"; // Integer expected

    mutable = 21;

    // Variables can be overwritten with shadowing

    let mutable = true;

    // Array signature containing type T and length as [T; length]

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // Tuples
    // A collection of values of different types

    let pair = (1, true);

    println!("The pair is {:?}", pair);

    println!("The value of the second member is {}", pair.1);
    // destructuring

    let tuple = (1, "hello", false, 9.5);

    let (a, b, c, d) = tuple;

    println!("b = {}", b);

    // Arrays and Slices

    let xs = [1, 2, 3, 4];

    // All elements can be initialized to be the same value

    let ys = [0; 500];

    // len() returns no. of elements in the array

    println!("The length of ys is {}", ys.len());

    // Arrays are stack allocated

    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    let slice = &ys[0..45];

    println!("Slice {:?}", slice);
}
