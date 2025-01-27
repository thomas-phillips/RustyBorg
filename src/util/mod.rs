use clap::Parser;
use log::{error, info, warn};
use rand::{distributions::Alphanumeric, Rng};
use ssh2::Error;
use ssh2::Session;
use std::env;
use std::net::TcpStream;
use std::path::Path;
use std::process;
use tempfile;

const KEY_PATH: &str = "./keys/id_rsa";

// Struct for managing the necessary arguments for verifying an SSH connection.
#[derive(Debug, Clone, Parser)]
pub struct VerifyArgs {
    user: String,
    host: String,
    #[arg(short, long, default_value_t = 22)]
    port: u32,
    #[arg(short, long, default_value_t = String::from(KEY_PATH))]
    key_file: String,
}

pub enum LogLevel {
    Info,
    Warn,
    Error,
}

// Verifies an SSH connection with options defined in the VerifyArgs parameter.
//
// If an error occurs then the error will be propagated to the caller.
pub fn verify_connection(verify_args: VerifyArgs) -> Result<bool, Error> {
    let host_port: String = format!("{}:{}", verify_args.host, verify_args.port);

    let tcp = match TcpStream::connect(&host_port) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error creating TCP stream: {}", e);
            process::exit(1);
        }
    };

    let key_path = Path::new(&verify_args.key_file);

    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_pubkey_file(&verify_args.user, None, key_path, None)?;

    Ok(sess.authenticated())
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

#[allow(dead_code)]
pub fn get_temp_directory() -> String {
    tempfile::tempdir()
        .expect("Failed to create a temporary directory")
        .path()
        .to_str()
        .unwrap()
        .to_owned()
}

#[allow(dead_code)]
pub fn get_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
