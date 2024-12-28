use borgbackup::common::{EncryptionMode, InitOptions};
use clap::{Parser, Subcommand};

mod borg;
mod sshverify;

#[derive(Parser, Debug)]
#[command(name = "RustyBorg")]
#[command(version = "1.0")]
#[command(author = "Thomas Phillips")]
#[command(about = "")]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Init {
        repository_path: String,
        passphrase: String,
    },
    Create {
        repository_path: String,
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
    },
    List {
        repository: String,
        passphrase: String,
    },
}

fn main() {
    let args = Args::parse();
    println!("{:#?}\n", args);

    match args.cmd {
        Commands::Init {
            repository_path,
            passphrase,
        } => {
            let init_options = InitOptions {
                repository: repository_path,
                encryption_mode: EncryptionMode::KeyfileBlake2(passphrase),
                append_only: false,
                make_parent_dirs: false,
                storage_quota: None,
            };
            borg::init::initialise_repository(&init_options);
        }
        Commands::Create {
            repository_path,
            archive,
            passphrase,
            paths,
            include_patterns,
            exclude_patterns,
        } => borg::create::create_archive(
            repository_path,
            passphrase,
            archive,
            paths,
            include_patterns,
            exclude_patterns,
        ),
        Commands::List {
            repository,
            passphrase,
        } => borg::list::list_contents(repository, passphrase),
    }
}
