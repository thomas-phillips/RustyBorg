use clap::{Parser, Subcommand};

mod borg;
mod util;

#[derive(Parser, Debug)]
#[command(name = "RustyBorg")]
#[command(version = "0.1.0")]
#[command(author = "Thomas Phillips")]
#[command(about = "")]
struct Args {
    // #[arg(short, long, default_value_t = false)]
    // daemonize: bool,
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Init(borg::init::InitArgs),
    Create(borg::create::CreateArgs),
    List(borg::list::ListArgs),
    Verify(util::VerifyArgs),
    Schedule(borg::schedule::ScheduleArgs),
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    match args.cmd {
        Commands::Init(init_args) => match borg::init::initialise_repository(&init_args) {
            Ok(_) => util::log_print("Repository successfully created", util::LogLevel::Info),
            Err(e) => util::log_print(&format!("Operation failed: {}", e), util::LogLevel::Error),
        },
        Commands::Create(create_args) => match borg::create::create_archive(&create_args) {
            Ok(n) => borg::create::display_create_info(n),
            Err(err) => borg::errors::parse_archive_error(err),
        },
        Commands::List(list_args) => match borg::list::list_contents(list_args) {
            Ok(()) => (),
            Err(err) => {
                util::log_print(&format!("{:?}", err), util::LogLevel::Error);
            }
        },
        Commands::Verify(verify_args) => {
            let test_con = util::verify_connection(verify_args);
            match test_con {
                Ok(_) => println!("Connection verified!"),
                Err(e) => util::log_print(&format!("{}", e), util::LogLevel::Error),
            }
        }
        Commands::Schedule(schedule_args) => borg::schedule::schedule_borg(&schedule_args),
    }
}
