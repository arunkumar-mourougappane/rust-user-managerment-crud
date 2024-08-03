use std::{collections::HashMap, io};

use crate::data_manager::user::User;
use sqlx::{Pool, Postgres};
use thiserror::Error;

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
    pool: Pool<Postgres>,
}

#[allow(dead_code)]
impl PostgresClient {
    pub async fn new(postgres_config: &PostgresConfig) -> PostgresClient {
        PostgresClient {
            postgres_config: postgres_config.clone(),
            pool: {
                let database_url = postgres_config.connection_string();
                let connect = Pool::connect(&database_url);
                connect.await.unwrap()
            },
        }
    }

    pub async fn get_all_users(&mut self) -> Result<HashMap<u64, User>, PostgressApiError> {
        let result_set = sqlx::query!(
            r#"SELECT account_id, first_name, last_name, username, email, active FROM user_info"#,
        )
        .fetch_all(&self.pool)
        .await;
        match result_set {
            Ok(result_set) => {
                let mut user_map = HashMap::<u64, User>::new();
                for user_info in result_set {
                    let user = User::new(
                        user_info.active,
                        user_info.first_name,
                        user_info.last_name,
                        user_info.username,
                        user_info.email,
                        user_info.account_id.try_into().unwrap(),
                    );
                    user_map.insert(user.get_account_id(), user);
                }
                Ok(user_map)
            }
            Err(_) => Err(PostgressApiError::FailedQueryStatement),
        }
    }

    pub async fn get_inactive_users(&mut self) -> Result<HashMap<u64, User>, PostgressApiError> {
        let result_set = sqlx::query!(
            r#"SELECT account_id, first_name, last_name, username, email, active FROM user_info WHERE active = false"#,
        )
        .fetch_all(&self.pool)
        .await;

        match result_set {
            Ok(result_set) => {
                let mut inactive_users = HashMap::<u64, User>::new();
                for user_info in result_set {
                    let user = User::new(
                        user_info.active,
                        user_info.first_name,
                        user_info.last_name,
                        user_info.username,
                        user_info.email,
                        user_info.account_id.try_into().unwrap(),
                    );
                    inactive_users.insert(user.get_account_id(), user);
                }
                Ok(inactive_users)
            }
            Err(_) => Err(PostgressApiError::FailedQueryStatement),
        }
    }

    pub async fn get_active_users(&mut self) -> Result<HashMap<u64, User>, PostgressApiError> {
        let result_set = sqlx::query!(
            r#"SELECT account_id, first_name, last_name, username, email, active FROM user_info WHERE active = false"#,
        )
        .fetch_all(&self.pool)
        .await;

        match result_set {
            Ok(result_set) => {
                let mut active_users = HashMap::<u64, User>::new();
                for user_info in result_set {
                    let user = User::new(
                        user_info.active,
                        user_info.first_name,
                        user_info.last_name,
                        user_info.username,
                        user_info.email,
                        user_info.account_id.try_into().unwrap(),
                    );
                    active_users.insert(user.get_account_id(), user);
                }
                Ok(active_users)
            }
            Err(_) => Err(PostgressApiError::FailedQueryStatement),
        }
    }

    pub async fn find_user_by_account_id(
        &mut self,
        account_id: u64,
    ) -> Result<User, PostgressApiError> {
        let result_set = sqlx::query!(
            r#"SELECT account_id, first_name, last_name, username, email, active FROM user_info WHERE account_id = $1"#, account_id as i32)
            .fetch_optional(&self.pool)
            .await;

        match result_set {
            Ok(user_info) => match user_info {
                Some(user) => Ok(User::new(
                    user.active,
                    user.first_name,
                    user.last_name,
                    user.username,
                    user.email,
                    user.account_id.try_into().unwrap(),
                )),
                None => Err(PostgressApiError::FailedQueryStatement),
            },
            Err(_) => Err(PostgressApiError::FailedQueryStatement),
        }
    }

    pub async fn find_user_and_id(&mut self, user: User) -> Result<User, PostgressApiError> {
        let result_set = sqlx::query!(
            r#"SELECT account_id, first_name, last_name, username, email, active FROM user_info WHERE username = $1 and first_name = $2 and last_name = $3 and email = $4"#, user.get_username(), user.get_first_name(), user.get_last_name(), user.get_email())
            .fetch_optional(&self.pool)
            .await;

        match result_set {
            Ok(user_info) => match user_info {
                Some(user_data) => Ok(User::new(
                    user_data.active,
                    user_data.first_name,
                    user_data.last_name,
                    user_data.username,
                    user_data.email,
                    user_data.account_id.try_into().unwrap(),
                )),
                None => Err(PostgressApiError::FailedQueryStatement),
            },
            Err(_) => Err(PostgressApiError::FailedQueryStatement),
        }
    }

    pub async fn activate_deactivate_user(
        &mut self,
        account_id: u64,
        active: bool,
    ) -> Result<User, PostgressApiError> {
        let result_set = sqlx::query(
            r#"
                UPDATE user_info SET "active" = $1 WHERE account_id = $2
            "#,
        )
        .bind(active)
        .bind(account_id as i32)
        .execute(&self.pool)
        .await;
        match result_set {
            Ok(result_data) => {
                if result_data.rows_affected() != 0 {
                    self.find_user_by_account_id(account_id).await
                } else {
                    println!("Cannot find user data.");
                    Err(PostgressApiError::FailedQueryStatement)
                }
            }
            Err(error) => {
                println!("{}", error);
                Err(PostgressApiError::FailedQueryStatement)
            }
        }
    }

    pub async fn update_user_infomation_by_id(
        &mut self,
        user: User,
    ) -> Result<User, PostgressApiError> {
        let result_set = sqlx::query(
            r#"
                UPDATE user_info SET first_name = $1, last_name = $2, username = $3, email = $4 WHERE account_id = $5 and active = true
            "#,
        )
        .bind(user.get_first_name())
        .bind(user.get_last_name())
        .bind(user.get_username())
        .bind(user.get_email())
        .bind(user.get_account_id() as i32)
        .execute(&self.pool)
        .await;
        match result_set {
            Ok(result_data) => {
                if result_data.rows_affected() != 0 {
                    self.find_user_by_account_id(user.get_account_id()).await
                } else {
                    println!("Cannot find user data.");
                    Err(PostgressApiError::FailedQueryStatement)
                }
            }
            Err(error) => {
                println!("{}", error);
                Err(PostgressApiError::FailedQueryStatement)
            }
        }
    }

    pub async fn add_user(&mut self, user: User) -> Result<User, PostgressApiError> {
        let result_set = sqlx::query(
            r#"
                INSERT INTO user_info (first_name, last_name, username, email) VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(user.get_first_name())
        .bind(user.get_last_name())
        .bind(user.get_username())
        .bind(user.get_email())
        .execute(&self.pool)
        .await;
        match result_set {
            Ok(result_data) => {
                if result_data.rows_affected() != 0 {
                    self.find_user_and_id(user.clone()).await
                } else {
                    println!("Cannot find user data.");
                    Err(PostgressApiError::FailedQueryStatement)
                }
            }
            Err(error) => {
                println!("{}", error);
                Err(PostgressApiError::FailedQueryStatement)
            }
        }
    }
}
