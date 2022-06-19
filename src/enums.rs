enum Movment {
    Up,
    Down,
    Left,
    Right,
}

fn mov_avatar(mov: Movment){
    match mov {
        Movment::Up => println!("Avatar is moving up"),
        Movment::Down => println!("Avatar is moving down"),
        Movment::Left => println!("Avatar is moving left"),
        Movment::Right => println!("Avatar is moving right"),
    }
}

pub fn run(){
    let Human = Movment::Up;
    mov_avatar(Human);
}