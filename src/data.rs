use std::{fmt::Formatter, io::{Read, Write}};

use sha256;

pub enum ObjectTypes{
    Blob = 0,
    Tree = 1
}

impl std::fmt::Display for ObjectTypes{
    fn fmt(&self, f:&mut Formatter) -> std::fmt::Result{
        match self{
            Self::Blob => write!(f, "{}", "blob"),
            _ => panic!("Unknown type!")
        }
    }
}


pub fn hash_object(bytes: Vec<u8>, expected:ObjectTypes) -> String{
    let name = sha256::digest(&bytes);
    let mut file_name = ".ugit/objects/".to_owned();
    file_name.push_str(&name);
    let mut file = std::fs::File::create(file_name).unwrap();
    file.write(std::slice::from_ref(&(expected as u8))).unwrap();
    file.write(&bytes).unwrap();
    name
}

pub fn get_object(hash: &String, expected: ObjectTypes) -> Vec<u8>{
    let path = format!(".ugit/objects/{hash}");
    let mut file = std::fs::File::open(&path).unwrap();
    let metadata = std::fs::metadata(&path).unwrap();
    let mut content = vec![0; metadata.len() as usize];

    file.read(&mut content).unwrap();

    if content[0] != (expected as u8){
        panic!("Object type is not as expected")
    }else{
        // std::string::String::from_utf8().unwrap()
        content[1..].to_vec()
    }
    
}