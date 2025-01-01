pub mod create;
pub mod errors;
pub mod init;
pub mod list;
pub mod schedule;

pub trait BorgTrait {
    fn repository(&self) -> String;
    fn passphrase(&self) -> String;
}

pub trait CreateTrait: BorgTrait {
    fn archive(&self) -> Option<String>;
    fn paths(&self) -> Vec<String>;
    fn include_patterns(&self) -> Option<Vec<String>>;
    fn exclude_patterns(&self) -> Option<Vec<String>>;
}
