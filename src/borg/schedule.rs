use chrono::Utc;
use chrono_tz::Tz;
use clap::Parser;
use cron::{self, Schedule};
use daemonize::Daemonize;
use std::fs::File;
use std::str::FromStr;
use std::thread;

use crate::util;

use super::create::{create_archive, display_create_info};
use super::errors::parse_archive_error;
use super::init::initialise_repository;
use super::list::verify_repo_location;
use super::{BorgTrait, CreateTrait};

#[derive(Debug, Clone, Parser)]
pub struct ScheduleArgs {
    #[arg(short, long, default_value_t = false)]
    daemonize: bool,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[arg(short, long, default_value = "0 0 * * 1")]
    expression: String,

    #[arg(short, long, default_value = "Etc/UTC")]
    timezone: String,

    #[arg(short, long)]
    repository: String,

    #[arg(short, long)]
    passphrase: String,

    #[arg(short, long)]
    archive: Option<String>,

    #[arg(long, num_args = 1.., value_delimiter = ' ')]
    paths: Vec<String>,

    #[arg(long, num_args = 1.., value_delimiter = ' ')]
    include_patterns: Option<Vec<String>>,

    #[arg(long, num_args = 1.., value_delimiter = ' ')]
    exclude_patterns: Option<Vec<String>>,
}

impl BorgTrait for ScheduleArgs {
    fn repository(&self) -> String {
        self.repository.to_owned()
    }

    fn passphrase(&self) -> String {
        self.passphrase.to_owned()
    }
}

impl CreateTrait for ScheduleArgs {
    fn archive(&self) -> Option<String> {
        self.archive.to_owned()
    }

    fn paths(&self) -> Vec<String> {
        self.paths.to_owned()
    }

    fn include_patterns(&self) -> Option<Vec<String>> {
        self.include_patterns.to_owned()
    }

    fn exclude_patterns(&self) -> Option<Vec<String>> {
        self.exclude_patterns.to_owned()
    }
}

impl ScheduleArgs {
    fn generate_expression(&self) -> Schedule {
        cron::Schedule::from_str(&self.expression).expect("Failed to parse CRON expression")
    }

    fn generate_timezone(&self) -> Tz {
        self.timezone.parse().expect("Failed to parse timezone")
    }
}

fn daemonize_schedule() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = File::create("/tmp/rusty_borg.out")?;
    let stderr = File::create("/tmp/rusty_borg.err")?;

    let daemonize = Daemonize::new()
        .pid_file("/tmp/rusty_borg.pid") // PID file
        .chown_pid_file(true) // Change ownership of the PID file
        .working_directory("/tmp") // for default behaviour.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr); // Redirect stderr to `/tmp/daemon.err`.

    daemonize.start()?;
    Ok(())
}

pub fn schedule_borg(schedule_args: &ScheduleArgs) {
    if schedule_args.daemonize {
        match daemonize_schedule() {
            Ok(()) => util::log_print("Running daemon", util::LogLevel::Info),
            Err(e) => util::log_print(&format!("Error: {}", e), util::LogLevel::Error),
        }
    }

    let schedule = schedule_args.generate_expression();
    let timezone = schedule_args.generate_timezone();

    loop {
        let now = Utc::now().with_timezone(&timezone);
        if let Some(next) = schedule.upcoming(timezone).take(1).next() {
            let until_next = next - now;
            thread::sleep(until_next.to_std().unwrap());
            if !verify_repo_location(&schedule_args.repository, &schedule_args.passphrase) {
                match initialise_repository(schedule_args) {
                    Ok(_) => {
                        util::log_print("Repository successfully created", util::LogLevel::Info)
                    }
                    Err(e) => {
                        util::log_print(&format!("Operation failed: {}", e), util::LogLevel::Error)
                    }
                }
            }

            match create_archive(schedule_args) {
                Ok(n) => {
                    if schedule_args.verbose {
                        display_create_info(n)
                    } else {
                        util::log_print("Archive created!", util::LogLevel::Info)
                    }
                }
                Err(err) => {
                    if schedule_args.verbose {
                        parse_archive_error(err);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_schedule_args() -> ScheduleArgs {
        ScheduleArgs {
            daemonize: false,
            verbose: false,
            expression: "0 0 * * 1".to_owned(),
            timezone: "Etc/UTC".to_owned(),
            repository: "repository".to_owned(),
            passphrase: "passphrase".to_owned(),
            archive: None,
            paths: vec![],
            include_patterns: None,
            exclude_patterns: None,
        }
    }

    #[test]
    fn test_get_repository() {
        let list_args = setup_schedule_args();
        assert_eq!(list_args.repository(), "repository".to_owned());
    }

    #[test]
    fn test_get_passphrase() {
        let list_args = setup_schedule_args();
        assert_eq!(list_args.passphrase(), "passphrase".to_owned());
    }

    #[test]
    fn test_get_archive_some() {
        let mut list_args = setup_schedule_args();
        let expected = "archive".to_owned();
        list_args.archive = Some(expected.clone());
        assert_eq!(list_args.archive(), Some(expected.clone()));
    }

    #[test]
    fn test_get_archive_none() {
        let mut list_args = setup_schedule_args();
        list_args.archive = None;
        assert_eq!(list_args.archive(), None);
    }

    #[test]
    fn test_get_paths() {
        let mut list_args1 = setup_schedule_args();
        list_args1.paths = vec![];
        assert_eq!(list_args1.paths.len(), 0);

        let mut list_args2 = setup_schedule_args();
        list_args2.paths = vec!["test".to_owned()];
        assert_eq!(list_args2.paths.len(), 1);
    }

    #[test]
    fn test_get_include_patterns_some() {
        let mut list_args1 = setup_schedule_args();
        list_args1.include_patterns = Some(vec![]);
        assert_eq!(list_args1.include_patterns().unwrap().len(), 0);

        let mut list_args2 = setup_schedule_args();
        list_args2.include_patterns = Some(vec!["test".to_owned()]);
        assert_eq!(list_args2.include_patterns().unwrap().len(), 1);
        assert_eq!(
            list_args2.include_patterns().unwrap().get(0).unwrap(),
            "test"
        );
    }

    #[test]
    fn test_get_include_patterns_none() {
        let mut list_args1 = setup_schedule_args();
        list_args1.include_patterns = None;
        assert_eq!(list_args1.include_patterns(), None);
    }

    #[test]
    fn test_get_exclude_patterns_some() {
        let mut list_args1 = setup_schedule_args();
        list_args1.exclude_patterns = Some(vec![]);
        assert_eq!(list_args1.exclude_patterns().unwrap().len(), 0);

        let mut list_args2 = setup_schedule_args();
        list_args2.exclude_patterns = Some(vec!["test".to_owned()]);
        assert_eq!(list_args2.exclude_patterns().unwrap().len(), 1);
        assert_eq!(
            list_args2.exclude_patterns().unwrap().get(0).unwrap(),
            "test"
        );
    }

    #[test]
    fn test_get_exclude_patterns_none() {
        let mut list_args1 = setup_schedule_args();
        list_args1.exclude_patterns = None;
        assert_eq!(list_args1.exclude_patterns(), None);
    }
}
