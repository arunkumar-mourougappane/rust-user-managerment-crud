mod data_manager;
mod graphics;
mod model_data_access;

use crate::{
    data_manager::user::User,
    graphics::padding::{print_borderline, print_padded},
};
use graphics::padding::{clear_terminal_screen, print_header, print_padded_to_left};
use model_data_access::postgres_api::{PostgresClient, PostgresConfig};

use std::option::Option;
use std::{
    fmt,
    io::{self, Write},
    process::exit,
    str::FromStr,
};

use base64::prelude::*;

enum UserCrudOptions {
    None = 0,
    Add,
    Read,
    ReadAll,
    Update,
    Deactivate,
    Activate,
}

impl FromStr for UserCrudOptions {
    type Err = ();

    fn from_str(input: &str) -> Result<UserCrudOptions, Self::Err> {
        match input {
            "0" => Ok(UserCrudOptions::None),
            "1" => Ok(UserCrudOptions::Add),
            "2" => Ok(UserCrudOptions::Read),
            "3" => Ok(UserCrudOptions::ReadAll),
            "4" => Ok(UserCrudOptions::Update),
            "5" => Ok(UserCrudOptions::Deactivate),
            "6" => Ok(UserCrudOptions::Activate),
            _ => Err(()),
        }
    }
}

impl fmt::Display for UserCrudOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserCrudOptions::None => write!(f, "0"),
            UserCrudOptions::Add => write!(f, "1"),
            UserCrudOptions::Read => write!(f, "2"),
            UserCrudOptions::ReadAll => write!(f, "3"),
            UserCrudOptions::Update => write!(f, "4"),
            UserCrudOptions::Deactivate => write!(f, "5"),
            UserCrudOptions::Activate => write!(f, "6"),
        }
    }
}

enum UpdateOptions {
    None = 0,
    FirstName,
    LastName,
    Email,
}

impl FromStr for UpdateOptions {
    type Err = ();

    fn from_str(input: &str) -> Result<UpdateOptions, Self::Err> {
        match input {
            "0" => Ok(UpdateOptions::None),
            "1" => Ok(UpdateOptions::FirstName),
            "2" => Ok(UpdateOptions::LastName),
            "3" => Ok(UpdateOptions::Email),
            _ => Err(()),
        }
    }
}

impl fmt::Display for UpdateOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UpdateOptions::None => write!(f, "0"),
            UpdateOptions::FirstName => write!(f, "1"),
            UpdateOptions::LastName => write!(f, "2"),
            UpdateOptions::Email => write!(f, "3"),
        }
    }
}

const DB_CREDENTIAL_PASS: &str = "Ni54ZS1nZk1vaVliYV90aGphYzI=";

fn get_user_info_from_stdio() -> (String, String, String, String) {
    print!("First Name: ");
    let _ = io::stdout().flush();
    let mut first_name = String::from("");
    std::io::stdin().read_line(&mut first_name).unwrap();
    print!("Last Name: ");
    _ = io::stdout().flush();
    let mut last_name = String::from("");
    std::io::stdin().read_line(&mut last_name).unwrap();
    print!("Username: ");
    _ = io::stdout().flush();
    let mut user_name = String::from("");
    std::io::stdin().read_line(&mut user_name).unwrap();
    print!("Email: ");
    _ = io::stdout().flush();
    let mut email = String::from("");
    std::io::stdin().read_line(&mut email).unwrap();

    return (
        first_name.trim().to_owned(),
        last_name.trim().to_owned(),
        user_name.trim().to_owned(),
        email.trim().to_owned(),
    );
}

fn get_update_user_property_option() -> UpdateOptions {
    print_header(String::from("User Management"));
    print_header(String::from("Edit User"));
    print_padded("1. First Name".to_string(), ' ');
    print_padded("2. Last Name".to_string(), ' ');
    print_padded("3. Email".to_string(), ' ');
    print_padded("*. Exit Program".to_string(), ' ');
    print_borderline('-');
    print!("Option: ");
    let _ = io::stdout().flush();

    let mut input_string: String = "".to_string();
    std::io::stdin().read_line(&mut input_string).unwrap();
    input_string.trim().parse().unwrap_or(UpdateOptions::None)
}

async fn add_to_user_database() {
    print_header("Add User".to_string());
    let (first_name, last_name, username, email) = get_user_info_from_stdio();
    let user_info = User::new(true, first_name, last_name, username, email, 0);
    let credential =
        String::from_utf8(BASE64_STANDARD.decode(DB_CREDENTIAL_PASS).unwrap()).unwrap();

    let postgres_config = PostgresConfig::new(
        "localhost".to_string(),
        5432,
        "studentdb".to_string(),
        "rustapp".to_string(),
        credential,
    );
    let postgres_cli = PostgresClient::new(&postgres_config);
    match postgres_cli.await.add_user(user_info).await {
        Ok(user_info) => {
            print_user_info(&user_info);
        }
        Err(_) => {
            println!("Failed to add user.");
        }
    };
}

fn get_menu_option() -> UserCrudOptions {
    print_header(String::from("User Management"));
    print_header(String::from("Menu"));
    print_padded("1. Add User".to_string(), ' ');
    print_padded("2. View User".to_string(), ' ');
    print_padded("3. View All Users".to_string(), ' ');
    print_padded("4. Edit User".to_string(), ' ');
    print_padded("5. Deactivate User".to_string(), ' ');
    print_padded("6. Activate User".to_string(), ' ');
    print_padded("*. Exit Program".to_string(), ' ');
    print_borderline('-');
    print!("Option: ");
    let _ = io::stdout().flush();
    let mut input_string: String = "".to_string();
    std::io::stdin().read_line(&mut input_string).unwrap();

    input_string.trim().parse().unwrap_or(UserCrudOptions::None)
}

async fn print_all_user_info() {
    print_header(String::from(" User Data "));
    let credential =
        String::from_utf8(BASE64_STANDARD.decode(DB_CREDENTIAL_PASS).unwrap()).unwrap();

    let postgres_config = PostgresConfig::new(
        "localhost".to_string(),
        5432,
        "studentdb".to_string(),
        "rustapp".to_string(),
        credential,
    );
    let postgres_cli = PostgresClient::new(&postgres_config);
    let user_data = postgres_cli.await.get_all_users().await;

    match user_data {
        Ok(user_data) => {
            println!(
                "{:^10}|{:^20}|{:^20}|{:^15}|{:^20}|{:^10}",
                "Account ID", "First Name", "Last Name", "Username", "Email Id", "Active"
            );
            print_borderline('-');
            for (_account_id, user_info) in user_data {
                println!("{}", user_info);
            }
        }
        Err(_) => println!("{:-^1$}", " No User Records ", 100),
    };

    print_borderline('-');
}

fn print_user_info(user_info: &User) {
    print_header(format!(
        "Account ID {} Information",
        user_info.get_account_id()
    ));
    print_padded_to_left(
        format!("First Name: {}", user_info.get_first_name()),
        ' ',
        20,
    );
    print_padded_to_left(format!("Last Name: {}", user_info.get_last_name()), ' ', 20);
    print_padded_to_left(format!("Usermame: {}", user_info.get_username()), ' ', 20);
    print_padded_to_left(format!("Email Id: {}", user_info.get_email()), ' ', 20);
    print_padded_to_left(
        format!(
            "Active Status: {}",
            if user_info.is_active() { "Yes" } else { "No" }
        ),
        ' ',
        20,
    );
}

async fn print_user_info_by_id() -> u64 {
    let credential =
        String::from_utf8(BASE64_STANDARD.decode(DB_CREDENTIAL_PASS).unwrap()).unwrap();

    let postgres_config = PostgresConfig::new(
        "localhost".to_string(),
        5432,
        "studentdb".to_string(),
        "rustapp".to_string(),
        credential,
    );
    let postgres_cli = PostgresClient::new(&postgres_config);
    print_header("User Information".to_string());
    print_borderline('-');
    print!(" Account Id: ");
    let _ = io::stdout().flush();

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u64>() {
        Ok(acc_id) => {
            let user_data = postgres_cli.await.find_user_by_account_id(acc_id).await;
            match user_data {
                Ok(user) => {
                    print_user_info(&user);
                    return acc_id;
                }
                Err(_) => {
                    return 0;
                }
            }
        }
        Err(..) => println!("Cannot parse input: {}", trimmed),
    }
    0
}

async fn deactivate_activate_user_by_id(activate_flag: bool) {
    print_header("User Activation/Deactivation".to_string());
    print_borderline('-');
    print!(" Account Id: ");
    let _ = io::stdout().flush();

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u64>() {
        Ok(acc_id) => {
            let user_data = find_user_by_account_id(acc_id).await;
            match user_data {
                Some(user) => {
                    print_user_info(&user);
                    print!(
                        "Do you want to {} user (Y/N)?  ",
                        if activate_flag {
                            "activate"
                        } else {
                            "deactivate"
                        }
                    );
                    _ = io::stdout().flush();
                    let mut input_text = String::new();
                    io::stdin()
                        .read_line(&mut input_text)
                        .expect("failed to read from stdin");
                    if input_text.trim().to_lowercase() == "y" {
                        let credential =
                            String::from_utf8(BASE64_STANDARD.decode(DB_CREDENTIAL_PASS).unwrap())
                                .unwrap();

                        let postgres_config = PostgresConfig::new(
                            "localhost".to_string(),
                            5432,
                            "studentdb".to_string(),
                            "rustapp".to_string(),
                            credential,
                        );
                        let postgres_cli = PostgresClient::new(&postgres_config);
                        let activation_result = postgres_cli
                            .await
                            .activate_deactivate_user(acc_id, activate_flag)
                            .await;

                        match activation_result {
                            Ok(user) => print_user_info(&user),
                            Err(_) => {
                                println!(
                                    "Failed to {} user account.",
                                    if activate_flag {
                                        "activate"
                                    } else {
                                        "deacivate"
                                    }
                                );
                            }
                        }
                    }
                }
                None => {
                    print_padded("Cannot Find User Information!".to_string(), ' ');
                }
            }
        }
        Err(..) => println!("Cannot parse input: {}", trimmed),
    };
}

async fn update_user_infomation_by_id() {
    let account_id: u64 = print_user_info_by_id().await;
    if account_id != 0 {
        let menu_option = get_update_user_property_option();
        if let UpdateOptions::None = menu_option {
            println!("Skip Updating User Information.");
            return;
        }
        let mut user_info = find_user_by_account_id(account_id).await.unwrap();
        print_borderline('#');
        match menu_option {
            UpdateOptions::FirstName => {
                println!("Current First Name: {}", user_info.get_first_name());
                print!("New First Name: ");
                let _ = io::stdout().flush();
                let mut first_name = String::from("");
                std::io::stdin().read_line(&mut first_name).unwrap();

                user_info.set_first_name(first_name.trim().to_owned());
            }
            UpdateOptions::LastName => {
                println!("Current Last Name: {}", user_info.get_last_name());
                print!("New Last Name: ");
                let _ = io::stdout().flush();
                let mut last_name = String::from("");
                std::io::stdin().read_line(&mut last_name).unwrap();

                user_info.set_last_name(last_name.trim().to_owned());
            }
            UpdateOptions::Email => {
                println!("Current Email Id: {}", user_info.get_email());
                print!("New Email Id: ");
                let _ = io::stdout().flush();
                let mut email = String::from("");
                std::io::stdin().read_line(&mut email).unwrap();

                user_info.set_email(email.trim().to_owned());
            }
            UpdateOptions::None => {
                print_header(format!(
                    "No data would be updated for account Id: {}",
                    account_id
                ));
            }
        }
        let credential =
            String::from_utf8(BASE64_STANDARD.decode(DB_CREDENTIAL_PASS).unwrap()).unwrap();
        let postgres_config = PostgresConfig::new(
            "localhost".to_string(),
            5432,
            "studentdb".to_string(),
            "rustapp".to_string(),
            credential,
        );
        let postgres_cli = PostgresClient::new(&postgres_config);
        let updateed_user = postgres_cli
            .await
            .update_user_infomation_by_id(user_info)
            .await;
        match updateed_user {
            Ok(user) => print_user_info(&user),
            Err(_) => println!("Cannot update user information."),
        }

        print_header("Updated User Information Successfully!".to_string());
    } else {
        print_header("Skip Updating User Information.".to_string());
    }
}

async fn find_user_by_account_id(account_id: u64) -> Option<User> {
    let credential =
        String::from_utf8(BASE64_STANDARD.decode(DB_CREDENTIAL_PASS).unwrap()).unwrap();
    let postgres_config = PostgresConfig::new(
        "localhost".to_string(),
        5432,
        "studentdb".to_string(),
        "rustapp".to_string(),
        credential,
    );
    let postgres_cli = PostgresClient::new(&postgres_config);
    let result_set = postgres_cli.await.find_user_by_account_id(account_id).await;
    match result_set {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    clear_terminal_screen();

    loop {
        let menu_option = get_menu_option();

        match menu_option {
            UserCrudOptions::Add => {
                add_to_user_database().await;
            }
            UserCrudOptions::Read => {
                print_user_info_by_id().await;
            }
            UserCrudOptions::ReadAll => {
                print_all_user_info().await;
            }
            UserCrudOptions::Update => {
                update_user_infomation_by_id().await;
            }
            UserCrudOptions::Deactivate => {
                deactivate_activate_user_by_id(false).await;
            }
            UserCrudOptions::None => {
                print_header("Exiting Program!!".to_string());
                exit(0)
            }
            UserCrudOptions::Activate => {
                deactivate_activate_user_by_id(true).await;
            }
        };
        println!("\n\nPress enter to continue...");
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
        clear_terminal_screen();
    }
}
