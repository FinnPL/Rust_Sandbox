pub fn run(){
    let args: Vec<String> = std::env::args().collect();
    let command = &args[1];
    let name = &args[2];
    print!("Hello {}  ", name);
    println!("{} command was used", command);
}