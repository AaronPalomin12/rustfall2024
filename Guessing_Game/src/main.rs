// Aaron Palomin

// Implement a function check_guess(guess: i32, secret: i32) -> i32 that returns:
fn check_guess(guess: i32, secret: i32) -> i32
{
    if guess == secret
    {
        // 0 if the guess is correct
        0
    }
    else if guess > secret
    {
        // 1 if the guess is too high
        1
    }
    else
    {
        // -1 if the guess is too low
        -1
    }
}

fn main() 
{
    // secret number that is trying to be guessed
    let secret_number: i32 = 8;

    // counter of guesses
    let mut guess_count = 0;

    // starting guess
    let mut guess = 5;

    // goes until secret number is found
    loop 
    {
        // increment
        guess_count += 1;

        // compare secret number and check_guess function
        let result = check_guess(guess, secret_number);

        // Use an if-else expression to print whether the guess was correct, too high, or too low
        if result == 0
        {
            // If the guess was correct, break the loop
            println!("Correct! The secret number is {}", guess);
            break;
        }
        else if result == 1
        {
            // too high
            println!("Too high! Try again.");
            guess -= 1;

        }
        else
        {
            // too low
            println!("Too low! Try again");
            guess += 1;

        }

    }

    // the amount of guesses it took
    println!("It took you {} guesses.", guess_count);
}
