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
    Verify(util::sshverify::VerifyArgs),
    Schedule(util::schedule::ScheduleArgs),
}

fn main() {
    let args = Args::parse();
    println!("{:#?}\n", args);

    match args.cmd {
        Commands::Init(init_args) => {
            borg::init::initialise_repository(&init_args);
        }
        Commands::Create(create_args) => {
            borg::create::create_archive(create_args);
        }
        Commands::List(list_args) => borg::list::list_contents(&list_args).unwrap(),
        Commands::Verify(verify_args) => {
            let test_con = util::sshverify::verify_connection(&verify_args);
            match test_con {
                Ok(()) => println!("Connection verified!"),
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }
        }
        Commands::Schedule(schedule_args) => {
            util::schedule::schedule_borg(&schedule_args)
        }
    }
}
