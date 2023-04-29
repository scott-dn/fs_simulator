mod fs;

use std::{
    fs::File,
    io::{self, BufRead, Write},
};

use fs::DirManager;

fn main() {
    let mut dir_manager = DirManager::new();

    let mut file = File::create("output.txt").unwrap();

    let stdin = io::stdin();

    for line in stdin.lock().lines().flatten() {
        file.write_all(format!("Command: {line}\n").as_bytes()).unwrap();
        if line.eq("dir") {
            process_dir(&dir_manager, &mut file);
        } else if line.starts_with("up") {
            process_up(&mut dir_manager, &mut file);
        } else if line.starts_with("mkdir ") {
            let arg = &line[8..];
            process_mkdir(arg, &mut dir_manager, &mut file);
        } else if line.starts_with("cd ") {
            let arg = &line[8..];
            process_cd(arg, &mut dir_manager, &mut file);
        } else if line.starts_with("mv ") {
            let src = &line[8..16].trim();
            let des = &line[16..].trim();
            process_mv(src, des, &mut dir_manager, &mut file);
        } else if line.starts_with("tree") {
            process_tree(&mut dir_manager, &mut file);
        }
    }
}

fn process_dir(dir_manager: &DirManager, file: &mut File) {
    let (dir, sub_dir) = dir_manager.handle_dir();
    file.write_all(format!("Directory of {}:\n", dir).as_bytes()).unwrap();
    if !sub_dir.is_empty() {
        for (i, dir) in sub_dir.iter().enumerate() {
            if i > 0 && i % 10 == 0 {
                file.write_all(b"\n").unwrap();
            }
            // end of line
            if i > 0 && (i % 9 == 0) || i == sub_dir.len() - 1 {
                file.write_all(format!("{dir}").as_bytes()).unwrap();
            } else {
                file.write_all(format!("{dir: <8}").as_bytes()).unwrap();
            }
        }
        file.write_all(b"\n").unwrap();
    } else {
        file.write_all(b"No subdirectories\n").unwrap();
    }
}

fn process_up(dir_manager: &mut DirManager, file: &mut File) {
    if let Err(e) = dir_manager.handle_up() {
        file.write_all(format!("{e}\n").as_bytes()).unwrap();
    }
}

fn process_mkdir(arg: &str, dir_manager: &mut DirManager, file: &mut File) {
    if let Err(e) = dir_manager.handle_mkdir(arg.to_string()) {
        file.write_all(format!("{e}\n").as_bytes()).unwrap();
    }
}

fn process_cd(arg: &str, dir_manager: &mut DirManager, file: &mut File) {
    if let Err(e) = dir_manager.handle_cd(arg.to_string()) {
        file.write_all(format!("{e}\n").as_bytes()).unwrap();
    }
}

fn process_mv(src: &str, des: &str, dir_manager: &mut DirManager, file: &mut File) {
    if let Err(e) = dir_manager.handle_mv(src.to_string(), des.to_string()) {
        file.write_all(format!("{e}\n").as_bytes()).unwrap();
    }
}

fn process_tree(dir_manager: &mut DirManager, file: &mut File) {
    let (dir, node) = dir_manager.handle_tree();
    file.write_all(format!("Tree of {}:\n", dir).as_bytes()).unwrap();
    file.write_all(b".\n").unwrap();
    file.write_all(format!("{node}").as_bytes()).unwrap();
}
