mod user;

use crate::user::User;
use std::collections::HashMap;

fn get_user_info() -> (String, String, String, String) {
    
    println!("First Name: ");
    let mut first_name = String::from("");
    std::io::stdin().read_line(&mut first_name).unwrap();
    println!("Last Name: ");
    let mut last_name = String::from("");
    std::io::stdin().read_line(&mut last_name).unwrap();
    println!("Username: ");
    let mut user_name = String::from("");
    std::io::stdin().read_line(&mut user_name).unwrap();
    println!("Email: ");
    let mut email = String::from("");
    std::io::stdin().read_line(&mut email).unwrap();


    return( first_name.trim().to_owned(),
        last_name.trim().to_owned(),
        user_name.trim().to_owned(),
        email.trim().to_owned());
}

fn add_to_user_database(account_id: u64, user_database: &mut HashMap<u64, User>) {
    let (first_name, last_name, username, email ) =  get_user_info();
    let user_info = User::new  (    true,
        first_name,
        last_name,
        username,
        email,
        account_id
    );
    user_database.insert(account_id, user_info);

}

fn print_user_info(user_database: HashMap<u64, User>) {
    println!( "{}", format!("{:#^1$}", " User Data ", 100));
    println!("{:^10}|{:^20}|{:^20}|{:^15}|{:^20}|{:^10}", "Account ID", "First Name", "Last Name", "Username", "Email Id", "Active" );
    println!( "{}", format!("{:#^1$}", " User Data ", 100));
    if user_database.len() > 0
    {
        for (_account_id, user_info) in user_database {
            println!("{}", user_info);
        }
    }
    else {
        println!("{:-^1$}", " No User Records ", 100);
    }
    println!( "{}", format!("{:#^1$}", " User Data", 100));
}

fn main() {
    let mut user_database:HashMap <u64,User> = HashMap::new();
    println!("hello");
    let mut account_id_index = 1;
    
    let mut no_more_add_user = false;
    while no_more_add_user == false {
        println!("Add New User?\t");
        let mut user_option = String::from("");
        std::io::stdin().read_line(&mut user_option).unwrap();
        if user_option.trim().to_lowercase() != "y"
        {
            println!("Exiting User Addition Logic!");
            no_more_add_user =  true;
        }
        else {
            add_to_user_database(account_id_index, &mut user_database);
            account_id_index += account_id_index;
        }

    }

    print_user_info(user_database);



}
