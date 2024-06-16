use clap::Parser;

mod command;
mod init;

fn main() {
    let cmd = command::Cli::parse();
    match &cmd.command{
        Some(command::Commands::Init) => init::init(),
        None => panic!("Unknown command")
    }
}
