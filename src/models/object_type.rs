use std::fmt::Formatter;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ObjectTypes{
    Blob = 0,
    Tree = 1,
    Commit = 2,
    Tag = 3,
}

impl From<u8> for ObjectTypes{
    fn from(value: u8) -> Self{
        match value{
            0 => Self::Blob,
            1 => Self::Tree,
            2 => Self::Commit,
            3 => Self::Tag,
            _ => panic!("Unknown type!")
        }
    }
}

impl std::fmt::Display for ObjectTypes{
    fn fmt(&self, f:&mut Formatter) -> std::fmt::Result{
        match self{
            Self::Blob => write!(f, "{}", "blob"),
            _ => panic!("Unknown type!")
        }
    }
}
