use borgbackup::common::{CommonOptions, CreateOptions};
use borgbackup::output::create::Create;
use borgbackup::sync::create;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn new_create_options(
    repository: String,
    passphrase: String,
    archive: String,
    pattern_file: String,
    paths: Vec<String>,
) -> CreateOptions {
    let create_options = CreateOptions {
        repository,
        archive,
        passphrase: Some(passphrase),
        comment: None,
        compression: None,
        paths,
        exclude_caches: false,
        patterns: vec![],
        pattern_file: Some(pattern_file),
        excludes: vec![],
        exclude_file: None,
        numeric_ids: false,
        sparse: false,
        read_special: false,
        no_xattrs: false,
        no_acls: false,
        no_flags: false,
    };
    create_options
}

pub fn create_archive(
    repository_path: String,
    passphrase: String,
    archive: Option<String>,
    paths: Vec<String>,
    pattern_file: String,
) {
    let archive_name: String = match archive {
        None => match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => Duration::as_secs(&n).to_string(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        },
        Some(s) => s,
    };

    let create_options = new_create_options(
        repository_path,
        passphrase,
        archive_name,
        pattern_file,
        paths,
    );
    let common_options = CommonOptions::default();

    let create_result: Create = match create(&create_options, &common_options) {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            panic!("Error creating archive: {}", e);
        },
    };

    println!(
        "Successfully created archive at {}::{}",
        create_result.repository.location,
        create_result.archive.name,
    );
    println!("Started at: {}", create_result.archive.start);
    println!("Ended at: {}", create_result.archive.end);
    println!("Took: {}", create_result.archive.duration);
    println!("Commands used: {:?}", create_result.archive.command_line);
}
