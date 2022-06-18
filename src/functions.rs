pub fn run(){
println!("{}", multiple(2, 4));
//closure
let add_num = |x: i32, y: i32| x + y;
println!("{}", add_num(2, 4));
    
}

fn multiple(x: i32, y: i32)-> i32 {
    return x * y
}