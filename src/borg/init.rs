use borgbackup::common::{CommonOptions, EncryptionMode, InitOptions};
use borgbackup::sync::init;
use clap::Parser;

// Struct for managing the necessary arguments for initialising a repository.
#[derive(Debug, Clone, Parser)]
pub struct InitArgs {
    repository: String,
    passphrase: String,
}

impl InitArgs {
    pub fn new(repository: String, passphrase: String) -> InitArgs {
        InitArgs {repository, passphrase}
    }
}

// The entrypoint for the `init` module where a variable of type
// InitArgs is passed containing the necessary information
// to create a borg repository.
//
// An InitOptions struct is created from the InitArgs parameter
// with default CommonOptions used for default behaviour.
//
// The function will print to the `stdout` upon creating a repository,
// else it will printing to the `stderr` if the operation failed.
pub fn initialise_repository(init_args: &InitArgs) {
    let init_options = InitOptions {
        repository: init_args.repository.clone(),
        encryption_mode: EncryptionMode::KeyfileBlake2(init_args.passphrase.clone()),
        append_only: false,
        make_parent_dirs: false,
        storage_quota: None,
    };
    let common_options = CommonOptions::default();

    match init(&init_options, &common_options) {
        Ok(_) => println!("Repository successfully created!"),
        Err(e) => println!("Operation failed: {}", e),
    }
}
