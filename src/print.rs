use crate::envir::Setting;
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

pub fn send(prefix: &Prefix, entry: &Entry) {
    prefix.print();

    let entry_attr = EntryAttr::new(&entry.metadata);
    entry_attr.print();

    /* todo: use bit flag */
    if Setting::is_color() {
        if entry.is_dir() {
            set_blue();
        } else if is_file_executable(&entry.metadata) {
            set_green();
        }
    }
    entry.print();
    reset();
}
