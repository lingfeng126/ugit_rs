use std::io::{Read, Write};

use sha256;

use crate::base::ObjectTypes;

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

pub fn set_head(oid: &String){
    std::fs::write(".ugit/HEAD", oid).unwrap();
}

pub fn get_head() -> String{
    std::fs::read_to_string(".ugit/HEAD").unwrap().to_string()
}