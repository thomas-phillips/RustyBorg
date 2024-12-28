use borgbackup::common::{CommonOptions, CreateOptions, Pattern, PatternInstruction};
use borgbackup::output::create::Create;
use borgbackup::sync::create;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn new_create_options(
    repository: String,
    passphrase: String,
    archive: String,
    paths: Vec<String>,
    pattern_instructions: Vec<PatternInstruction>,
) -> CreateOptions {
    let mut create_options = CreateOptions::new(repository, archive, paths, pattern_instructions);
    create_options.passphrase = Some(passphrase);
    create_options
}

fn print_command(commands: Vec<String>) {
    println!("\nCommand used:");

    for command in commands.iter() {
        print!("{} ", command);
    }
    println!("");
}

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

pub fn create_archive(
    repository_path: String,
    passphrase: String,
    archive: Option<String>,
    paths: Vec<String>,
    include_patterns: Option<Vec<String>>,
    exclude_patterns: Option<Vec<String>>,
) {
    let archive_name: String =
        archive.unwrap_or(match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => Duration::as_secs(&duration).to_string(),
            Err(e) => panic!("SystemTimeError difference: {:?}", e.duration()),
        });

    let pattern_instructions = generate_pattern_instructions(include_patterns, exclude_patterns);

    let create_options = new_create_options(
        repository_path,
        passphrase,
        archive_name,
        paths,
        pattern_instructions,
    );
    let common_options = CommonOptions::default();

    let create_result: Create = match create(&create_options, &common_options) {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            panic!("Error creating archive: {}", e);
        }
    };

    println!(
        "Successfully created archive at {}::{}",
        create_result.repository.location, create_result.archive.name,
    );
    println!("Started at: {}", create_result.archive.start);
    println!("Ended at: {}", create_result.archive.end);
    println!("Took: {}", create_result.archive.duration);
    // println!("Commands used: {:?}", create_result.archive.command_line);
    print_command(create_result.archive.command_line);
}
