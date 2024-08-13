use std::fs;
use std::path;
use crate::base::Commit;
use crate::base::ObjectTypes;
use crate::data;
use crate::base;

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
    fs::create_dir_all(".ugit/refs").unwrap();
}

pub fn hash_object<P: AsRef<std::path::Path>>(path: P) -> String{
    // save objects
    let bytes = std::fs::read(path).unwrap();
    data::hash_object(bytes, base::ObjectTypes::Blob)
}

pub fn cat_file(hash: &String){
    let bytes = data::get_object(hash, base::ObjectTypes::Blob);
    println!("{:?}\n", bytes)
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
    data::hash_object(result.into_bytes(), base::ObjectTypes::Tree)
}

fn ignored_directory(directory: &std::path::PathBuf) -> bool{
    directory.ends_with(".ugit") || directory.ends_with(".git") 
}

pub fn read_tree(hash: &String){
    let tree_raw = data::get_object(hash, base::ObjectTypes::Tree);
    let tree = std::string::String::from_utf8(tree_raw).unwrap();

    // delete existing files before checkout

    for line in tree.split("\n"){
        let mut iter = line.split(' ');
        let type_ = iter.next().unwrap();
        let hash = iter.next().unwrap().to_string();
        let filename = iter.next().unwrap();
        
        match type_{
            "Blob" => {
                let bytes = data::get_object(&hash, base::ObjectTypes::Blob);
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

pub fn commit(message: &String){
    let mut content = String::new();
    let parent = data::get_head();
    let hash = write_tree(&".".to_owned());
    content.push_str(&format!("tree {}\n", hash));
    if !parent.is_empty(){
        content.push_str(&format!("parent {}\n", parent));
    }
    content.push_str("\n");
    content.push_str(message);
    let commit_id = data::hash_object(content.into_bytes(), base::ObjectTypes::Commit);
    data::set_head(&commit_id);
    println!("{}\n", Commit::from_oid(&commit_id))
}



pub fn log(oid: String) {
    println!("{}\n", &oid);
    let mut commit_obj = Commit::from_oid(&oid);
    loop{
        println!("{}\n", commit_obj);
        match commit_obj.get_parent(){
            Some(commit) => {
                commit_obj = commit;
            },
            None => break
        }
    }
}

pub fn check_out(ref_: &String){
    let commit_id = data::get_ref(ref_.clone()).unwrap_or(ref_.clone());
    println!("Checking out {}", commit_id);
    let commit = Commit::from_oid(&commit_id);
    read_tree(commit.get_tree());
    data::set_head(commit.get_tree());
}



pub fn tag(name: &String, hash: &String){
    if let Ok(_) = data::test_object_type(hash, ObjectTypes::Commit){
        data::set_ref(hash, name.clone())
    }else{
        println!("{} is not a commit!", hash)
    }
}