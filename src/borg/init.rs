use super::super::util;
use super::BorgTrait;
use borgbackup::common::{CommonOptions, EncryptionMode, InitOptions};
use borgbackup::sync::init;
use clap::Parser;

// Struct for managing the necessary arguments for initialising a repository.
#[derive(Debug, Clone, Parser)]
pub struct InitArgs {
    repository: String,
    passphrase: String,
}

impl BorgTrait for InitArgs {
    fn repository(&self) -> String {
        self.repository.to_owned()
    }

    fn passphrase(&self) -> String {
        self.passphrase.to_owned()
    }
}

// The entrypoint for the `init` module where a variable of type
// InitArgs is passed containing the necessary information
// to create a borg repository.
//
// An InitOptions struct is created from consuming InitArgs parameter
// with default CommonOptions used for default behaviour.
//
// The function will print to the `stdout` upon creating a repository,
// else it will printing to the `stderr` if the operation failed.
pub fn initialise_repository(init_args: &impl BorgTrait) {
    let init_options = InitOptions {
        repository: init_args.repository(),
        encryption_mode: EncryptionMode::KeyfileBlake2(init_args.passphrase()),
        append_only: false,
        make_parent_dirs: false,
        storage_quota: None,
    };
    let common_options = CommonOptions::default();

    match init(&init_options, &common_options) {
        Ok(_) => util::log_print("Repository successfully created", util::LogLevel::Info),
        Err(e) => util::log_print(&format!("Operation failed: {}", e), util::LogLevel::Warn),
    }
}
