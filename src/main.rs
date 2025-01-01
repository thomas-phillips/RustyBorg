use std::process;

use clap::{Parser, Subcommand};

mod borg;
mod util;

#[derive(Parser, Debug)]
#[command(name = "RustyBorg")]
#[command(version = "1.0")]
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
    let args = Args::parse();
    println!("{:#?}\n", args);

    match args.cmd {
        Commands::Init(init_args) => {
            borg::init::initialise_repository(&init_args);
        }
        Commands::Create(create_args) => match borg::create::create_archive(&create_args) {
            Ok(n) => borg::create::display_create_info(n),
            Err(err) => borg::errors::parse_archive_error(err),
        },
        Commands::List(list_args) => match borg::list::list_contents(list_args) {
            Ok(()) => (),
            Err(err) => {
                eprintln!("{:?}", err);
                process::exit(1);
            }

        },
        Commands::Verify(verify_args) => {
            let test_con = util::verify_connection(verify_args);
            match test_con {
                Ok(()) => println!("Connection verified!"),
                Err(e) => util::exiterr_with_message(1, &format!("{}", e)),
            }
        }
        Commands::Schedule(schedule_args) => borg::schedule::schedule_borg(&schedule_args),
    }
}
