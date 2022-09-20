use std::io::stdin;
use walkdir::WalkDir;
fn main() {
    let mut dir = String::new();
    println!("Enter the dirctory name:");
    stdin().read_line(&mut dir).expect("err");
    let mut dir = dir.trim();
    let mut file_ext = String::new();
    println!("Enter the file extension:");
    stdin().read_line(&mut file_ext).expect("err");
    let file_ext = file_ext.trim();
    println!("File Extension chosen :{}", file_ext);
    let mut path = "/Users/home".to_string();
    path.push_str("/");
    path.push_str(dir);
    println!("{}", path);
    search(&path);
}

fn search(path: &str) {
    for file in WalkDir::new(path).into_iter() {
        let i = file.as_ref().unwrap();
        let path = i.path().to_str().unwrap();
        println!("Path:{}", path);
        if std::fs::metadata(path).unwrap().is_dir() {
            continue;
        } else {
            // search(&file.unwrap().path().to_str().unwrap())
            search(path)
        }
    }
}
