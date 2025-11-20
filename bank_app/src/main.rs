use std::collections::HashMap;
use std::io;
use std::process;
use std::sync::{LazyLock, RwLock};

static BANK_ACCOUNTS: LazyLock<RwLock<HashMap<String, AccountDetails>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
pub struct AccountDetails {
    username: String,
    age: u64,
    gender: Gender,
    address: String,
    current_balance: u64,
    last_withdrawal_amount: u64,
}

impl AccountDetails {
    pub fn new(username: String, age: u64, gender: Gender, address: String) -> Self {
        AccountDetails {
            username,
            age,
            gender,
            address,
            current_balance: 0,
            last_withdrawal_amount: 0,
        }
    }
}
fn main() {
    println!(".........Welcome to our Bank App........... ");
    loop {
        println!("What do you want to do??");
        println!("1. Create new account");
        println!("2. Deposit");
        println!("3. Withdraw");
        println!("4. Check Balance");
        println!("5. Delete Account");
        println!("6. View Account Details");
        println!("7. Exit");

        println!("Enter your choice....");

        let mut user_choice: String = String::new();

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to input your choice");

        let user_choice_number: u32 = user_choice.trim().parse().expect("Not a valid choice.");

        match user_choice_number {
            1 => create_new_account(),
            2 => deposit(),
            3 => withdraw(),
            4 => check_balance(),
            5 => delete_account(),
            6 => view_account_details(),
            7 => {
                println!("Exiting.....");
                break;
            }
            _ => println!("Such a choice doesn't exits"),
        }
    }
}
pub fn create_new_account() {
    println!("Enter following Details");
    println!("Username: ");
    let mut username: String = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("failed to read username");

    println!("Age: ");
    let mut age: String = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("failed to read gender");
    let age_number: u64 = age.trim().parse().expect("Not a valid age.");

    println!("Gender: ");
    let mut gender: String = String::new();
    io::stdin()
        .read_line(&mut gender)
        .expect("failed to read gender");

    let gender_str = gender.trim();
    let gender = match gender_str {
        "male" => Gender::Male,
        "female" => Gender::Female,
        _ => panic!("wrong gender"),
    };

    println!("Address: ");
    let mut address: String = String::new();
    io::stdin()
        .read_line(&mut address)
        .expect("failed to read address");

    println!("Creating account....");
    println!("Please wait....");

    let account = AccountDetails::new(username.clone(), age_number, gender, address);

    {
        let mut bank_account = BANK_ACCOUNTS.write().unwrap();
        bank_account.insert(username, account);
    }
    println!("Account created successfully.....");
    println!("You can access you account by your username...");
}

pub fn view_account_details() {
    println!("Enter your username....");
    let mut username: String = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username");

    let bank_account = BANK_ACCOUNTS.read().unwrap();
    if bank_account.contains_key(&username) {
        println!("Your Account Details: \n {:?}", bank_account.get(&username));
    } else {
        println!("No Account on this username {}....", username);
    }
}

pub fn deposit() {
    println!("Enter your username...");
    let mut username: String = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username");

    let mut bank_account = BANK_ACCOUNTS.write().unwrap();
    if bank_account.contains_key(&username) {
        println!("Enter the amount you want to deposit....");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");

        let input: u64 = input.trim().parse().expect("Please enter a valid amount");
        if let Some(account) = bank_account.get_mut(&username) {
            account.current_balance += input;
        }
        println!("Your amount has been successfuly deposited......");
    } else {
        println!("No Account on this username {}....", username);
    }
}

pub fn withdraw() {
    println!("Enter your username...");
    let mut username: String = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username");

    let mut bank_account = BANK_ACCOUNTS.write().unwrap();
    if bank_account.contains_key(&username) {
        println!("Enter the amount you want to withdraw....");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");

        let input: u64 = input.trim().parse().expect("Please enter a valid amount");
        if let Some(account) = bank_account.get_mut(&username) {
            if account.current_balance > input {
                account.current_balance -= input;
                account.last_withdrawal_amount = input;
                println!("You have successfully withdrawed {} amount...", input);
            } else {
                println!("You don't have sufficient balance...");
            }
        }
        println!("Your amount has been successfuly deposited......");
    } else {
        println!("No Account on this username {}....", username);
    }
}
pub fn check_balance() {
    println!("Enter your username...");
    let mut username: String = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username");

    let mut bank_account = BANK_ACCOUNTS.write().unwrap();
    if let Some(account) = bank_account.get(&username) {
        println!("Your current balance is : {}", account.current_balance);
    } else {
        println!("No Account on this username {}....", username);
    }
}

pub fn delete_account() {
    println!("Enter your username...");
    let mut username: String = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username");

    let mut bank_account = BANK_ACCOUNTS.write().unwrap();
    if bank_account.contains_key(&username) {
        bank_account.remove(&username);
        println!("Your account has been successfully removed...");
    } else {
        println!("No Account on this username {}....", username);
    }
}
