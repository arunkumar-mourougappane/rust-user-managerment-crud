use std::fmt;

use serde::{Deserialize, Serialize};

#[warn(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    active: bool,
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    account_id: Option<u64>,
}

#[allow(dead_code)]
impl User {
    pub fn new(
        active: bool,
        first_name: String,
        last_name: String,
        username: String,
        email: String,
        account_id: u64,
    ) -> User {
        User {
            active,
            first_name,
            last_name,
            username,
            email,
            account_id: Some(account_id),
        }
    }

    pub fn set_last_name(&mut self, last_name: String) {
        self.last_name = last_name;
    }

    pub fn set_first_name(&mut self, first_name: String) {
        self.first_name = first_name;
    }

    pub fn set_active(&mut self) {
        self.active = true;
    }

    pub fn set_inactive(&mut self) {
        self.active = false;
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }
    pub fn get_first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn get_last_name(&self) -> String {
        self.last_name.clone()
    }
    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl fmt::Display for User {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "{:10}|{:20}|{:20}|{:15}|{:20}|{:6}",
            self.get_account_id(),
            self.get_first_name(),
            self.get_last_name(),
            self.get_username(),
            self.get_email(),
            self.is_active()
        )
    }
}
