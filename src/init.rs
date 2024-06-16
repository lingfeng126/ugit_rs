use std::fs;
use std::path;

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
    fs::create_dir(".ugit").unwrap()
}