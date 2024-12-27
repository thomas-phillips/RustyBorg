use borgbackup::common::{CommonOptions, InitOptions};
use borgbackup::sync::init;

pub fn initialise_repository(init_options: &InitOptions) {
    let common_options = CommonOptions::default();

    match init(&init_options, &common_options) {
        Ok(_) => println!("Repository successfully created!"),
        Err(e) => println!("Operation failed: {}", e),
    }
}
