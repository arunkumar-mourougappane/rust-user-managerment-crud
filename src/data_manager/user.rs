use std::fmt;

#[warn(dead_code)]
#[derive(Debug)]

pub struct User {
    active: bool,
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    account_id: u64,
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
            account_id,
        }
    }

    pub fn set_last_name(&mut self, last_name: String) {
        self.last_name = last_name;
    }

    pub fn set_first_name(&mut self, first_name: String) {
        self.first_name = first_name;
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }
    pub fn get_first_name(&self) -> &str {
      &self.first_name
  }

  pub fn get_last_name(&self) -> &str {
   &self.last_name
}
    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id
    }

    pub fn get_email(&self) -> &str {
        &self.email
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
       write!(f, "{:10}|{:20}|{:20}|{:15}|{:20}|{:6}", self.get_account_id(), self.get_first_name(), self.get_last_name(), self.get_username(), self.get_email(), self.is_active())
   }
}