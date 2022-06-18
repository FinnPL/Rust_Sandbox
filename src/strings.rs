//Two types of string:
//Primitive Strings: str  = Immutable fixed-lenght string
//String Literals: String = Mutable, growable string, heap-allocated


pub fn run(){
    //primitiv string:
    let name = "John";

    //String literal:
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{}", s);

    //Create a new string with capacity for 100 characters
    let mut x = String::with_capacity(100);
    
    //Get the length of a string
    println!("Length: {}", s.len());

    //Check if string is empty
    println!("Is Empty: {}", s.is_empty());

    //Contains a substring
    println!("Contains 'World': {}", s.contains("World"));

    // Replace a substring
    println!("Replace: {}", s.replace("World", "There"));

    //...



    // Split a string into substrings at a given pattern
    let v: Vec<&str> = s.split(" ").collect();
    println!("{:?}", v);

    // Iterate over string by character
    for c in s.chars(){
        println!("{}", c);
    }

    //Assertion testing
    let s = "Hello World!";
    assert_eq!(12,s.len());
    print!("{}", s);
    


}