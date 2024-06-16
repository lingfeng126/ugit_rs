use std::io::{Read, Write};

use sha256;

pub enum ObjectTypes{
    Blob = 0,
}

pub fn hash_object(bytes: Vec<u8>, expected:ObjectTypes){
    let name = sha256::digest(&bytes);
    let mut file_name = ".ugit/objects/".to_owned();
    file_name.push_str(&name);
    println!("{}", file_name);
    let mut file = std::fs::File::create(file_name).unwrap();
    file.write(std::slice::from_ref(&(expected as u8))).unwrap();
    file.write(&bytes).unwrap();
}

pub fn get_object(hash: &String, expected: ObjectTypes) -> String{
    let path = format!(".ugit/objects/{hash}");
    let mut file = std::fs::File::open(&path).unwrap();
    let metadata = std::fs::metadata(&path).unwrap();
    let mut content = vec![0; metadata.len() as usize];

    file.read(&mut content).unwrap();

    if content[0] != (expected as u8){
        panic!("Object type is not as expected")
    }else{
        std::string::String::from_utf8(content[1..].to_vec()).unwrap()
    }
    
}