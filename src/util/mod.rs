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
    let mut session = Session::new().map_err(|_| format!("Error creating session object"))?;

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
    session
        .userauth_publickey_auto(Some("Test"))
        .map_err(|e| format!("ERROR - {}", e))?;

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

#[cfg(test)]
mod test {
    use super::*;

    const USER_KEY: &str = "TARGET_USER";
    const HOST_KEY: &str = "TARGET_HOST";
    const PORT_KEY: &str = "TARGET_PORT";

    fn env_setup_verify_args(user_key: &str, host_key: &str, port_key: &str) -> VerifyArgs {
        let user = match env::var(user_key) {
            Ok(n) => n,
            Err(_) => panic!("Can't get user environment variable!"),
        };
        let host = match env::var(host_key) {
            Ok(n) => n,
            Err(_) => panic!("Can't get host environment variable!"),
        };
        let port = match env::var(port_key) {
            Ok(n) => n,
            Err(_) => panic!("Can't get port environment variable!"),
        };

        VerifyArgs {
            user: user.to_owned(),
            host: host.to_owned(),
            port: port.parse().unwrap(),
        }
    }

    #[test]
    fn test_verify_connection_pass() {
        let verify_args = env_setup_verify_args(USER_KEY, HOST_KEY, PORT_KEY);
        match verify_connection(verify_args) {
            Ok(_) => assert!(true),
            Err(e) => {
                println!("{}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_verify_connection_user_fail() {
        let mut verify_args = env_setup_verify_args(USER_KEY, HOST_KEY, PORT_KEY);
        verify_args.user = get_random_string(6).to_owned();
        match verify_connection(verify_args) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_verify_connection_host_fail() {
        let mut verify_args = env_setup_verify_args(USER_KEY, HOST_KEY, PORT_KEY);
        verify_args.host = "0.0.0.0.0.0.0.0".to_owned();
        match verify_connection(verify_args) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_verify_connection_port_fail() {
        let mut verify_args = env_setup_verify_args(USER_KEY, HOST_KEY, PORT_KEY);
        verify_args.port = 9999;
        match verify_connection(verify_args) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_get_random_string() {
        for n in 1..101 {
            assert_eq!(get_random_string(n).len(), n);
        }
    }
}
