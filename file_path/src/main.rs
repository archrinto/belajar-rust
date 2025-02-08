// File Path
// String

use std::path::Path;

// File System

use std::fs;

fn main() {
    // check path exists
    if Path::new(&path).exists() {
        // path exists
    }

    if Path::new(&path).is_file() {
        // path is file
    }

    if Path::new(&path).is_dir() {
        // path is dirctory
    }

    if Path::new(&path).is_absolute() {
        // path is an absolute path
    }

    if Path::new(&path).is_relative() {
        // path is a relative path
    }

    // membuat directory
    fs::create_dir("path")

    match fs::create_dir("./files") {
        Err(err) => {
            println!("error on creating directory! {}", err);
        },
        _ => {
            println!("directory created");
        },
    }

    // menulis content
    let path = Path::new("./files").join("target.txt");
    let content = "hello rust!";
    let res = fs::write(&path, &content);

    match res {
        Err(err) => {
            println!("error on writing file {}! {}", path.to_str().unwrap_or_default(), err);
        },
        _ => {
            println!("file created");
        },
    }

    // membaca content dalam bentuk string
    let path = Path::new("./files").join("target.txt");
    let res = fs::read_to_string(&path);

    match res {
        Err(err) => {
            println!("error on reading file {}! {}", path.to_str().unwrap_or_default(), err);
        },
        Ok(content) => {
            println!("file {:?} content is: {:?}", path, content);
        },
    }

    // membaca content dalam bentuk vector
    let path = Path::new("./files").join("target.txt");
    let res = fs::read(&path);
    
    if res.is_err() {
        println!("error on reading file");
        return;
    }
    
    let content = res.unwrap_or_default();
    match std::str::from_utf8(&content) {
        Err(err) => {
            println!("error on reading file! Invalid UTF-8 sequence. {}", err);
        },
        Ok(content) => {
            println!("file {:?} content is: {:?}", path, content);
        },
    };

    // menghapus file
    let path = Path::new("./files").join("target.txt");
    let res = fs::remove_file(&path);
    
    match res {
        Err(err) => {
            println!("error on deleting file {}! {}", path.to_str().unwrap_or_default(), err);
        },
        _ => {
            println!("file deleted");
        },
    }

    // menghapus dir
    let path = Path::new("./files");
    let res = fs::remove_dir(&path);

    match res {
        Err(err) => {
            println!("error on deleting directory {}! {}", path.to_str().unwrap_or_default(), err);
        },
        _ => {
            println!("directory deleted");
        },
    }

    // list item dalam directory
    let path = Path::new("D:\\Labs\\Adam Studio\\Ebook\\dasarpemrogramanrust\\file_path_directory_1");
    let paths = fs::read_dir(&path).unwrap();
    
    for path in paths {
        let item = &path.unwrap();
        println!("file name: {:?}, file path: {:?}", item.file_name(), item.path().display())
    }

    println!("Hello, world!");
}
