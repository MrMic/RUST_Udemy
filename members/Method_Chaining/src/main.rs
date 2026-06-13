// INFO: METHOD CHAINING CONSTRAINTS //

#[derive(Debug)]
struct BankAccount {
    balance: i32,
    owner: String,
}

impl BankAccount {
    fn new(owner: String, initial_balance: i32) -> Self {
        println!(
            "Creating a new bank account for {} with initial balance of ${}",
            owner, initial_balance
        );
        BankAccount {
            balance: initial_balance,
            owner,
        }
    }

    fn change_owner(mut self, new_owner: String) -> Self {
        println!("Changing owner from {} to {}", self.owner, new_owner);
        self.owner = new_owner;
        self
    }

    fn check_balance(&self) {
        println!("Checking balance for {}: ${}", self.owner, self.balance);
    }

    fn deposit(&mut self, amount: i32) -> &mut Self {
        println!("Depositing ${} to {}'s account", amount, self.owner);
        self.balance += amount;
        self
    }

    fn withdraw(&mut self, amount: i32) -> &mut Self {
        if self.balance >= amount {
            println!("Withdrawing ${} from {}'s account", amount, self.owner);
            self.balance -= amount;
        } else {
            println!(
                "Insufficient funds for {}. Current balance: ${}",
                self.owner, self.balance
            );
        }
        self
    }

    fn view_owner(&self) -> &Self {
        println!("The owner of this account is {}", self.owner);
        self
    }
}

fn main() {
    let mut account = BankAccount::new("Michaël".to_string(), 4_000);

    account.check_balance();
    account.deposit(100).withdraw(50).view_owner();
    account.view_owner().check_balance();
    account
        .change_owner(String::from("New_Owner"))
        .change_owner(String::from("Another_Owner"))
        .deposit(100)
        .view_owner();
    // println!("Account: {:?}", account); WARN: ownership moved => NOK
}
