pub fn run(){
    //Simple print
    println!("Hello World!");

    // Basic Formating
    println!("Number: {}",1);

    // Positional Arguments
    println!("{0} {1} {0}", "Hello", "World");

    // Named Arguments
    println!("{greeting} {name}", greeting="Hello", name="World");

    // Placeholder traits
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);  
    //{:b} is binary  {:x} is hexadecimal {:o}  is octal {:e} is exponential {:g} is general {:f} is fixed point {:t} is a datetime {:c} is a character     ...

    // Placeholder for debug trait
    println!("{:?}", (16, true, "hello World"));

    // Basic math
    println!("{}", 1+2);

    // Align text
    println!("{number:>width$}", number=1, width=6); 

    // Fill text
    println!("{number:>0width$}", number=1, width=6);

    // Precision
    println!("{number:.precision$}", number=3.14, precision=2);
}