// Aaron Palomin
// Added some notes for myself to help understand
// This is needed for account balance as private
#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

// Being able to create a new BankAccount
impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount { balance: initial_balance }
    }
    // Being able to deposit money into account
    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }
    // Being able to withdraw money from account
    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }
    // Being able to return to self of current balance of account
    pub fn balance(&self) -> f64 {
        self.balance
    }
    // (Bonus challenge) Being able to use interest on a balance
    pub fn apply_interest(&mut self, interest_rate: f64) {
        if interest_rate > 0.0 {
            self.balance += self.balance * interest_rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // This tests that a new account with a default balance
    #[test]
    fn test_new_account() {
        let account = BankAccount::new(550.0);
        assert_eq!(account.balance(), 550.0);
    }
    // This tests depositing money to account
    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(550.0);
        account.deposit(250.0);
        assert_eq!(account.balance(), 800.0);
    }
    // This tests withdrawing money from account
    #[test]
    fn test_withdraw() {
        let mut account = BankAccount::new(800.0);
        account.withdraw(150.0);
        assert_eq!(account.balance(), 650.0);
    }
    // This tests withdrawing money that is more than the balance contained
    #[test]
    fn test_withdraw_more_than_balance() {
        let mut account = BankAccount::new(650.0);
        account.withdraw(700.0);
        assert_eq!(account.balance(), 650.0);
    }
    // This tests depositing money that is negative which is wrong
    #[test]
    fn test_negative_deposit() {
        let mut account = BankAccount::new(650.0);
        account.deposit(-700.0);
        assert_eq!(account.balance(), 650.0);
    }
    // This tests withdrawing money that is negative which is wrong
    #[test]
    fn test_negative_withdraw() {
        let mut account = BankAccount::new(650.0);
        account.withdraw(-700.0);
        assert_eq!(account.balance(), 650.0);
    }
    // This tests applying interest of a account balance
    #[test]
    fn test_apply_interest() {
        let mut account = BankAccount::new(650.0);
        account.apply_interest(0.12);
        assert!((account.balance() - 728.0).abs() < 1e-10);
    }
}
