use crate::envir::Setting;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs;
use std::fs::Metadata;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::PathBuf;
use std::time::SystemTime;

enum PrefixMode {
    FileTree,
    None,
}

enum TreePrefix {
    Tab,
    Leaf,
    EndLeaf,
    SubDirTab,
}

fn get_tree_prefix(prefix_type: TreePrefix) -> String {
    match prefix_type {
        TreePrefix::Tab => "    ".to_string(),
        TreePrefix::Leaf => "├── ".to_string(),
        TreePrefix::EndLeaf => "└── ".to_string(),
        TreePrefix::SubDirTab => "│   ".to_string(),
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
                PrefixMode::None
            } else {
                PrefixMode::FileTree
            },
        }
    }

    pub fn set_init_value(&mut self, init_prefix: String) {
        match self.mode {
            PrefixMode::FileTree => self.prefix.push_back(init_prefix),
            _ => {}
        }
    }

    pub fn add_prefix(&mut self, is_begin: bool, is_last: bool, is_dir: bool) {
        match &self.mode {
            PrefixMode::FileTree => {
                if is_dir {
                    self.prefix.pop_back();
                    if is_last {
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::Tab).clone());
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::EndLeaf).clone());
                    } else {
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::SubDirTab).clone());
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::Leaf).clone());
                    }
                } else {
                    if is_last {
                        self.prefix.pop_back();
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::EndLeaf).clone());
                    } else if is_begin {
                        self.prefix.pop_back();
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::Leaf).clone());
                    }
                }
            }
            PrefixMode::None => {
                // no operation is needed here
            }
        }
    }

    pub fn remove_prefix(&mut self, is_next_last: bool, is_dir: bool) {
        match &self.mode {
            PrefixMode::FileTree => {
                if is_dir {
                    self.prefix.pop_back();
                    self.prefix.pop_back();
                    if is_next_last {
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::EndLeaf).clone())
                    } else {
                        self.prefix
                            .push_back(get_tree_prefix(TreePrefix::Leaf).clone())
                    }
                } else {
                    // no operation is needed here
                }
            }
            PrefixMode::None => {
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
    pub metadata: Metadata,
    is_empty: bool, // identify fake entry
}

impl Entry {
    pub fn new(path: PathBuf) -> Entry {
        if path.exists() {
            Entry {
                is_dir: path.is_dir(),
                is_visible: Entry::visible_or_not(match path.file_name() {
                    Some(path) => path.to_str().unwrap(),
                    Option::None => "/",
                }),
                path_prefix: {
                    path.strip_prefix(Setting::get_root_prefix())
                        .unwrap()
                        .to_path_buf()
                },
                entry_name: String::from(match path.file_name() {
                    Some(path) => path.to_str().unwrap(),
                    Option::None => "/",
                }),
                metadata: path.metadata().unwrap(),
                path,
                is_empty: false,
            }
        } else {
            // fake empty entry
            Entry {
                is_dir: false,
                is_visible: false,
                path_prefix: path.clone(),
                path,
                entry_name: "".to_string(),
                metadata: fs::metadata("/").unwrap(),
                is_empty: true,
            }
        }
    }

    pub fn print(&self) {
        let mut entry_name_to_print = self.entry_name.clone();
        if Setting::is_full_path() {
            entry_name_to_print
                .insert_str(0, self.path_prefix.to_str().expect("not utf-8 filename"));
        }
        if Setting::is_quote() {
            println!("{:?}", entry_name_to_print);
        } else {
            println!("{}", entry_name_to_print);
        }
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn traverse(&self) -> Result<Vec<Entry>, std::io::Error> {
        // check
        if !self.is_dir {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "cannot open a file as dir",
            ));
        }

        // make entry list
        let mut path_list: Vec<Entry> = match fs::read_dir(&self.path) {
            Ok(list) => list,
            Err(e) => return Err(e),
        }
        .map(|item| -> Entry {
            match item {
                Ok(sth) => {
                    if sth.path().exists() {
                        Entry::new(sth.path())
                    } else {
                        Entry::new(PathBuf::new())
                    }
                }
                _ => Entry::new(PathBuf::new()), // not correct, need to return error
            }
        })
        .filter(Entry::filter)
        .collect();

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

    fn filter(item: &Entry) -> bool {
        // delete not exist file
        if item.is_empty {
            return false;
        }

        // -a
        if !Setting::is_all() && !item.is_visible {
            return false;
        }

        // -d
        if Setting::is_dir_only() && !item.is_dir {
            return false;
        }

        // pattern (-I or -P)
        if let Some((method, pattern, ignore_case)) = Setting::get_pattern() {
            let entry_name = if ignore_case {
                item.entry_name.to_lowercase()
            } else {
                item.entry_name.clone()
            };
            let p = if ignore_case {
                pattern.to_lowercase()
            } else {
                pattern
            };
            if method == 'i' {
                return entry_name.find(p.as_str()) == Option::None;
            } else {
                return entry_name.find(p.as_str()) != Option::None;
            }
        }

        return true;
    }

    fn visible_or_not(name: &str) -> bool {
        if let Some(character) = name.get(0..1) {
            !(character == ".")
        } else {
            false
        }
    }

    // sort functions
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
            Err(_e) => SystemTime::now(),
        };
        let b_time = match b.metadata.modified() {
            Ok(time) => time,
            Err(_e) => SystemTime::now(),
        };
        a_time.cmp(&b_time)
    }
}

pub struct EntryAttr {
    content: String,
}

impl EntryAttr {
    pub fn new(metadata: &Metadata) -> EntryAttr {
        let mut cont = String::new();
        if Setting::need_protection() {
            EntryAttr::setup_protection(metadata, &mut cont);
        }
        if Setting::need_uid() {
            EntryAttr::setup_uid(metadata, &mut cont);
        }
        if Setting::need_gid() {
            EntryAttr::setup_gid(metadata, &mut cont);
        }
        if Setting::need_size() != 0 {
            EntryAttr::setup_size(metadata, &mut cont);
        }
        if Setting::need_ctime() {
            EntryAttr::setup_time(metadata, &mut cont);
        }
        if Setting::need_inode() {
            EntryAttr::setup_inode(metadata, &mut cont);
        }
        if Setting::need_device() {
            EntryAttr::setup_device(metadata, &mut cont);
        }

        EntryAttr { content: cont }
    }

    pub fn print(&self) {
        print!("[{:}] ", self.content);
    }

    fn setup_protection(metadata: &Metadata, content: &mut String) {
        let flags_bit = vec![
            0b0100_000_000,
            0b0010_000_000,
            0b0001_000_000,
            0b0000_100_000,
            0b0000_010_000,
            0b0000_001_000,
            0b0000_000_100,
            0b0000_000_010,
            0b0000_000_001,
        ];
        let mode = metadata.permissions().mode();
        let flags_char = "rwxrwxrwx";

        let mut protection = String::from(if metadata.is_dir() { "d" } else { "-" });

        for i in 0..9 {
            if mode & flags_bit[i] == 0 {
                protection.push('-');
            } else {
                protection.push(flags_char.chars().nth(i).unwrap());
            }
        }
        content.push_str(&protection);
    }

    fn setup_uid(metadata: &Metadata, content: &mut String) {
        content.push_str(&format!(" {:}", metadata.uid()))
    }

    fn setup_gid(metadata: &Metadata, content: &mut String) {
        content.push_str(&format!(" {:}", metadata.gid()))
    }

    fn setup_size(metadata: &Metadata, content: &mut String) {
        let raw_size = metadata.size();
        match Setting::need_size() {
            1 => content.push_str(&format!(" {:}", raw_size)),
            2 => content.push_str(&EntryAttr::convert_size(raw_size, 1024)),
            3 => content.push_str(&EntryAttr::convert_size(raw_size, 1000)),
            _ => panic!(),
        }
    }

    /*
        show it in UNIX timestamp style.
    */
    fn setup_time(metadata: &Metadata, content: &mut String) {
        content.push_str(&format!(" {:}", metadata.ctime()))
    }

    fn setup_inode(metadata: &Metadata, content: &mut String) {
        content.push_str(&format!(" {:}", metadata.ino()))
    }

    fn setup_device(metadata: &Metadata, content: &mut String) {
        content.push_str(&format!(" {:}", metadata.dev()))
    }

    fn convert_size(raw_size: u64, base: u16) -> String {
        let unit = vec!["B", "K", "M", "G", "T", "P"];
        let mut size: f64 = raw_size as f64;
        let mut count: usize = 0;
        while size > base.into() {
            size /= base as f64;
            count += 1;
        }
        size = (size * 10.0).round() / 10.0;
        format!(" {:4}{}", size, unit[count])
    }
}
