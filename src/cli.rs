use std::fs;
use std::path;
use crate::data;

pub fn init(){
    let dirpath = path::Path::new(".ugit");

    if dirpath.exists() {
        let metadata = fs::metadata(dirpath).unwrap();
        if !metadata.is_dir(){
            panic!("Failed to create cache directory, exiting...");
        }
        panic!("Current directory has been initialized")
    }
    println!("Creating .ugit directory...");
    fs::create_dir(".ugit").unwrap();
    fs::create_dir_all(".ugit/objects").unwrap();
}

pub fn hash_object<P: AsRef<std::path::Path>>(path: P){
    // save objects
    let bytes = std::fs::read(path).unwrap();
    data::hash_object(bytes, data::ObjectTypes::Blob)
}

pub fn cat_file(hash: &String){
    let bytes = data::get_object(hash, data::ObjectTypes::Blob);
    println!("{}", bytes)
}