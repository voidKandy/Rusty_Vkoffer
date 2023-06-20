#[allow(unused)]
use crate::data_module::sql_module;
use crate::types_module::types;
use anyhow::Result;
use clap::Parser;

pub mod cli_module;
pub mod data_module;
pub mod types_module;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = sql_module::create_pool().await?;
    let mut bank = types::Password::new();
    bank.load_password()?;

    let args = types::Cli::parse();
    println!("{}", args);

    match args.pattern.as_str() {
        "help" => println!(
            "\
Commands: \n
'add': create a new password and add it to the database\n
'show': show all saved passwords\n
'query <string>': query any field by inputted string\n
'update': update currently selected password\n
Selecting a password through query or show will cache it for quick access or updating"
        ),

        "add" => {
            let password = cli_module::prompt_for_password(&pool).await;
            cli_module::create_new_password(password, &pool)
                .await
                .unwrap();
        }
        "show" => {
            bank = cli_module::show_all_passwords(&pool).await;
            bank.save_password()?;
        }
        "query" => {
            bank =
                cli_module::query_passwords(args.command.expect("No Value Passed").as_str(), &pool)
                    .await;
            bank.save_password()?;
        }
        "update" => {
            bank = cli_module::update_password(bank, &pool).await.unwrap();
            bank.save_password()?;
            println!("Password Updated");
        }

        _ => println!("Pattern: {} not found", args.pattern),
    }

    Ok(())
}
