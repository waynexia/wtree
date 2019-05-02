use std::collections::VecDeque;

use crate::envir::Setting;
use crate::print::*;

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
        match self.mode{
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

pub struct Entry {}
