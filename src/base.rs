use std::fmt::Formatter;

use crate::data;

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


pub struct Blob{
    content: Vec<u8>
}

impl std::fmt::Display for Blob{
    fn fmt(&self, f:&mut Formatter) -> std::fmt::Result{
        write!(f, "{}", String::from_utf8_lossy(&self.content))
    }
}

impl Blob{
    pub fn new(content: Vec<u8>) -> Self{
        Blob{
            content
        }
    }

    pub fn from_oid(oid: &String) -> Self{
        let content = data::get_object(oid, ObjectTypes::Blob);
        Self::new(content)
    }
}

pub struct TreeEntry{
    mode: ObjectTypes,
    name: String,
    oid: String
}

impl TreeEntry{
    pub fn new(mode: ObjectTypes, name: String, oid: String) -> Self{
        TreeEntry{
            mode, name, oid
        }
    }
}

pub struct Tree{
    entries: Vec<TreeEntry>,
    oid: String
}

impl Tree{
    fn parse(content: &str) -> Vec<TreeEntry>{
        let mut entries: Vec<TreeEntry> = Vec::new();
        let mut iter: std::str::Split<'_, char> = content.split('\n').into_iter();
        loop{
            let line = iter.next();
            if line.is_none(){
                break
            }
            let line = line.unwrap();
            if line.len() < 1{
                break
            }
            let mut loc = line.splitn(2, " ");
            let mode: u8 = loc.next().unwrap().as_bytes()[0];
            let name = loc.next().unwrap();
            let oid = loc.next().unwrap();
            entries.push(TreeEntry::new(ObjectTypes::from(mode), name.into(), oid.into()));
        }
        entries
    }

    fn dump(entries: &Vec<TreeEntry>) -> String{
        let mut result = String::new();
        for entry in entries{
            result.push_str(&format!("{} {} {}\n", entry.mode as u8, entry.name, entry.oid));
        }
        result
    }

    pub fn from_oid(oid: &String) -> Self{
        let tree = data::get_object(oid, ObjectTypes::Tree);
        let row: &str = &String::from_utf8(tree).unwrap();
        let entries = Self::parse(row);

        Self{
            entries,
            oid: oid.to_string()
        }
    }

    pub fn from_path(path: &String) -> Self{
        let mut entries = Vec::new();
        for entry in std::fs::read_dir(path).unwrap(){
            let entry = entry.unwrap();
            let path = entry.path();
            let name = entry.file_name().into_string().unwrap();
            let mode = if path.is_dir(){
                ObjectTypes::Tree
            }else{
                ObjectTypes::Blob
            };
            let oid = if mode == ObjectTypes::Blob{
                data::hash_object(std::fs::read(path).unwrap(), ObjectTypes::Blob)
            }else{
                Self::from_path(&path.into_os_string().into_string().unwrap()).oid
            };
            entries.push(TreeEntry::new(mode, name, oid));
        }
        let oid = Self::dump(&entries);

        Self{
            entries,
            oid: oid
        }
    }
}

pub struct Ref{
    oid: String
}

pub struct Commit{
    tree: String,
    parent: Option<String>,
    message: String
}


impl std::fmt::Display for Commit{
    fn fmt(&self, f:&mut Formatter) -> std::fmt::Result{
        write!(f, "tree: {}\nparent: {}\nmessage:{} ", self.tree, self.parent.as_ref().unwrap_or(&"".to_string()), self.message)
    }
}

impl Commit{
    pub fn new(tree: String, parent:Option<String>, message:String) -> Self{
        Commit{
            tree, parent, message
        }
    }

    pub fn from_oid(oid: &String) -> Self{
        let commit = data::get_object(oid, ObjectTypes::Commit);
        let content = String::from_utf8(commit).unwrap();
        let mut content_iter = content.split("\n").into_iter();
        let mut tree = "";
        let mut parent = None;
        for line in content_iter.by_ref(){
            if line.len() < 1 {
                break
            }
            let mut loc = line.splitn(2, " ");
            let key = loc.next().unwrap();
            let value = loc.next().unwrap();
            if key == "tree"{
                tree = value;
            }else if  key == "parent" {
                if value.trim().len() > 1{
                    parent = Some(value.to_string());
                }
            }
        }
        let message = content_iter.fold(String::new(), |a, b| a + b);
        Self::new(tree.to_string(), parent, message)
    }

    pub fn get_parent(&self) -> Option<Self>{
        match &self.parent{
            Some(oid) => Some(Self::from_oid(oid)),
            None => None
        }
    }

    pub fn get_tree(&self) -> &String{
        &self.tree
    }
}