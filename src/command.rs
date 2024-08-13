
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli{
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands{
    Init,
    HashObject {
        path : String
    },
    CatFile {
        ref_ : String
    },
    WriteTree {
        directory: String
    },
    ReadTree {
        hash: String
    },
    Commit {
        message: String
    },
    Log {
        hash: Option<String>
    },
    Checkout {
        ref_: String
    },
    Tag {
        name: String,
        hash: String
    }
}
