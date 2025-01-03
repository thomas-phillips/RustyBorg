use clap::Parser;
use log::{error, info, warn};
use rand::{distributions::Alphanumeric, Rng};
use ssh::Session;
use std::env;
use std::process;
use tempfile;

// Struct for managing the necessary arguments for verifying an SSH connection.
#[derive(Debug, Clone, Parser)]
pub struct VerifyArgs {
    user: String,
    host: String,
    #[arg(short, long, default_value_t = 22)]
    port: u32,
}

pub enum LogLevel {
    Info,
    Warn,
    Error,
}

// Verifies an SSH connection with options defined in the VerifyArgs parameter.
//
// If an error occurs then the error will be propagated to the caller.
pub fn verify_connection(verify_args: VerifyArgs) -> Result<(), String> {
    let port_usize = usize::try_from(verify_args.port).unwrap();
    let mut session = Session::new().unwrap();

    session
        .set_username(verify_args.user.as_str())
        .map_err(|e| format!("Error setting username - {}", e))?;
    session
        .set_host(verify_args.host.as_str())
        .map_err(|e| format!("Error setting host - {}", e))?;
    session
        .set_port(port_usize)
        .map_err(|e| format!("Error setting port - {}", e))?;
    session
        .connect()
        .map_err(|e| format!("Error connecting to host - {}", e))?;
    Ok(())
}

fn exiterr_with_message(code: i32, message: &str) {
    log_print(message, LogLevel::Error);
    process::exit(code);
}

pub fn log_print(message: &str, level: LogLevel) {
    if let Ok(_) = env::var("RUST_LOG") {
        match level {
            LogLevel::Info => info!("{}", message),
            LogLevel::Warn => warn!("{}", message),
            LogLevel::Error => error!("{}", message),
        }
    } else {
        match level {
            LogLevel::Error => exiterr_with_message(1, &format!("{}", message)),
            _ => println!("{}", message),
        }
    }
}

pub fn get_temp_directory() -> String {
    tempfile::TempDir::new()
        .expect("Failed to create a temporary directory")
        .path()
        .to_str()
        .unwrap()
        .to_owned()
}

pub fn get_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
