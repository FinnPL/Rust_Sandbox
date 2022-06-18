pub fn run(){
    let mut c = 0;

    loop{
        c += 1;
        println!("{}", c);
        if c == 10{
            break;
        }
    }

    while c < 10{
        c += 1;
        println!("{}", c);
    }
    
    for c in 0..10{
        println!("{}", c);
    }
    
}

