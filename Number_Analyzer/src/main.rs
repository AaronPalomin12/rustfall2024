// Implement a function is_even(n: i32) -> bool that returns 
// true if a number is even, false otherwise.
fn is_even(n: i32) -> bool
{
    n % 2 == 0
}


fn main() 
{
    // Create an array of 10 integer numbers of your choice
    let numbers: [i32; 10] = [5,26,4,1,39,20,5,19,2,8];

    // Implement a function is_even(n: i32) -> bool that 
    // returns true if a number is even, false otherwise.
    // iterate
    for &num in numbers.iter() 
    {
        // even or odd
        if is_even(num)
        {
            println!("{} is even", num);
        }
        else
        {
            println!("{} is odd", num);
        }

        //Fizzbuzz logic
        // If it's divisible by both 3 and 5, print "FizzBuzz"
        if num % 3 == 0 && num % 5 == 0
        {
            println!("FizzBuzz");
        }
        // If the number is divisible by 3, print "Fizz" instead
        else if num % 3 == 0
        {
            println!("Fizz");
        }
        // If the number is divisible by 5, print "Buzz" instead
        else if num % 5 == 0
        {
            println!("Buzz");
        }

    }
    

    // Use a while loop to find and print the sum of all numbers in the array.
    let mut index = 0;
    let mut sum = 0;
    while index < numbers.len()
    {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Use a loop to find and print the largest number in the array.
    let mut largest = numbers[0];
    // iterate
    for &num in numbers.iter() 
    {
        if num > largest
        {
            largest = num;
        }
    }
    println!("Largest number that is in the array: {}", largest);
}
