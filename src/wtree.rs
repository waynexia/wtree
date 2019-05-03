use std::fs;
use std::os::unix::fs::PermissionsExt;

use crate::utils::*;
extern crate lazy_static;
use crate::envir::Setting;

struct Counter {
    file_count: u32,
    dir_count: u32,
}
impl Counter {
    pub fn new() -> Counter {
        Counter {
            file_count: 0,
            dir_count: 0,
        }
    }

    pub fn increase_counter(&mut self, is_dir: bool) {
        if is_dir {
            self.dir_count += 1;
        } else {
            self.file_count += 1;
        }
    }

    pub fn print_counter(&self) {
        if Setting::is_needing_report() {
            println!(
                "\n{} directories, {} files",
                self.dir_count, self.file_count,
            );
        }
    }
}

pub fn print_tree() -> std::io::Result<()> {
    let mut counter = Counter::new();
    let mut prefix = Prefix::new();
    prefix.set_init_value("├── ".to_string());
    print_subdir(
        &Entry::new(std::path::PathBuf::from("./").canonicalize().unwrap()),
        &mut prefix,
        &mut counter,
    )?;
    counter.print_counter();

    Ok(())
}

fn print_subdir(root: &Entry, prefix: &mut Prefix, counter: &mut Counter) -> std::io::Result<()> {
    let path_list = root.traverse()?;

    let file_num = path_list.len();

    let mut iter_cnt = 0;
    for path in path_list {
        iter_cnt += 1;

        // identify the last item
        prefix.add_prefix(iter_cnt == 1, iter_cnt == file_num, false);

        prefix.print();
        path.print();
        counter.increase_counter(path.is_dir());

        // is dir
        if path.is_dir() {
            prefix.add_prefix(false, iter_cnt == file_num, true);

            // recursive
            print_subdir(&path, prefix, counter)?;

            // recover prefix
            prefix.remove_prefix(iter_cnt + 1 == file_num, true);
        }
    }

    Ok(())
}

fn is_file_executable(metadata: fs::Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}
