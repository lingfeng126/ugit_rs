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

pub fn hash_object<P: AsRef<std::path::Path>>(path: P) -> String{
    // save objects
    let bytes = std::fs::read(path).unwrap();
    data::hash_object(bytes, data::ObjectTypes::Blob)
}

pub fn cat_file(hash: &String){
    let bytes = data::get_object(hash, data::ObjectTypes::Blob);
    println!("{:?}", bytes)
}

pub fn write_tree(directory: &String) -> String{
    let paths = fs::read_dir(directory).unwrap();
    
    let mut temp: Vec<String> = vec![];
    for path in paths{
        let entry = path.unwrap().path();
        if entry.is_dir(){
            if !ignored_directory(&entry){
                let hash = write_tree(&entry.to_str().unwrap().to_owned());
                temp.push(format!("Tree {} {}", hash, entry.display()));
            }
        }
        else {
            let hash = hash_object(&entry);
            temp.push(format!("Blob {} {}", hash, entry.display()))
        }
    }
    let result = temp.join("\n");
    data::hash_object(result.into_bytes(), data::ObjectTypes::Tree)
}

pub fn ignored_directory(directory: &std::path::PathBuf) -> bool{
    directory.ends_with(".ugit") || directory.ends_with(".git") 
}

pub fn read_tree(hash: &String){
    let tree_raw = data::get_object(hash, data::ObjectTypes::Tree);
    let tree = std::string::String::from_utf8(tree_raw).unwrap();

    for line in tree.split("\n"){
        let mut iter = line.split(' ');
        let type_ = iter.next().unwrap();
        let hash = iter.next().unwrap().to_string();
        let filename = iter.next().unwrap();
        
        match type_{
            "Blob" => {
                let bytes = data::get_object(&hash, data::ObjectTypes::Blob);
                if std::path::Path::new(filename).exists(){
                    fs::remove_file(filename).unwrap();
                }
                fs::write(filename, bytes).unwrap();
            }
            "Tree" => {
                if std::path::Path::new(filename).exists(){
                    fs::remove_dir_all(filename).unwrap();
                }
                fs::create_dir(filename).unwrap();
                read_tree(&hash);
            }
            _ => panic!("Unknown file type, illegal object!")
        }
    }
}