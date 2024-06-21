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

pub fn set_ref(oid: &String, ref_: String){
    std::fs::write(format!(".ugit/refs/{}", ref_), oid).unwrap();
}

pub fn set_head(oid: &String){
    set_ref(oid, "HEAD".to_string())
}

pub fn get_ref(ref_: String) -> Option<String>{
    if let Ok(content) = std::fs::read_to_string(format!(".ugit/refs/{}", ref_)){
        return Some(content)
    }
    None
}

pub fn get_head() -> String{
    get_ref("HEAD".to_string()).unwrap_or("".to_string())
}

