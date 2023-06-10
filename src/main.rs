#![allow(unused)]

use crate::data_module::sql_module;
use anyhow::{Context, Result};
use clap::Parser;
use std::fmt;
use std::fs;
use std::path;

pub mod data_module;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: path::PathBuf,
}

impl fmt::Display for Cli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Pattern: {}\nPath: {}",
            self.pattern,
            self.path.clone().into_os_string().into_string().unwrap()
        )
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = sql_module::create_pool().await?;

    Ok(())
}
