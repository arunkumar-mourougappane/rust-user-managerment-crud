mod data_manager;

use crate::data_manager::user::User;
use pad::PadStr;
use std::{collections::HashMap, fmt, io::{self, Write}, process::{exit, Command}, str::FromStr};

enum UserCrudOptions {
    None = 0,
    Add,
    Read,
    ReadAll,
    Update,
    Deactivate,
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
        }
    }
}

// enum UpdateOptions {
//     None,
//     FirstName,
//     LastName,
//     Email
// }

fn get_user_info() -> (String, String, String, String) {
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

fn print_borderline(border_delimiter: char) {
    println!(
        "{}",
        String::from("").pad(100, border_delimiter, pad::Alignment::Left, false)
    );
}

fn print_custom_padded(string_to_pring:String, delimiter:char, padding_length:usize )
{
    println!(
        "{}",
        string_to_pring.pad( padding_length , delimiter, pad::Alignment::Left, false)
    );
}

fn print_padded(string_to_pring:String, delimiter:char)
{
    println!(
        "{}",
        string_to_pring.pad(100, delimiter, pad::Alignment::Left, false)
    );
}
fn print_header(header_string: String) {
    print_borderline('#');
    println!(
        "{}",
        header_string.pad(100, ' ', pad::Alignment::Middle, false)
    );
    print_borderline('#');
}

fn add_to_user_database(account_id: u64, user_database: &mut HashMap<u64, User>) {
    let (first_name, last_name, username, email) = get_user_info();
    let user_info = User::new(true, first_name, last_name, username, email, account_id);
    user_database.insert(account_id, user_info);
}

fn get_menu_option() -> UserCrudOptions {
    print_header(String::from("User Management"));
    print_header(String::from("Menu"));
    print_padded("1. Add User".to_string(), ' ');
    print_padded("2. View User".to_string(), ' ');
    print_padded("3. View All Users".to_string(), ' ');
    print_padded("4. Edit User".to_string(), ' ');
    print_padded("5. Deactivate User".to_string(), ' ');
    print_borderline('-');
    print!("{}", "Option: ");
    let _ = io::stdout().flush();
    let menu_option: UserCrudOptions;
    let mut input_string: String = "".to_string();
    std::io::stdin().read_line(&mut input_string).unwrap();
    menu_option = input_string.trim().parse().unwrap_or(UserCrudOptions::None);

    return menu_option;
}

fn print_all_user_info(user_database: &HashMap<u64, User>) {
    print_header(String::from(" User Data "));
    println!(
        "{:^10}|{:^20}|{:^20}|{:^15}|{:^20}|{:^10}",
        "Account ID", "First Name", "Last Name", "Username", "Email Id", "Active"
    );
    print_borderline('-');
    if user_database.len() > 0 {
        for (_account_id, user_info) in user_database {
            println!("{}", user_info);
        }
    } else {
        println!("{:-^1$}", " No User Records ", 100);
    }
    print_borderline('-');
}

pub fn clear_terminal_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}

fn print_user_info(user_database: &HashMap<u64, User>){
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
            if user_database.len() == 0  || ! user_database.contains_key(&acc_id) {
                print_padded("Cannot Find User Information!".to_string(), ' ');
            }
            else{
                print_header(format!("Account ID {} Information", acc_id));
                print_custom_padded(format!("First Name: {}", user_database.get(&acc_id).unwrap().get_first_name()), ' ', 20);
                print_custom_padded(format!("Last Name: {}", user_database.get(&acc_id).unwrap().get_last_name()), ' ', 20);
                print_custom_padded(format!("Usermame: {}", user_database.get(&acc_id).unwrap().get_username()), ' ', 20);
                print_custom_padded(format!("Email Id: {}", user_database.get(&acc_id).unwrap().get_email()), ' ', 20);

            }
        }
        Err(..) => println!("Cannot parse input: {}", trimmed),
    };
}

fn main() {

    let mut user_database: HashMap<u64, User> = HashMap::new();

    let mut account_id_index = 1;

    loop {
        let menu_option = get_menu_option();

        match menu_option {
            UserCrudOptions::Add => {
                clear_terminal_screen();
                add_to_user_database(account_id_index, &mut user_database);
                account_id_index += account_id_index;
            }
            UserCrudOptions::Read => {
                clear_terminal_screen();
                print_user_info(&user_database);
            }
            UserCrudOptions::ReadAll => {
                clear_terminal_screen();
                print_all_user_info(&user_database);
            }
            UserCrudOptions::Update => {
                clear_terminal_screen();
                print_padded("Updating Data".to_string(), ' ');
            }
            UserCrudOptions::Deactivate => {
                print_padded("Updating Data".to_string(), ' ');
            },
            UserCrudOptions::None => {
                print_header("Exiting Program!!".to_string());
                exit(0)
            }
        };
        println!("\n\nPress enter to continue...");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to read line");
    }

}
