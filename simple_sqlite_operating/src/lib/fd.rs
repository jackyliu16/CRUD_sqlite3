///! to finish the Internal logic of path and show the file under the path

use std::fs::{self, DirEntry};
use std::path::{PathBuf};
// use path_absolutize::*;
// extern crate glob;
// use glob::glob;

#[allow(dead_code)]
pub fn get_file_under_path(path_root: &PathBuf) -> Vec<DirEntry> {

    let mut list: Vec<DirEntry> = Vec::new();

    let paths = fs::read_dir(path_root.as_path()).unwrap();

    for path in paths {
        let path = path.unwrap();
        // list.push(path.unwrap());
        // if is file then check if it's with a .db ending
        // println!("path: {:?}", path);
        if is_file(&path) {
            if get_last_num_char(get_file_name_from_direntry(&path), 3).as_str() == ".db" {
                list.push(path);
            }
        } else {
            list.push(path);
        }
    } 

    list
}

pub fn get_last_num_char(inp: String, num: usize) -> String {
    let mut tmp = Vec::new();
    for i in 0..num {
        tmp.push(inp.chars().nth_back(num - i - 1).unwrap());
    }
    tmp.iter().collect()
}

pub fn get_file_name_from_direntry(dir: &DirEntry) -> String {
    dir.file_name().as_os_str().to_str().unwrap().to_string()
}

pub fn is_file(dir: &DirEntry) -> bool {
    dir.file_type().unwrap().is_file()
}
#[allow(dead_code)]
pub fn print_pwd(path: &PathBuf) {
    println!("{:?}", path);
}

#[allow(dead_code)]
pub fn move_to_parent(root: &mut PathBuf) {
    *root = PathBuf::from(root.parent().unwrap());
}

#[allow(dead_code)]
pub fn move_to_direntry(root: &mut PathBuf, dst: String)  {
    if let Some(dst_direntry) = match_path(&root, dst) {
        *root = dst_direntry.path();
    } else {
        println!("wrong input");
    }
}

#[allow(dead_code, unused_variables)]
pub fn match_path(root: &PathBuf, str: String) -> Option<DirEntry> {
    let file_list = get_file_under_path(&root);

    for file in file_list {
        if get_file_name_from_direntry(&file).as_str() == str.as_str() {
            return Some(file);
        }
    }

    None
}

#[cfg(test)]
pub mod test {
    use crate::lib::fd::get_file_name_from_direntry;

    use super::{get_file_under_path, move_to_parent, print_pwd, move_to_direntry, match_path};
    use std::{env::current_dir, path::PathBuf};

    #[test]
    pub fn test_could_find_location() {
        let cwd = current_dir().unwrap();
        let list = get_file_under_path(&cwd);
        // for item in &list {
        //     println!("{:?}", *item);
        // }
        assert_eq!(
            list.len(),
            5
        )
    }

    #[test]
    pub fn move_test_parent() {
        let mut cwd = current_dir().unwrap();
        print_pwd(&cwd);
        move_to_parent(&mut cwd);
        print_pwd(&cwd);

        let list = get_file_under_path(&cwd);
        for i in list {
            println!("i: {:?}", get_file_name_from_direntry(&i));
        }
    }

    #[test]
    pub fn move_test_move_left_and_right() {
        let mut cwd = current_dir().unwrap();
        print_pwd(&cwd);
        print_everything(&cwd);

        println!("start testing");
        move_to_parent(&mut cwd);
        print_everything(&cwd);
        move_to_direntry(&mut cwd, String::from("simple_sqlite_operating"));
        print_everything(&cwd);
        
    }

    pub fn print_everything(path: &PathBuf) {
        println!("=========");
        let file = get_file_under_path(&path);
        for i in file {
            println!("{:?}", get_file_name_from_direntry(&i));
        }
    }
}