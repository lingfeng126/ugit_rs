use std::fmt::Formatter;

use crate::data;

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