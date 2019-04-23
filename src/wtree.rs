use std::fs;
use std::os::unix::fs::PermissionsExt;

use crate::envir::*;

struct Counter {
    visible_file_count: u32,
    file_count: u32,
    visible_dir_count: u32,
    dir_count: u32,
    pub tab: String,
    pub leaf: String,
    pub end_leaf: String,
    pub sub_dir_tab: String,
}
impl Counter {
    pub fn new() -> Counter {
        Counter {
            visible_file_count: 0,
            file_count: 0,
            visible_dir_count: 0,
            dir_count: 0,
            tab: "    ".to_string(),
            leaf: "├── ".to_string(),
            end_leaf: "└── ".to_string(),
            sub_dir_tab: "│   ".to_string(),
        }
    }

    pub fn increase_file(&mut self, is_visible: bool) {
        self.file_count += 1;
        if is_visible {
            self.visible_file_count += 1;
        }
    }

    pub fn increase_dir(&mut self, is_visible: bool) {
        self.dir_count += 1;
        if is_visible {
            self.visible_dir_count += 1;
        }
    }

    pub fn get_counter(&self) -> (u32, u32, u32, u32) {
        (
            self.visible_dir_count,
            self.dir_count,
            self.visible_file_count,
            self.file_count,
        )
    }
}

pub fn print_tree() -> std::io::Result<()> {
    let setting: Setting = parase_parameter()?;;
    let mut prefix = vec!["".to_string()];
    let mut counter = Counter::new();
    print_prefix(&prefix);
    print_subdir(
        &std::path::PathBuf::from(setting.root),
        &mut prefix,
        &mut counter,
    )?;
    println!(
        "\ntotal file: {}, printed file: {}, total directory: {}, printed directory: {}",
        counter.get_counter().0,
        counter.get_counter().1,
        counter.get_counter().2,
        counter.get_counter().3
    );

    Ok(())
}

fn print_prefix(prefix: &Vec<String>) {
    for item in prefix {
        print!("{}", item);
    }
}

fn print_subdir(
    root: &std::path::PathBuf,
    prefix: &mut Vec<String>,
    counter: &mut Counter,
) -> std::io::Result<()> {
    //println!("root : {}",root);
    for _file in fs::read_dir(root)? {
        let file = _file?;
        let path = file.path();
        let mut file_name = "";
        if let Some(os_str) = path.file_name() {
            if let Some(s) = os_str.to_str() {
                file_name = s;
            }
        }
        let metadata = path.metadata().expect("metadata call failed");
        //println!("{:b}",metadata.permissions().mode());
        print_prefix(&prefix);
        println!("{:?}", file_name);

        // is dir
        if path.is_dir() {
            counter.increase_dir(is_visible(file_name));
            prefix.push(counter.leaf.clone());
            print_subdir(&path, prefix, counter)?;
            prefix.pop();
        }
        // else is file
        else {
            counter.increase_file(is_visible(file_name));
        }
    }

    Ok(())
}

fn is_file_executable(metadata: fs::Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}

fn is_visible(name: &str) -> bool {
    if let Some(character) = name.get(0..0) {
        character == "."
    } else {
        false
    }
}
