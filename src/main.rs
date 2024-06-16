use clap::Parser;

mod command;
mod cli;
mod data;

fn main() {
    let cmd = command::Cli::parse();
    match &cmd.command{
        Some(command::Commands::Init) => cli::init(),
        Some(command::Commands::HashObject{ path }) => cli::hash_object(path),
        Some(command::Commands::CatFile{ hash }) => cli::cat_file(hash),
        None => panic!("Unknown command")
    }
}
