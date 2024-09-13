// Aaron Palomin
// reminders for myself: don't have to put semicolons
// make sure for put 

// Declare a constant for the freezing point of water in Fahrenheit (32F)
const freezing_point_of_water: f64 = 32.0;

//Fahrenheit to celsius

fn fahrenheit_to_celsius(f: f64) -> f64
{
    (f - freezing_point_of_water) * 5.0 / 9.0
}

// celsius to fahrenheit

fn celsius_to_fahrenheit(c: f64) -> f64
{
    (c * 9.0 / 5.0) + freezing_point_of_water
}

fn main()
{

    //Declare a mutable variable with a temperature in Fahrenheit
    // A mutal is variable allows to change its own data.
    let mut fahrenheit_temp: f64 = 32.0;
    println!("starting temperature in fahrenheit: {}",fahrenheit_temp);

    // Convert it to Celsius using your function and print the result
    let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
    println!("temperature in celsius: {:.2}", celsius_temp);

    // Use a loop to convert and print the next 5 integer temperatures 
    // (e.g., if you start with 32°F, print conversions for 33°F, 34°F, 35°F, 36°F, and 37°F)
    // loop
    for temp in fahrenheit_temp as i64..fahrenheit_temp as i64 + 6 
    {
        let celsius = fahrenheit_to_celsius(temp as f64);
        println!("{}F = {:.2}C", temp, celsius);
    }

}