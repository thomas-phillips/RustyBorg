use super::BorgTrait;
use borgbackup::common::{CommonOptions, ListOptions};
use borgbackup::errors::ListError;
use borgbackup::sync::list;
use clap::Parser;
use super::super::util;

// Struct for managing the necessary arguments for listing a
// repository's details.
#[derive(Debug, Clone, Parser, Default)]
pub struct ListArgs {
    repository: String,
    passphrase: String,
    #[arg(short, long, default_value_t = false)]
    last_modified: bool,
    #[arg(short, long, default_value_t = false)]
    encryption: bool,
    #[arg(short, long, default_value_t = false)]
    archives: bool,
    // TODO: Add list options
}

impl BorgTrait for ListArgs {
    fn repository(&self) -> String {
        self.repository.to_owned()
    }

    fn passphrase(&self) -> String {
        self.passphrase.to_owned()
    }
}

impl ListArgs {
    fn new(repository: &str, passphrase: &str) -> ListArgs {
        let mut list_args = ListArgs::default();
        list_args.repository = repository.to_owned();
        list_args.passphrase = passphrase.to_owned();
        list_args
    }
}

pub fn verify_repo_location(repository: &str, passphrase: &str) -> bool {
    let list_args = ListArgs::new(repository, passphrase);
    match list_contents(list_args) {
        Ok(()) => true,
        Err(_) => false,
    }
}

// The entrypoint for the `list` module where a variable of type
// ListArgs is consumed containing the necessary information
// to list a borg repository's details.
//
// A ListOptions struct is created from the ListArgs parameter
// with default CommonOptions used for default behaviour.
//
// If a repository is not found then the function propagates the error.
// Else the function will display the last modified time,
// encryption used (if any) and the repository's
// archives (if any).
pub fn list_contents(list_args: ListArgs) -> Result<(), ListError> {
    let list_options = ListOptions {
        repository: list_args.repository,
        passphrase: Some(list_args.passphrase),
    };
    let common_options = CommonOptions::default();

    let repository_details = list(&list_options, &common_options)?;

    if list_args.last_modified {
        util::log_print(&format!("Last modified: {}", repository_details.repository.last_modified), util::LogLevel::Info);
    }

    if list_args.encryption {
        let encryption_option = repository_details.encryption;
        if Option::is_some(&encryption_option) {
            let encryption = encryption_option.unwrap();
            util::log_print(&format!("Encryption mode: {:?}", encryption.mode), util::LogLevel::Info);

            match encryption.keyfile {
                Some(n) => util::log_print(&format!("Path of keyfile: {}", n), util::LogLevel::Info),
                None => (),
            }
        } else {
            util::log_print(&format!("Repository includes no encryption!"), util::LogLevel::Info)
        }
    }
    if list_args.archives {
        util::log_print("\nArchives:", util::LogLevel::Info);
        if repository_details.archives.len() == 0 {
            util::log_print("Repository has no archives", util::LogLevel::Warn);
            return Ok(());
        }
        repository_details.archives.iter().for_each(|archive| {
            util::log_print(&format!("ID: {}, Name: {}, Start: {}", archive.id, archive.name, archive.start), util::LogLevel::Info);
        });
    }
    Ok(())
}
