// Aaron Palomin

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

// Create a struct car
struct Car {
    name: String,
    manufactuerer: String,
    model: String,
    year: u32,
}


// Notes for myself to help me understand
// Function to ask the user about his or her car
fn get_car_info() -> Car {
    let mut buffer = String::new();

    // user's name
    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    // car manufacturer
    print!("Enter the manufactuerer of your car: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let manufactuerer = buffer.trim().to_string();
    buffer.clear();

    // car model
    print!("Enter the model of your car: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    buffer.clear();

    // car year
    print!("Enter the year of your car: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year: u32 = buffer.trim().parse().expect("Please enter a valid year");

    // Creating and returning the Car struct
    let car = Car { name, manufactuerer, model, year };

    // Print out a summary of the car information
    println!(
        "Hi {}, I see that your car is a {} {}, made in {}!",
        car.name, car.manufactuerer, car.model, car.year
    );

    car
}

// Saving information into the file 
fn save_car_info(car: &Car) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("user_car_info.txt")
        .unwrap();

    // Actually writing the information to the file
    writeln!(file, "Name: {}", car.name).unwrap();
    writeln!(file, "manufactuerer: {}", car.manufactuerer).unwrap();
    writeln!(file, "Model: {}", car.model).unwrap();
    writeln!(file, "Year: {}", car.year).unwrap();


    writeln!(
        file,
        "Hi {}, I see that your car is a {} {}, made in {}!",
        car.name, car.manufactuerer, car.model, car.year
    ).unwrap();
}

// reading car infomation from the file while also displaying it
fn read_car_info() {
    let mut file = File::open("user_car_info.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("\n--- Car Info from user_car_info.txt ---\n{}", contents);
}

fn main() {
    
    let car = get_car_info();
    save_car_info(&car);
    read_car_info();
}
