use clap::Parser;

mod command;
mod cli;
mod data;
mod base;

fn main() {
    let cmd = command::Cli::parse();
    match &cmd.command{
        Some(command::Commands::Init) => cli::init(),
        Some(command::Commands::HashObject{ path }) => {cli::hash_object(path);},
        Some(command::Commands::CatFile{ hash }) => cli::cat_file(hash),
        Some(command::Commands::WriteTree{ directory }) => {cli::write_tree(directory);},
        Some(command::Commands::ReadTree{ hash }) => {cli::read_tree(hash);},
        Some(command::Commands::Commit{ message }) => {cli::commit(message);},
        Some(command::Commands::Log{ hash }) => {cli::log(hash);}, 
        None => panic!("Unknown command")
    }
}
