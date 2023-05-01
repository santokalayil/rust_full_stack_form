use std::{env, fs};
use std::path::{PathBuf};

// const 
// pub const ASSETS_DIR:Path = CURRENT_DIR.join("assets");


pub fn create_required_directories() {

    let current_dir:PathBuf = env::current_dir().unwrap();
    let assets_dir:PathBuf = current_dir.join("assets");

    if let Err(e) = fs::metadata(&assets_dir) {
        if e.kind() == std::io::ErrorKind::NotFound {
            fs::create_dir(&assets_dir).unwrap();
            println!("Created directory {:?}", assets_dir);
        } else {
            panic!("Error checking directory: {:?}", e);
        }
    } else {
        println!("Directory {:?} already exists", assets_dir);
    }
}

