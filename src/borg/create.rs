use super::super::util;
use super::errors::ArchiveError;
use super::{BorgTrait, CreateTrait};
use borgbackup::common::{CommonOptions, CreateOptions, Pattern, PatternInstruction};
use borgbackup::output::create::Create;
use borgbackup::sync::create;
use clap::Parser;
use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};

// Struct for managing the necessary arguments for creating an archive.
#[derive(Debug, Clone, Parser)]
pub struct CreateArgs {
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

impl BorgTrait for CreateArgs {
    fn repository(&self) -> String {
        self.repository.to_owned()
    }

    fn passphrase(&self) -> String {
        self.passphrase.to_owned()
    }
}

impl CreateTrait for CreateArgs {
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

// Creates a CreateOption struct using the struct's `new`
// then manually sets the passphrase after `new` is called.
fn new_create_options(
    repository: String,
    passphrase: String,
    paths: Vec<String>,
    archive: String,
    pattern_instructions: Vec<PatternInstruction>,
) -> CreateOptions {
    let mut create_options = CreateOptions::new(repository, archive, paths, pattern_instructions);
    create_options.passphrase = Some(passphrase);
    create_options
}

// Prints the command used for the BorgBackup crate.
fn print_used_command(commands: Vec<String>) {
    let mut command = String::new();
    for c in commands.iter() {
        command.push_str(c);
    }
    util::log_print(&format!("Command used: {}", command), util::LogLevel::Info);
}

// This function generates a Vector of `PatternInstruction`
// based upon a provided Option Vector of type String
// If `include_patterns` and `exclude_patterns` are of type None then an
// empty Vector of type String will be returned.
fn generate_pattern_instructions(
    include_patterns: Option<Vec<String>>,
    exclude_patterns: Option<Vec<String>>,
) -> Vec<PatternInstruction> {
    let include_pattern_instruction: Vec<PatternInstruction> = match include_patterns {
        Some(n) => n
            .into_iter()
            .map(|x| PatternInstruction::Include(Pattern::Shell(x)))
            .collect(),
        None => Vec::new(),
    };
    let exclude_pattern_instruction: Vec<PatternInstruction> = match exclude_patterns {
        Some(n) => n
            .into_iter()
            .map(|x| PatternInstruction::Exclude(Pattern::Shell(x)))
            .collect(),
        None => Vec::new(),
    };

    return vec![include_pattern_instruction, exclude_pattern_instruction].concat();
}

fn get_epoch_name() -> Result<String, SystemTimeError> {
    let epoch_duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(Duration::as_secs(&epoch_duration).to_string())
}

pub fn display_create_info(create_result: Create) {
    util::log_print(
        &format!(
            "Successfully created archive at {}::{}",
            create_result.repository.location, create_result.archive.name
        ),
        util::LogLevel::Info,
    );
    util::log_print(
        &format!("Started at: {}", create_result.archive.start),
        util::LogLevel::Info,
    );
    util::log_print(
        &format!("Ended at: {}", create_result.archive.end),
        util::LogLevel::Info,
    );
    util::log_print(
        &format!("Took: {}", create_result.archive.duration),
        util::LogLevel::Info,
    );
    print_used_command(create_result.archive.command_line);
}

// This is the entrypoint of the **create** module where variable of type
// CreateArgs is consumed containing the necessary information
// to create a borg archive.
//
// The archive name will be automatically set to epoch time if isn't set,
// and pattern instructions are generated from include and excude Vectors.
//
// Upon a successful archive creation the start and end time, duration and
// commands used are displayed.
pub fn create_archive(create_args: &impl CreateTrait) -> Result<Create, ArchiveError> {
    let archive_name: String = create_args.archive().unwrap_or(match get_epoch_name() {
        Ok(n) => n,
        Err(_) => return Err(ArchiveError::EpochTimeError),
    });

    let pattern_instructions = generate_pattern_instructions(
        create_args.include_patterns(),
        create_args.exclude_patterns(),
    );

    let create_options = new_create_options(
        create_args.repository(),
        create_args.passphrase(),
        create_args.paths(),
        archive_name,
        pattern_instructions,
    );
    let common_options = CommonOptions::default();

    match create(&create_options, &common_options) {
        Ok(n) => return Ok(n),
        Err(e) => return Err(ArchiveError::ArchiveCreateError(e)),
    };
}

#[cfg(test)]
mod tests {
    use super::super::super::util;
    use super::super::init;
    use super::*;

    fn setup_create_args() -> CreateArgs {
        CreateArgs {
            repository: String::from("repository"),
            passphrase: String::from("passphrase"),
            archive: Some(String::from("archive")),
            paths: Vec::new(),
            include_patterns: Some(Vec::new()),
            exclude_patterns: Some(Vec::new()),
        }
    }

    #[test]
    fn test_get_repository() {
        let create_args = setup_create_args();
        assert_eq!(create_args.repository(), "repository")
    }

    #[test]
    fn test_get_passphrase() {
        let create_args = setup_create_args();
        assert_eq!(create_args.passphrase(), "passphrase")
    }

    #[test]
    fn test_archive_some() {
        let create_args = setup_create_args();
        assert_eq!(create_args.archive(), Some("archive".to_owned()));
    }

    #[test]
    fn test_archive_none() {
        let mut create_args = setup_create_args();
        create_args.archive = None;
        assert_eq!(create_args.archive(), None);
    }

    #[test]
    fn test_paths() {
        let result: Vec<String> = Vec::new();
        assert_eq!(result.len(), 0);

        let create_args = setup_create_args();
        assert_eq!(create_args.paths.len(), 0);

        assert_eq!(create_args.paths(), result);
    }

    #[test]
    fn test_include_patterns_some() {
        let create_args = setup_create_args();
        let result: Option<Vec<String>> = Some(Vec::new());
        assert_eq!(create_args.include_patterns(), result);
    }

    #[test]
    fn test_include_patterns_none() {
        let mut create_args = setup_create_args();
        create_args.include_patterns = None;
        assert_eq!(create_args.include_patterns(), None);
    }

    #[test]
    fn test_exclude_patterns_some() {
        let create_args = setup_create_args();
        let result: Option<Vec<String>> = Some(Vec::new());
        assert_eq!(create_args.exclude_patterns(), result);
    }

    #[test]
    fn test_exclude_patterns_none() {
        let mut create_args = setup_create_args();
        create_args.exclude_patterns = None;
        assert_eq!(create_args.exclude_patterns(), None);
    }

    #[test]
    fn test_new_create_options() {
        let create_options = new_create_options(
            "repository".to_owned(),
            "passphrase".to_owned(),
            Vec::new(),
            "archive".to_owned(),
            Vec::new(),
        );
        assert_eq!(create_options.repository, "repository");
        assert_eq!(create_options.passphrase, Some("passphrase".to_owned()));
        assert_eq!(create_options.paths.len(), 0);
        assert_eq!(create_options.archive, "archive");
        assert_eq!(create_options.patterns.len(), 0);
    }
    #[test]
    fn test_generate_pattern_instructions_some() {
        let include: Option<Vec<String>> = Some(vec!["test_include".to_owned()]);
        let exclude: Option<Vec<String>> = Some(vec!["test_exclude".to_owned()]);

        let result = generate_pattern_instructions(include, exclude);

        assert_eq!(result.len(), 2);
        for p in result.into_iter() {
            match p {
                PatternInstruction::Include(Pattern::Shell(val)) => {
                    assert_eq!(val, "test_include")
                }
                PatternInstruction::Exclude(Pattern::Shell(val)) => {
                    assert_eq!(val, "test_exclude")
                }
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn test_generate_pattern_instructions_none() {
        let include: Option<Vec<String>> = None;
        let exclude: Option<Vec<String>> = None;

        let result = generate_pattern_instructions(include, exclude);
        assert_eq!(result.len(), 0)
    }

    #[test]
    fn test_generate_pattern_instructions_mix() {
        let include1: Option<Vec<String>> = Some(vec!["test_include".to_owned()]);
        let exlude1: Option<Vec<String>> = None;
        let result1 = generate_pattern_instructions(include1, exlude1);

        assert_eq!(result1.len(), 1);
        for p in result1.into_iter() {
            match p {
                PatternInstruction::Include(Pattern::Shell(val)) => {
                    assert_eq!(val, "test_include");
                }
                _ => assert!(false),
            }
        }

        let include2: Option<Vec<String>> = None;
        let exlude2: Option<Vec<String>> = Some(vec!["test_exclude".to_owned()]);

        let result2 = generate_pattern_instructions(include2, exlude2);
        assert_eq!(result2.len(), 1);
        for p in result2.into_iter() {
            match p {
                PatternInstruction::Exclude(Pattern::Shell(val)) => {
                    assert_eq!(val, "test_exclude");
                }
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn test_create_archive_pass() {
        let repo_dir = util::get_temp_directory();
        let target_dir = util::get_temp_directory();
        let passphrase = "passphrase".to_owned();

        let init_args = init::InitArgs {
            repository: repo_dir.clone(),
            passphrase: passphrase.clone(),
        };
        let _ = init::initialise_repository(&init_args);

        let mut create_args = setup_create_args();
        create_args.repository = repo_dir.clone();
        create_args.paths = vec![target_dir.clone()];
        create_args.passphrase = passphrase;

        match create_archive(&create_args) {
            Ok(n) => {
                assert_eq!(n.repository.location, repo_dir);
                assert_eq!(n.archive.name, "archive");
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn test_create_archive_passphrase_fail() {
        let repo_dir = util::get_temp_directory();
        let target_dir = util::get_random_string(10);
        let passphrase = "passphrase".to_owned();

        let init_args = init::InitArgs {
            repository: repo_dir.clone(),
            passphrase: passphrase.clone(),
        };
        let _ = init::initialise_repository(&init_args);

        let mut create_args = setup_create_args();
        create_args.repository = repo_dir.clone();
        create_args.paths = vec![target_dir.clone()];
        create_args.passphrase = util::get_random_string(10);

        match create_archive(&create_args) {
            Ok(_) => assert!(false),
            Err(e) => match e {
                ArchiveError::ArchiveCreateError(create_error) => match create_error {
                    borgbackup::errors::CreateError::PassphraseWrong => assert!(true),
                    _ => assert!(false),
                },
                ArchiveError::EpochTimeError => assert!(false),
            },
        }
    }
}
