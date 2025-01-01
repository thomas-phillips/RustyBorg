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
    println!("\nCommand used:");

    for command in commands.iter() {
        print!("{} ", command);
    }
    println!("");
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
    println!(
        "Successfully created archive at {}::{}",
        create_result.repository.location, create_result.archive.name,
    );
    println!("Started at: {}", create_result.archive.start);
    println!("Ended at: {}", create_result.archive.end);
    println!("Took: {}", create_result.archive.duration);
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
    let archive_name: String =
        create_args
            .archive()
            .unwrap_or(match get_epoch_name() {
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
