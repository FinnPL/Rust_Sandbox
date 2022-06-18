/*
Primitiv types:

Integer: Unsigned: u8, u16, u32, u64, u128, usize 
         Signed: i8, i16, i32, i64, i128, isize   
Floating point: f32, f64
Boolean: bool
Character: char


Tuple: Tuple structs are useful for grouping together related values.
       They are used when the order of the values is important.
       They are also used for disambiguating between multiple possible types.
       For example, a tuple struct can be used to return multiple values from a function.
       The type of the tuple struct is a tuple of the types of the values it contains.

Array: Arrays are a fixed size collection of values of the same type.
       They are used when you know exactly how many values you want to store.
       They are also used when you want to store a collection of the same type.

Vector: Vectors are growable arrays.
        They are used when you don't know how many values you want to store.
        They are also used when you want to store a collection of the same type.
*/

pub fn run(){
    // Integer
    let x = 2; // Compiler --> i32
    let y: i32 = 2; // code     --> i32

    // Floating point
    let z = 2.0; // Compiler --> f64
    let i: f64 = 2.0; // code     --> f64
    
    //...

    //Find max sice of data type
    println!("Max i8: {}", std::i8::MAX);
    println!("Max i16: {}", std::i16::MAX);

    //Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1); //create a tuple
    let (x, y, z) = tup;  // Destructuring tuple
    println!("{:?}",(x,y,z));

    //Array
    let a: [i32; 5] = [1,2,3,4,5]; //create an array 
    println!("{:?}", a); //print array
    println!("{}", a[0]); //print array element

    let slice: &[i32] = &a[0..2];//create a slice
    println!("{:?}", slice); //print slice

    //Vector
    let mut v:Vec<i32>= vec![1,2,3,4,5]; //create a vector
    v.push(6); //add element to vector
    println!("{:?}", v); //print vector
    println!("{}", v[0]); //print vector element

}