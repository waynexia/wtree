use crate::envir::Setting;
use crate::print::*;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs;
use std::fs::Metadata;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

enum PrefixMode {
    file_tree,
    none,
}

enum TreePrefix {
    tab,
    leaf,
    end_leaf,
    sub_dir_tab,
}

fn get_tree_prefix(prefix_type: TreePrefix) -> String {
    match prefix_type {
        TreePrefix::tab => "    ".to_string(),
        TreePrefix::leaf => "├── ".to_string(),
        TreePrefix::end_leaf => "└── ".to_string(),
        TreePrefix::sub_dir_tab => "│   ".to_string(),
    }
}

pub struct Prefix {
    prefix: VecDeque<String>,
    mode: PrefixMode,
}

impl Prefix {
    pub fn new() -> Prefix {
        Prefix {
            prefix: VecDeque::new(),
            mode: if Setting::is_no_indentation() {
                PrefixMode::none
            } else {
                PrefixMode::file_tree
            },
        }
    }

    pub fn set_init_value(&mut self, init_prefix: String) {
        match self.mode {
            PrefixMode::file_tree => self.prefix.push_back(init_prefix),
            _ => {}
        }
    }

    pub fn add_prefix(&mut self, is_begin: bool, is_last: bool, is_dir: bool) {
        match &self.mode {
            PrefixMode::file_tree => {
                if is_dir {
                    self.prefix.pop_back();
                    if is_last {
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::tab).clone());
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::end_leaf).clone());
                    } else {
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::sub_dir_tab).clone());
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::leaf).clone());
                    }
                } else {
                    if is_last {
                        self.prefix.pop_back();
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::end_leaf).clone());
                    } else if is_begin {
                        self.prefix.pop_back();
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::leaf).clone());
                    }
                }
            }
            PrefixMode::none => {
                // no operation is needed here
            }
        }
    }

    pub fn remove_prefix(&mut self, is_next_last: bool, is_dir: bool) {
        match &self.mode {
            PrefixMode::file_tree => {
                if is_dir {
                    self.prefix.pop_back();
                    self.prefix.pop_back();
                    if is_next_last {
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::end_leaf).clone())
                    } else {
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::leaf).clone())
                    }
                } else {
                    // no operation is needed here
                }
            }
            PrefixMode::none => {
                // no operation is needed here
            }
        }
    }

    pub fn print(&self) {
        for item in &self.prefix {
            print!("{}", item);
        }
    }
}

pub struct Entry {
    path: PathBuf,
    is_dir: bool,
    is_visible: bool,
    path_prefix: PathBuf,
    entry_name: String,
    metadata: Metadata,
}

impl Entry {
    pub fn new(path: PathBuf) -> Entry {
        Entry {
            is_dir: path.is_dir(),
            is_visible: Entry::visible_or_not(
                path.file_name()
                    .expect(path.to_str().unwrap())
                    .to_str()
                    .unwrap(),
            ),
            path_prefix: path.parent().unwrap().to_path_buf(),
            entry_name: String::from(path.file_name().unwrap().to_str().unwrap()),
            metadata: path.metadata().expect(path.to_str().unwrap()),
            path,
        }
    }

    pub fn print(&self) {
        println!("{}", self.entry_name);
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn traverse(&self) -> Result<Vec<Entry>, std::io::Error> {
        if !self.is_dir {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "cannot open a file as dir",
            ));
        }
        let mut path_list: Vec<Entry> = fs::read_dir(&self.path)?
            .map(|item| -> Entry {
                match item {
                    Ok(sth) => Entry::new(sth.path()),
                    _ => Entry::new(PathBuf::new()), // not correct, need to return error
                }
            })
            .collect();

        // sort to invisible first to avoid prefix error. not a correct way
        path_list.sort_by(Entry::invisible_file_first);

        if Setting::is_unsort() {
            return Ok(path_list);
        }

        if Setting::is_dir_first() {
            path_list.sort_by(Entry::dir_first)
        }

        if Setting::is_sort_alphanumerically() {
            path_list.sort_by_key(|entry| entry.entry_name.clone())
        }

        if Setting::is_sort_mod_time() {
            path_list.sort_by(Entry::sort_by_modified_time);
        }

        if Setting::is_sort_reverse() {
            path_list.reverse();
        }

        Ok(path_list)
    }

    fn visible_or_not(name: &str) -> bool {
        if let Some(character) = name.get(0..1) {
            !(character == ".")
        } else {
            false
        }
    }

    // sort functions
    fn invisible_file_first(a: &Entry, b: &Entry) -> Ordering {
        if !(a.is_visible ^ b.is_visible) {
            return Ordering::Equal;
        } else if !a.is_visible {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }

    fn dir_first(a: &Entry, b: &Entry) -> Ordering {
        if !(a.is_dir ^ b.is_dir) {
            return Ordering::Equal;
        } else if !a.is_dir {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }

    fn sort_by_modified_time(a: &Entry, b: &Entry) -> Ordering {
        let a_time = match a.metadata.modified() {
            Ok(time) => time,
            Err(e) => SystemTime::now(),
        };
        let b_time = match a.metadata.modified() {
            Ok(time) => time,
            Err(e) => SystemTime::now(),
        };
        a_time.cmp(&b_time)
    }
}
