use crate::envir::SETTING;
use crate::utils::{Entry, EntryAttr, Prefix};
use std::fs;
use std::os::unix::fs::PermissionsExt;

/* todo: use enum */
fn set_color(color: u8) {
    let esc_char = vec![27];
    let esc = String::from_utf8(esc_char).unwrap();
    let bright: u8 = 1;
    print!("{}[{};{}m", esc, bright, color);
}

fn set_black() {
    let black: u8 = 30;
    set_color(black);
}
fn set_red() {
    let red: u8 = 31;
    set_color(red);
}

fn set_green() {
    let green: u8 = 32;
    set_color(green);
}

fn set_blue() {
    let blue: u8 = 34;
    set_color(blue);
}

fn reset() {
    let esc_char = vec![27];
    let esc = String::from_utf8(esc_char).unwrap();
    let reset: u8 = 0;
    print!("{}[{}m", esc, reset);
}

fn is_file_executable(metadata: &fs::Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}

fn need_print_attr() -> bool {
    SETTING.need_protection
        || SETTING.need_uid
        || SETTING.need_gid
        || SETTING.need_size != 0
        || SETTING.need_ctime
        || SETTING.need_inode
        || SETTING.need_device
}

pub fn send(prefix: &Prefix, entry: &Entry) {
    // print prefix
    prefix.print();

    // print attributes
    if need_print_attr() {
        let entry_attr = EntryAttr::new(&entry.metadata);
        entry_attr.print();
    }

    // print entry
    /* todo: use bit flag */
    if SETTING.is_color {
        if entry.is_dir() {
            set_blue();
        } else if is_file_executable(&entry.metadata) {
            set_green();
        }
    }
    entry.print();
    reset();
}
