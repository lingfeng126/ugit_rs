use std::io::{Read, Write};

use sha256;

use crate::base::ObjectTypes;

pub fn hash_object(bytes: Vec<u8>, expected:ObjectTypes) -> String{
    // TODO use first 2 digits to divide the files into subdirectories
    let name = sha256::digest(&bytes);
    let mut file_name = ".ugit/objects/".to_owned();
    file_name.push_str(&name);
    let mut file = std::fs::File::create(file_name).unwrap();
    file.write(std::slice::from_ref(&(expected as u8))).unwrap();
    file.write(&bytes).unwrap();
    name
}

pub fn get_object(hash: &String, expected: ObjectTypes) -> Vec<u8>{
    // Read contents from a object
    let path = format!(".ugit/objects/{hash}");
    if let Ok(mut file) = std::fs::File::open(&path){
        let metadata = std::fs::metadata(&path).unwrap();
        let mut content = vec![0; metadata.len() as usize];

        file.read(&mut content).unwrap();

        // first digit is the object type
        let exp = expected as u8;
        if content[0] != exp{
            panic!("Object type is not as expected: {:?} {:?}", content[0], exp)
        }else{
            content[1..].to_vec()
        }
    }else{
        panic!("File not found! {}", path)
    }
}

// pub fn get_object_any(hash: &String) -> 

pub fn test_object_type(hash: &String, expected:ObjectTypes) -> Result<u8, String>{
    let path = format!(".ugit/objects/{hash}");
    if let Ok(mut file) = std::fs::File::open(&path){
        let metadata = std::fs::metadata(&path).unwrap();
        let mut content = vec![0; metadata.len() as usize];

        file.read(&mut content).unwrap();
        let exp = expected as u8;
        if content[0] != exp{
            Err(format!("Object type is not as expected: {:?} {:?}", content[0], exp))
        }else{
            Ok(exp)
        }
    }else{
        Err(format!("File not found! {}", path))
    }
}

pub fn set_ref(oid: &String, ref_: String){
    std::fs::write(format!(".ugit/refs/{}", ref_), oid).unwrap();
}

pub fn set_head(oid: &String){
    set_ref(oid, "HEAD".to_string())
}

pub fn get_ref(ref_: String) -> Option<String>{
    for file in [".ugit", ".ugit/refs", ".ugit/tags", ".ugit/heads"]{
        if let Ok(content) = std::fs::read_to_string(format!("{file}/{ref_}")){
            return Some(content)
        }
    }
    None
}

pub fn get_head() -> String{
    get_ref("HEAD".to_string()).unwrap_or("".to_string())
}

pub fn get_type(content: &String) -> ObjectTypes{
    ObjectTypes::from(content.bytes().nth(0).unwrap())
}