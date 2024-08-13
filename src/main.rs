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
        Some(command::Commands::CatFile{ ref_ }) => {
            let ref_ = data::get_ref(ref_.clone()).unwrap_or(ref_.to_string());
            cli::cat_file(&ref_);
        },
        Some(command::Commands::WriteTree{ directory }) => {cli::write_tree(directory);},
        Some(command::Commands::ReadTree{ hash }) => {
            let hash = data::get_ref(hash.clone()).unwrap_or(hash.to_string());
            cli::read_tree(&hash);
        },
        Some(command::Commands::Commit{ message }) => {cli::commit(message);},
        Some(command::Commands::Log{ hash }) => {cli::log(hash);}, 
        Some(command::Commands::Checkout{ ref_ }) => {
            let ref_ = data::get_ref(ref_.clone()).unwrap_or(ref_.to_string());
            cli::check_out(&ref_);
        }, 
        Some(command::Commands::Tag{ name, hash }) => {
            let hash = data::get_ref(hash.clone()).unwrap_or(hash.to_string());
            cli::tag(name, &hash);
        }, 
        None => panic!("Unknown command")
    }
}
