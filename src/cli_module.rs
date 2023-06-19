use crate::data_module::sql_module;
use crate::types::Password;
use anyhow::Result;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use inquire::InquireError;
use inquire::MultiSelect;
use inquire::Select;
use inquire::Text;
use sqlx::Connection;
use std::io::Error;
use std::ops::IndexMut;

pub async fn create_new_password(password: Password, pool: &sqlx::PgPool) -> Result<()> {
    sql_module::insert(&password, pool).await
}

fn select_from_choices(choices: Vec<Password>) -> Password {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let choice: Result<Password, InquireError> = Select::new("Choose a password", choices).prompt();

    let selected_password = match choice {
        Ok(password) => password,
        Err(_) => panic!("Error selecting password."),
    };

    ctx.set_contents(selected_password.password.clone())
        .unwrap();

    selected_password
}

pub async fn prompt_for_password(pool: &sqlx::PgPool) -> Password {
    let service = Text::new("What is the service?").prompt().unwrap();
    let username = Text::new("What is the username?").prompt().unwrap();
    let password = Text::new("What is the password?").prompt().unwrap();

    let mut temp_pass = Password {
        id: "".to_string(),
        username: username.to_string(),
        password: password.to_string(),
        service: service.to_string(),
    };
    match sql_module::password_exist(&temp_pass, pool).await {
        true => temp_pass,
        false => {
            temp_pass.generate_uuid();
            temp_pass
        }
    }
}

pub async fn show_all_passwords(pool: &sqlx::PgPool) -> Password {
    let choices: Vec<Password> = match sql_module::read(&pool).await {
        Ok(pwords) => pwords,
        Err(error) => {
            println!("Error: {:?}", error);
            Vec::new()
        }
    };
    select_from_choices(choices)
}

pub async fn query_passwords(val: &str, pool: &sqlx::PgPool) -> Password {
    let options: Vec<&str> = vec!["service", "username", "password"];
    let ans: Result<&str, InquireError> = Select::new("By which field?", options).prompt();

    let choices: Vec<Password> = match ans {
        Ok(choice) => match sql_module::read_by_key(choice, val, &pool).await {
            Ok(pwords) => pwords,
            Err(error) => {
                println!("Error: {:?}", error);
                Vec::new()
            }
        },
        Err(_) => {
            println!("There was an error, please try again");
            Vec::new()
        }
    };

    select_from_choices(choices)
}

pub async fn update_password(
    mut password: Password,
    pool: &sqlx::PgPool,
) -> Result<Password, anyhow::Error> {
    println!("Currently Updating: \n{}", password);
    let options: &[&str] = &["service", "username", "password"];
    let ans: Result<Vec<&str>, InquireError> =
        MultiSelect::new("Update which fields?", options.to_vec()).prompt();
    match ans {
        Ok(fields) => {
            for field in fields {
                let response = Text::new(&format!("new {}:  ", field)).prompt()?;
                password.update_field(field, &response);
                println!("{}:{}", field, response);
            }
        }
        Err(error) => {
            panic!("No fields provided");
        }
    }
    Ok(password)
}
