pub fn run(){
    let age=17;
    let lives_in_germany = true;


    // if/else
    if age>=18{
        println!("You are old enough to vote!"); 
        println!("Have you registered to vote yet?");
    }else if age>=16 && lives_in_germany{
        println!("You can Drink Beer!");
    }else{
        println!("Sorry, you are too young to vote.");
        println!("Please register to vote as soon as you turn 18!");
    }

// Short hand if/else
    let is_old = if age>=18{true}else{false};
    println!("Is old: {}", is_old);


// Conditional Strings
    let check_adult = if age>=18 {
        "Adult"
    } else {
        "Not Adult"
    };

}