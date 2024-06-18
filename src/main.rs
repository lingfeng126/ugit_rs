use clap::Parser;

mod command;
mod cli;
mod data;

fn main() {
    let cmd = command::Cli::parse();
    match &cmd.command{
        Some(command::Commands::Init) => cli::init(),
        Some(command::Commands::HashObject{ path }) => {cli::hash_object(path);},
        Some(command::Commands::CatFile{ hash }) => cli::cat_file(hash),
        Some(command::Commands::WriteTree{ directory }) => {cli::write_tree(directory);},
        Some(command::Commands::ReadTree{ hash }) => {cli::read_tree(hash);},
        None => panic!("Unknown command")
    }
}
