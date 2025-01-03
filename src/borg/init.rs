use super::BorgTrait;
use borgbackup::common::{CommonOptions, EncryptionMode, InitOptions};
use borgbackup::errors::InitError;
use borgbackup::sync::init;
use clap::Parser;

// Struct for managing the necessary arguments for initialising a repository.
#[derive(Debug, Clone, Parser)]
pub struct InitArgs {
    pub repository: String,
    pub passphrase: String,
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
pub fn initialise_repository(init_args: &impl BorgTrait) -> Result<(), InitError> {
    let init_options = InitOptions {
        repository: init_args.repository(),
        encryption_mode: EncryptionMode::KeyfileBlake2(init_args.passphrase()),
        append_only: false,
        make_parent_dirs: false,
        storage_quota: None,
    };
    let common_options = CommonOptions::default();

    init(&init_options, &common_options)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::super::super::util;
    use super::*;

    fn setup_init_args() -> InitArgs {
        InitArgs {
            repository: String::from("repository"),
            passphrase: String::from("passphrase"),
        }
    }
    #[test]
    fn test_get_repository() {
        let init_args = setup_init_args();
        assert_eq!(init_args.repository, "repository");
    }

    #[test]
    fn test_get_passphrase() {
        let init_args = setup_init_args();
        assert_eq!(init_args.passphrase, "passphrase");
    }

    #[test]
    fn test_initialise_repository_pass() {
        let repo_dir = util::get_temp_directory();
        let mut init_args = setup_init_args();
        init_args.repository = repo_dir;

        match initialise_repository(&init_args) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_initialise_repository_permissions_fail() {
        let repo_dir = "/".to_owned();
        let mut init_args = setup_init_args();
        init_args.repository = repo_dir;

        match initialise_repository(&init_args) {
            Ok(_) => assert!(false),
            Err(e) => match e {
                InitError::DeserializeError(_) => assert!(true),
                _ => assert!(false),
            },
        }
    }

    #[test]
    fn test_initialise_repository_alreadyexists_fail() {
        let repo_dir = util::get_temp_directory();
        let mut init_args1 = setup_init_args();
        init_args1.repository = repo_dir.clone();

        initialise_repository(&init_args1).unwrap();

        let mut init_args2 = setup_init_args();
        init_args2.repository = repo_dir;

        match initialise_repository(&init_args2) {
            Ok(_) => assert!(false),
            Err(e) => match e {
                InitError::RepositoryAlreadyExists => assert!(true),
                _ => assert!(false),
            }
        }
    }
}
