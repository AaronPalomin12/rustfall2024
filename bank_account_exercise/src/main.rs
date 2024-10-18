// Aaron Palomin
// Added some notes for myself to help understand
mod bank_account;

fn main() {
// This creates a new account with a default balance
    let mut account = bank_account::BankAccount::new(550.0);
    println!("Initial balance: {}", account.balance());
// This deposits 250 dollars into account
    account.deposit(250.0);
    println!("After deposit: {}", account.balance());
// This deposits 150 dollars into account
    account.withdraw(150.0);
    println!("After withdrawal: {}", account.balance());
// This attempts to withdraw more money than is actually in the account
    account.withdraw(700.0);
    println!("After attempting to withdraw 700 (should remain unchanged): {}", account.balance());
// This attempts to deposit a negative amount of money which is wrong
    account.deposit(-700.0);
    println!("After attempting to deposit -700 (should remain unchanged): {}", account.balance());
    // This attempts to deposit a negative amount of money which is wrong
    account.withdraw(-700.0);
    println!("After attempting to withdraw -700 (should remain unchanged): {}", account.balance());
// This attempts to use 12 % interest.
    account.apply_interest(0.12);
    println!("After applying 12% interest: {}", account.balance());

    
}
