
//Normal Struct
struct Color{
    red: u8,
    green: u8,
    blue: u8
}
//truple struct
struct ColorTuple(u8, u8, u8);

//
struct Person{
    first_name: String,
    last_name: String
}

impl Person{
    fn new(first_name: &str, last_name: &str) -> Person{
        Person{
            first_name: first_name.to_string(),
            last_name: last_name.to_string()
        }
    }
    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last_name: &str){
        self.last_name = last_name.to_string();
    }

    fn to_tuple(&self) -> (String, String){
        (self.first_name.to_string(), self.last_name.to_string())
    }
}

pub fn run(){
    let mut x = Color{red: 255, green: 120, blue: 0};
    x.red = 200;
    println!("{}{}{}", x.blue, x.green, x.red);

    let mut y = ColorTuple(255, 120, 0);
    y.0 = 200;
    println!("{}{}{}", y.2, y.1, y.0);

    let mut p = Person::new("Johny", "Gnu");
    p.first_name = "Jane".to_string();
    println!("{} {}", p.first_name, p.last_name);
    p.set_last_name("Doe");
    println!("{} {}", p.first_name, p.last_name);
    println!("{}", p.full_name());
    println!("{}", p.to_tuple().0); 
    println!("{}", p.to_tuple().1);
}