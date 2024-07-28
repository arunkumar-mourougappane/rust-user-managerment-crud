use std::io;

use postgres::{Client, NoTls};
use thiserror::Error;

use crate::data_manager::user::User;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PostgresConfig {
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String,
}

#[allow(dead_code)]
impl PostgresConfig {
    pub fn new(
        host: String,
        port: u16,
        database: String,
        username: String,
        password: String,
    ) -> PostgresConfig {
        PostgresConfig {
            host,
            port,
            database,
            username,
            password,
        }
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum PostgressApiError {
    #[error("Database disconnected")]
    Disconnect(#[from] io::Error),
    #[error("Cannot Prepare query statement")]
    FailedQueryStatement,
    #[error("Cannot complete Database transation")]
    FailedTransaction,
    #[error("unknown data store error")]
    Unknown,
}

#[allow(dead_code)]
pub struct PostgresClient {
    postgres_config: PostgresConfig,
    client: Client,
}

#[allow(dead_code)]
impl PostgresClient {
    pub fn new(&mut self, postgres_config: &PostgresConfig) -> PostgresClient {
        PostgresClient {
            postgres_config: postgres_config.clone(),
            client: Client::connect(&postgres_config.connection_string(), NoTls).unwrap(),
        }
    }

    pub fn find_user_by_account_id(&mut self, account_id: u64) -> Result<User, PostgressApiError> {
        let stmt = self.client.prepare("SELECT account_id, first_name, last_name, username, email, active FROM user_info WHERE account_id = $1").unwrap();
        let row = self
            .client
            .query_one(&stmt, &[&account_id.to_string().as_str()]);
        match row {
            Ok(user_info) => {
                let id: String = user_info.get(0);
                let user = User::new(
                    user_info.get("active"),
                    user_info.get("first_name"),
                    user_info.get("last_name"),
                    user_info.get("username"),
                    user_info.get("email"),
                    id.parse::<u64>().unwrap(),
                );
                return Ok(user);
            }
            Err(_) => {
                return Err(PostgressApiError::FailedQueryStatement);
            }
        };
    }

    pub fn find_user_and_id(&mut self, user: User) -> Result<User, PostgressApiError> {
        let stmt = self.client.prepare("SELECT account_id, first_name, last_name, username, email, active FROM user_info WHERE username = $1 and first_name = $2 and last_name = $3 and email = $4").unwrap();
        let row = self.client.query_one(
            &stmt,
            &[
                &user.get_first_name().as_str(),
                &user.get_last_name().as_str(),
                &user.get_username().as_str(),
                &user.get_email().as_str(),
            ],
        );
        match row {
            Ok(user_info) => {
                let id: String = user_info.get(0);
                let user = User::new(
                    user_info.get("active"),
                    user_info.get("first_name"),
                    user_info.get("last_name"),
                    user_info.get("username"),
                    user_info.get("email"),
                    id.parse::<u64>().unwrap(),
                );
                return Ok(user);
            }
            Err(_) => {
                return Err(PostgressApiError::FailedQueryStatement);
            }
        };
    }

    pub fn add_user(&mut self, user: User) -> Result<User, PostgressApiError> {
        let mut transaction = self.client.transaction().unwrap();
        let stmt: Result<postgres::Statement, postgres::Error> = transaction.prepare(
            "INSERT INTO student (first_name, last_name, username, email) VALUES ($1, $2, $3, $4)",
        );
        match stmt {
            Ok(stmt) => {
                let result_set = transaction.execute(
                    &stmt,
                    &[
                        &user.get_first_name().as_str(),
                        &user.get_last_name().as_str(),
                        &user.get_username().as_str(),
                        &user.get_email().as_str(),
                    ],
                );
                match result_set {
                    Ok(count) => {
                        if count == 1 {
                            let _ = transaction.commit();
                            let user_data = self.find_user_and_id(user);
                            match user_data {
                                Ok(user_data) => return Ok(user_data),
                                Err(_) => return Err(PostgressApiError::FailedQueryStatement),
                            };
                        }
                        let _ = transaction.rollback();
                        return Err(PostgressApiError::FailedTransaction);
                    }
                    Err(_) => {
                        match transaction.rollback() {
                            Ok(it) => it,
                            Err(_) => return Err(PostgressApiError::FailedTransaction),
                        };
                    }
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
                return Err(PostgressApiError::FailedQueryStatement);
            }
        }
        return Err(PostgressApiError::Unknown);
    }
}
