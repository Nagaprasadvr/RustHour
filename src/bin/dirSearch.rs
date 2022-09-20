use std::fs::File;
use std::io::stdin;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;
fn main() {
    let mut dir = String::new();
    println!("Enter the directory name:");
    stdin().read_line(&mut dir).expect("err");
    let mut dir = dir.trim();
    let mut file_ext = String::new();
    println!("Enter the file extension:");
    stdin().read_line(&mut file_ext).expect("err");
    let file_ext = file_ext.trim();
    println!("File Extension chosen :{}", file_ext);
    let mut path = "/Users/home/rusthour".to_string();
    path.push_str("/");
    path.push_str(dir);
    println!("{}", path);
    search(&path, file_ext);
}

fn search(path: &str, ext: &str) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    let mut ct: u8 = 0;
    for file in WalkDir::new(path).into_iter() {
        let i = file.as_ref().unwrap();
        let path = i.path().to_str().unwrap();

        if i.file_name().to_str().unwrap().ends_with(ext) {
            println!("Path:{}", path);
            let f = BufReader::new(File::open(path).expect("Err in opening file"));
            ct = 0;
            for i in f.lines().into_iter() {
                ct = ct + 1;
            }

            println!("file name :{:?}", &i.file_name());
            println!("No of lines : {}", ct);
            v.push(ct);
        }
        // else if i.metadata().unwrap().is_dir() {
        //     return search(path, ext);
        // }
    }
    return v;
}

#[cfg(test)]
mod FileSearchtest {
    use crate::search;

    #[test]
    fn test1() {
        assert_eq!(
            search("/Users/home/rusthour/Rusthour/f2", ".py"),
            vec![0, 1]
        );
    }
    #[test]
    fn test2() {
        assert_eq!(search("/Users/home/rusthour/Rusthour/f2", ".rs"), vec![3]);
    }
    #[test]
    fn test3() {
        assert_eq!(search("/Users/home/rusthour/Rusthour/f2", ".rs"), vec![3]);
    }
    #[test]
    fn test4() {
        assert_eq!(search("/Users/home/rusthour/Rusthour", ".rs"), vec![3, 3]);
    }
    #[test]
    fn test5() {
        assert_eq!(
            search("/Users/home/rusthour/Rusthour", ".py"),
            vec![0, 1, 1]
        );
    }
}
