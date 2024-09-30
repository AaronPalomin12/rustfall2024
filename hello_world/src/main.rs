use std::io::{self, Read, Write};

struct Person {
    name: String,
    car: String,
}

fn reading_from_console() {
    let mut buffer = String::new();




    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("What kind of car do you have? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let car = buffer.trim().to_string();

    let person = Person { name, car };
    println!("Hi {}, you have {} car!", person.name, person.car);

}

fn main()
{
    reading_from_console();
}