use std::collections::VecDeque;
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
    let setting: Setting = parase_parameter()?;
    let mut counter = Counter::new();
    let mut prefix = VecDeque::new();
    prefix.push_back(counter.leaf.clone());
    println!("{}",setting.root);
    print_subdir(
        &std::path::PathBuf::from(&setting.root),
        &mut prefix,
        &mut counter,
        & setting,
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

fn print_prefix(prefix: &VecDeque<String>) {
    for item in prefix {
        print!("{}", item);
    }
}

fn print_subdir(
    root: &std::path::PathBuf,
    prefix: &mut VecDeque<String>,
    counter: &mut Counter,
    setting: &Setting,
) -> std::io::Result<()> {
    let mut path_list : Vec<std::path::PathBuf> = fs::read_dir(root)?.map(|item|->std::path::PathBuf {
        match item{
            Ok(sth) => sth.path(),
            _ => std::path::PathBuf::new(),
        }
    }).collect();
    path_list.sort();

    let file_num = fs::read_dir(root)?.count();
    let mut iter_cnt = 0;
    for path in path_list {
        let mut file_name = "";
        iter_cnt += 1;
        if let Some(os_str) = path.file_name() {
            if let Some(s) = os_str.to_str() {
                file_name = s;
            }
        }

        // identify the last item
        if iter_cnt == file_num{
            prefix.pop_back();
            prefix.push_back(counter.end_leaf.clone());
        }
        else if iter_cnt == 1{
            prefix.pop_back();
            prefix.push_back(counter.leaf.clone());
        }

        // judge for flag `-a`
        if setting.is_all == false && !is_visible(file_name){
            continue;
        }

        let metadata = path.metadata().expect("metadata call failed");
        print_prefix(&prefix);
        println!("{}", file_name);
        increase_counter(&path,&file_name,counter);      

        // is dir
        if path.is_dir() {
            // insert prefix
            prefix.pop_back();
            if iter_cnt == file_num{
                prefix.push_back(counter.tab.clone());
                prefix.push_back(counter.end_leaf.clone());
            }
            else{
                prefix.push_back(counter.sub_dir_tab.clone());
                prefix.push_back(counter.leaf.clone());
            }

            // recursive
            print_subdir(&path, prefix, counter,setting)?;
            
            // recover prefix
            prefix.pop_back();
            prefix.pop_back();
            if iter_cnt +1  == file_num{
                prefix.push_back(counter.end_leaf.clone());
            }
            else{
                prefix.push_back(counter.leaf.clone());
            }
        }
    }

    Ok(())
}

fn increase_counter(path: &std::path::PathBuf, file_name : &str, counter: &mut Counter){
    if path.is_dir() {
        counter.increase_dir(is_visible(file_name));
    }
    else{
        counter.increase_file(is_visible(file_name));
    }
}

fn is_file_executable(metadata: fs::Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}

fn is_visible(name: &str) -> bool {
    if let Some(character) = name.get(0..1) {
        !(character == ".")
    } else {
        false
    }
}


#[test]
fn test_is_visible(){
    assert_eq!(is_visible(".git"),false);
    assert_eq!(is_visible("asdfasd"),true);
}