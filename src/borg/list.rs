use borgbackup::common::{CommonOptions, ListOptions};
use borgbackup::errors::ListError;
use borgbackup::sync::list;
use clap::Parser;

// Struct for managing the necessary arguments for listing a
// repository's details.
#[derive(Debug, Clone, Parser)]
pub struct ListArgs {
    repository: String,
    passphrase: String,
    // TODO: Add list options
}

impl ListArgs {
    pub fn new(repository: String, passphrase: String) -> ListArgs {
        ListArgs {repository, passphrase}
    }
}

// The entrypoint for the `list` module where a variable of type
// ListArgs is passed containing the necessary information
// to list a borg repository's details.
//
// A ListOptions struct is created from the ListArgs parameter
// with default CommonOptions used for default behaviour.
//
// If a repository is not found then the function propagates the error.
// Else the function will display the last modified time,
// encryption used (if any) and the repository's
// archives (if any).
pub fn list_contents(list_args: &ListArgs) -> Result<(), ListError> {
    let list_options = ListOptions {
        repository: list_args.repository.clone(),
        passphrase: Some(list_args.passphrase.clone()),
    };
    let common_options = CommonOptions::default();

    let repository_details = list(&list_options, &common_options)?; 

    println!(
        "Last modified: {}",
        repository_details.repository.last_modified
    );

    let encryption_option = repository_details.encryption;
    if Option::is_some(&encryption_option) {
        let encryption = encryption_option.unwrap();
        println!("Encryption mode: {:?}", encryption.mode);

        match encryption.keyfile {
            Some(n) => println!("Path of keyfile: {}", n),
            None => (),
        }
    } else {
        println!("Repository includes no encryption!")
    }

    println!("\nArchives:");
    if repository_details.archives.len() == 0 {
        println!("Repository has no archives");
        return Ok(())
    }
    repository_details.archives.iter().for_each(|archive| {
        println!(
            "ID: {}, Name: {}, Start: {}",
            archive.id, archive.name, archive.start
        );
    });
    Ok(())
}
