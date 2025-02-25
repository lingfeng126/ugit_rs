use std::fmt::Formatter;

use crate::data;

use super::object_type::ObjectTypes;

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

impl std::fmt::Display for Tree{
    fn fmt(&self, f:&mut Formatter) -> std::fmt::Result{
        let content = Self::dump(&self.entries);
        write!(f, "{}", content)
    }
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