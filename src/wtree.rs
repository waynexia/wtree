use crate::utils::*;
extern crate lazy_static;
use crate::envir::{Setting, SETTING};
use crate::print::send;

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
        if SETTING.is_needing_report {
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
    let root_entry = Entry::new(Setting::get_root());
    send(&prefix, &root_entry);
    prefix.set_init_value("├── ".to_string());
    print_subdir(&root_entry, &mut prefix, &mut counter, Setting::get_level())?;
    counter.print_counter();

    Ok(())
}

fn print_subdir(
    root: &Entry,
    prefix: &mut Prefix,
    counter: &mut Counter,
    level_limit: i32,
) -> std::io::Result<()> {
    if level_limit == 0 {
        return Ok(());
    }
    let path_list = match root.traverse() {
        Ok(list) => list,
        Err(_) => return Ok(()),
    };

    let file_num = path_list.len();

    let mut iter_cnt = 0;
    for path in path_list {
        iter_cnt += 1;

        // identify the last item
        prefix.add_prefix(iter_cnt == 1, iter_cnt == file_num, false);

        send(&prefix, &path);
        counter.increase_counter(path.is_dir());

        // is dir
        if path.is_dir() {
            prefix.add_prefix(false, iter_cnt == file_num, true);

            // recursive
            print_subdir(&path, prefix, counter, level_limit - 1)?;

            // recover prefix
            prefix.remove_prefix(iter_cnt + 1 == file_num, true);
        }
    }

    Ok(())
}
