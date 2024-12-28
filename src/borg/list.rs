use std::process::exit;
use borgbackup::common::{ ListOptions, CommonOptions };
use borgbackup::sync::list;

pub fn list_contents(repository: String, passphrase: String) {
    let list_options = ListOptions{
        repository,
        passphrase: Some(passphrase)
    };
    let common_options = CommonOptions::default();
    
    let repository_details = match list(&list_options, &common_options) {
        Ok(n) => n,
        Err(e) => panic!("Error listing repository: {}", e)
    };

    println!("Last modified: {}", repository_details.repository.last_modified);

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
        exit(0);
    }
    repository_details.archives.iter().for_each(|archive| {
        println!("ID: {}, Name: {}, Start: {}", archive.id, archive.name, archive.start);
    });
} 
