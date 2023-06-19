use crate::cli_module::query_passwords;
use inquire::Text;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use sqlx::FromRow;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::{Index, IndexMut};
// Interaction with a set of / a single saved password
pub mod types {

    use super::*;
    use clap::Parser;

    #[derive(Parser, Debug)]
    pub struct Cli {
        pub pattern: String,
        pub command: Option<String>,
        // path: path::PathBuf,
    }

    impl fmt::Display for Cli {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Pattern: {}\n", self.pattern,)
        }
    }

    #[derive(Debug, FromRow, Serialize, Deserialize)]
    pub struct Password {
        pub id: String,
        pub username: String,
        pub password: String,
        pub service: String,
    }

    impl Password {
        pub fn new() -> Self {
            Password {
                id: String::new(),
                service: String::new(),
                username: String::new(),
                password: String::new(),
            }
        }
        pub fn update_field(&mut self, field: &str, value: &str) {
            match field {
                "id" => self.id = value.to_owned(),
                "service" => self.service = value.to_owned(),
                "username" => self.username = value.to_owned(),
                "password" => self.password = value.to_owned(),
                _ => panic!("unknown field: {}", field),
            }
        }
        pub fn save_password(&self) -> std::io::Result<()> {
            let mut passwords_file = File::create("save_passwords.json")?;
            let pword_buf = serde_json::to_string(&self).unwrap();
            passwords_file.write_all(pword_buf.as_bytes())?;
            Ok(())
        }
        pub fn load_password(&mut self) -> Result<()> {
            let mut passwords_file = File::open("save_passwords.json").unwrap();
            let mut password = String::new();
            passwords_file.read_to_string(&mut password).unwrap();
            *self = serde_json::from_str(&password)?;
            println!("password for {} loaded", self.service);
            Ok(())
        }
    }
    impl fmt::Display for Password {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Id: {}\nService: {}\nUsername: {}\nPassword: {}\n",
                self.id, self.service, self.username, self.password,
            )
        }
    }
}
