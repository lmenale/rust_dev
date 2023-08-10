// Rust Application: Memory safety Showcase
// 
// This example demonstrates a basic Rust application that simulates a simple bank account management system.
// We'll use Rust's memory safety, strong type system, and ownership model to showcase its advantages.
//
// - Define a BankAccount struct with an owner's name and balance.
// - Use Rust's strong type system to ensure that the owner's name is a String and the balance is an f64.
// - Demonstrate memory safety by using Rust's ownership model and references to manipulate the BankAccount instance.
// - Prevent invalid operations like negative deposits or excessive withdrawals using Rust's type system and strong ownership checks.
// 
// This example highlights how Rust's memory safety, strong type system, and ownership model contribute to developing a secure and
// reliable application.

// Please note that this is a simplified example for illustrative purposes, and real-world applications may involve
// more complexity and features.

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: &str, balance: f64) -> Self {
        Self {
            owner: owner.to_string(),
            balance,
        }
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account = BankAccount::new("Alice", 1000.0);

    account.deposit(500.0);
    account.withdraw(300.0);

    println!("Account balance for {}: {}", account.owner, account.get_balance());
}
