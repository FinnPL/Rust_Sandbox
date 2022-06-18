//var are imutable by default
//var are mutable if we use mut keyword
//Rust is a block-scoped language

pub fn run(){
    let name = "John";
    let mut age = 30;
    age = 31;
    println!("My name is {} and I am {}", name, age);

    //Define constant
    const BIRTHYEAR: i32 = 1991;
    println!("My birthyear is {}", BIRTHYEAR);

    // Multiple variables
    let (my_name, my_age) = ("John", 30);
    println!("My name is {} and I am {}", my_name, my_age);
}