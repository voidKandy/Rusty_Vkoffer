use crate::cli_module;
use crate::types::Password;
use serde_json::{Result, Value};
use std::fs::File;
use std::io::{Read, Write};

pub async fn load_up_decrypts(pool: &sqlx::PgPool) -> Result<()> {
    let mut passwords_file = File::open("decrypted_passwords.json").unwrap();
    let mut password = String::new();
    passwords_file.read_to_string(&mut password).unwrap();
    // println!("{}", password);

    let jsoned: Value = serde_json::from_str(&password)?;

    if let Some(json_array) = jsoned.as_array() {
        for json_obj in json_array {
            let mut new_password: Password = Password::new();

            if let Some(password_value) = json_obj.get("password") {
                println!("{}", password_value);
                new_password.update_field("password", password_value.as_str().unwrap());
            }

            if let Some(user_value) = json_obj.get("user") {
                println!("{}", user_value);
                new_password.update_field("username", user_value.as_str().unwrap());
            }

            if let Some(website_value) = json_obj.get("website") {
                println!("{}", website_value);
                new_password.update_field("service", website_value.as_str().unwrap());
            }
            new_password.generate_uuid();
            // println!("{}", new_password);
            cli_module::create_new_password(new_password, pool).await;
        }
    }
    Ok(())
}
