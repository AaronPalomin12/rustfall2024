// Aaron Palomin
// reminders for myself: don't have to put semicolons after expressions
// make sure for put 

// Declare a constant for the freezing point of water in Fahrenheit (32F)
const freezing_point_of_water: f64 = 32.0;

// fahrenheit to celsius with formula (f - 32) * 5/9 = C

fn fahrenheit_to_celsius(f: f64) -> f64
{
    (f - freezing_point_of_water) * 5.0 / 9.0
}

// celsius to fahrenheit with formula (C * 9/5) + 32 = F

fn celsius_to_fahrenheit(c: f64) -> f64
{
    (c * 9.0 / 5.0) + freezing_point_of_water
}

fn main()
{

    //Declare a mutable variable with a temperature in Fahrenheit
    // A 'mut' keyword means the value can be changed later in the program
    let mut fahrenheit_temp: f64 = 32.0;
    println!("starting temperature in fahrenheit: {}",fahrenheit_temp);

    // Convert it to Celsius using your function and print the result
    // {:.2} is used to format the output to 2 decimal places
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