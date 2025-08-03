use std::io;

fn main() {
    let mut balance: f64 = 1000.0;

    loop {
        println!("\nPlease type your choice number:");
        println!("1 --> View balance");
        println!("2 --> Deposit");
        println!("3 --> Withdraw");
        println!("4 --> Exit");

        let mut user_c = String::new();
        io::stdin()
            .read_line(&mut user_c)
            .expect("Failed to read input.");

        let user_c: u32 = match user_c.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 to 4.");
                continue;
            }
        };

        if user_c == 1 {
            view_balance(balance);
        } else if user_c == 2 {
            balance = deposit(balance);
        } else if user_c == 3 {
            balance = withdraw(balance);
        } else if user_c == 4 {
            println!("Exiting... Goodbye!");
            break;
        } else {
            println!("Option number should be between 1 to 4");
        }
    }
}

fn view_balance(balance: f64) {
    println!("Your current balance is: ${}", balance);
}

fn deposit(balance: f64) -> f64 {
    println!("Enter deposit amount:");

    let mut user_deposit = String::new();
    io::stdin()
        .read_line(&mut user_deposit)
        .expect("Failed to read deposit input.");

    let user_deposit: f64 = match user_deposit.trim().parse() {
        Ok(amount) => amount,
        Err(_) => {
            println!("Invalid input. Deposit cancelled.");
            return balance;
        }
    };

    let new_balance = balance + user_deposit;
    println!("Deposit successful. New balance: ${}", new_balance);
    new_balance
}

fn withdraw(balance: f64) -> f64 {
    println!("Enter withdrawal amount:");

    let mut user_withdraw = String::new();
    io::stdin()
        .read_line(&mut user_withdraw)
        .expect("Failed to read withdrawal input.");

    let user_withdraw: f64 = match user_withdraw.trim().parse() {
        Ok(amount) => amount,
        Err(_) => {
            println!("Invalid input. Withdrawal cancelled.");
            return balance;
        }
    };

    if user_withdraw > balance {
        println!("Insufficient funds. Withdrawal cancelled.");
        return balance;
    }

    let new_balance = balance - user_withdraw;
    println!("Withdrawal successful. New balance: ${}", new_balance);
    new_balance
}
