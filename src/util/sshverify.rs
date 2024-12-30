use clap::Parser;
use ssh::Session;

// Struct for managing the necessary arguments for verifying an SSH connection.
#[derive(Debug, Clone, Parser)]
pub struct VerifyArgs {
    user: String,
    host: String,
    #[arg(short, long, default_value_t = 22)]
    port: u32,
}

// Verifies an SSH connection with options defined in the VerifyArgs parameter.
//
// If an error occurs then the error will be propagated to the caller.
pub fn verify_connection(verify_args: &VerifyArgs) -> Result<(), String> {

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
