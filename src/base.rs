use std::fmt::Formatter;

use crate::cli;

pub enum ObjectTypes{
    Blob = 0,
    Tree = 1,
    Commit = 2
}

impl std::fmt::Display for ObjectTypes{
    fn fmt(&self, f:&mut Formatter) -> std::fmt::Result{
        match self{
            Self::Blob => write!(f, "{}", "blob"),
            _ => panic!("Unknown type!")
        }
    }
}

pub struct Commit{
    id: String,
    parent: Option<String>,
    message: String
}

impl std::fmt::Display for Commit{
    fn fmt(&self, f:&mut Formatter) -> std::fmt::Result{
        write!(f, "hash: {}\nparent: {}\nmessage:{} ", self.id, self.parent.as_ref().unwrap_or(&"".to_string()), self.message)
    }
}

impl Commit{
    pub fn new(id: String, parent:Option<String>, message:String) -> Self{
        Commit{
            id, parent, message
        }
    }

    pub fn get_parent(&self) -> Option<Self>{
        match &self.parent{
            Some(oid) => Some(cli::get_commit(oid)),
            None => None
        }
    }
}