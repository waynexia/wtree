use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;
use std::process::exit;

#[macro_use]
lazy_static! {
    static ref SETTING: Setting = parse_parameter();
}

pub struct Setting {
    pub is_all: bool,
    pub is_dir_only: bool,
    pub is_no_indentation: bool,
    pub is_sort_alphanumerically: bool,
    pub is_sort_mod_time: bool,
    pub is_unsort: bool,
    pub is_sort_reverse: bool,
    pub is_dir_first: bool,
    pub is_needing_report: bool,
    pub pattern_p: bool,
    pub pattern_i: bool,
    pub is_quote: bool,
    pub is_full_path: bool,
    pub pattern_ignore_case: bool,
    pub is_color: bool,        // -n, -C
    pub need_protection: bool, // -p
    pub need_uid: bool,        // -u
    pub need_gid: bool,        // -g
    pub need_size: u8,         // -s, -h, --si
    pub need_ctime: bool,      // -D
    pub need_inode: bool,      // --inodes
    pub need_device: bool,     // --device

    pub pattern: String,
    pub root: PathBuf,
}

impl Default for Setting {
    fn default() -> Setting {
        Setting {
            is_all: false,
            is_dir_only: false,
            is_no_indentation: false,
            is_sort_alphanumerically: true,
            is_sort_mod_time: false,
            is_unsort: false,
            is_sort_reverse: false,
            is_dir_first: false,
            is_needing_report: true,
            is_quote: false,
            is_color: false,
            is_full_path: false,
            pattern_i: false,
            pattern_p: false,
            pattern_ignore_case: false,
            need_protection: false,
            need_uid: false,
            need_gid: false,
            need_size: 0,
            need_ctime: false,
            need_inode: false,
            need_device: false,
            pattern: String::new(),
            root: PathBuf::new(),
        }
    }
}

impl Setting {
    fn error_report(hint: String) {
        Setting::print_help();

        // print hint
        println!("{}", hint);
        exit(0);
    }

    fn print_help() {
        println! {"
usage: tree [-acdfghilnpqrstuvxACDFJQNSUX] [-H baseHREF] [-T title ]
[-L level [-R]] [-P pattern] [-I pattern] [-o filename] [--version]
[--help] [--inodes] [--device] [--noreport] [--nolinks] [--dirsfirst]
[--charset charset] [--filelimit[=]#] [--si] [--timefmt[=]<f>]
[--sort[=]<name>] [--matchdirs] [--ignore-case] [--fromfile] [--]
[<directory list>]
  ------- Listing options -------
  -a            All files are listed.
  -d            List directories only.
  -l            Follow symbolic links like directories.
  -f            Print the full path prefix for each file.
  -x            Stay on current filesystem only.
  -L level      Descend only level directories deep.
  -R            Rerun tree when max dir level reached.
  -P pattern    List only those files that match the pattern given.
  -I pattern    Do not list files that match the given pattern.
  --ignore-case Ignore case when pattern matching.
  --matchdirs   Include directory names in -P pattern matching.
  --noreport    Turn off file/directory count at end of tree listing.
  --charset X   Use charset X for terminal/HTML and indentation line output.
  --filelimit # Do not descend dirs with more than # files in them.
  --timefmt <f> Print and format time according to the format <f>.
  -o filename   Output to file instead of stdout.
  ------- File options -------
  -q            Print non-printable characters as '?'.
  -N            Print non-printable characters as is.
  -Q            Quote filenames with double quotes.
  -p            Print the protections for each file.
  -u            Displays file owner or UID number.
  -g            Displays file group owner or GID number.content
  -s            Print the size in bytes of each file.content
  -h            Print the size in a more human readable way.
  --si          Like -h, but use in SI units (powers of 1000).
  -D            Print the date of last modification or (-c) status change.
  -F            Appends '/', '=', '*', '@', '|' or '>' as per ls -F.
  --inodes      Print inode number of each file.
  --device      Print device ID number to which each file belongs.
  ------- Sorting options -------
  -v            Sort files alphanumerically by version.
  -t            Sort files by last modification time.
  -c            Sort files by last status change time.
  -U            Leave files unsorted.
  -r            Reverse the order of the sort.
  --dirsfirst   List directories before files (-U disables).
  --sort X      Select sort: name,version,size,mtime,ctime.
  ------- Graphics options -------
  -i            Don't print indentation lines.
  -A            Print ANSI lines graphic indentation lines.
  -S            Print with CP437 (console) graphics indentation lines.
  -n            Turn colorization off always (-C overrides).
  -C            Turn colorization on always.
  ------- XML/HTML/JSON options -------
  -X            Prints out an XML representation of the tree.
  -J            Prints out an JSON representation of the tree.
  -H baseHREF   Prints out HTML format with baseHREF as top directory.
  -T string     Replace the default HTML title and H1 header with string.
  --nolinks     Turn off hyperlinks in HTML output.
  ------- Input options -------
  --fromfile    Reads paths from files (.=stdin)
  ------- Miscellaneous options -------
  --version     Print version and exit.
  --help        Print usage and this help message and exit.
  --            Options processing terminator.
        "}
    }
}

fn need_extra_para(flag: &String) -> bool {
    flag.eq("H")
        || flag.eq("T")
        || flag.eq("L")
        || flag.eq("P")
        || flag.eq("I")
        || flag.eq("o")
        || flag.eq("charset")
        || flag.eq("filelimit")
        || flag.eq("timefmt")
        || flag.eq("sort")
}

/*
    @brief
        insert into decomposed args,
        check for flags that need extra parameter
*/
fn vec_push_hook(
    vec: &mut Vec<String>,
    item: &String,
    expect_pattern: &mut bool,
) -> Result<(), ()> {
    if *expect_pattern == true {
        return Err(());
    }

    if need_extra_para(item) {
        *expect_pattern = true;
    }

    vec.push(item.clone());
    Ok(())
}

fn decompose_arg(args: &Vec<String>) -> Result<Vec<String>, ()> {
    let mut expect_pattern = false;
    let mut ret_args: Vec<String> = Vec::new();

    for arg in args {
        if arg.starts_with("--") {
            if let Some(substr) = arg.get(2..) {
                vec_push_hook(&mut ret_args, &substr.to_string(), &mut expect_pattern)?
            };
        } else if arg.starts_with("-") {
            if let Some(substr) = arg.get(1..) {
                for abbr in substr.chars() {
                    vec_push_hook(&mut ret_args, &abbr.to_string(), &mut expect_pattern)?;
                }
            }
        } else if expect_pattern {
            ret_args.push(arg.to_string());
            expect_pattern = false;
        } else {
            return Err(());
        }
    }
    Ok(ret_args)
}

fn parse_parameter() -> Setting {
    let mut args: Vec<String> = env::args().collect();
    // remove the first arg which is command name
    args.remove(0);
    //extract last parameter, if not a path, put it back
    let root_path: PathBuf = if let Some(path) = args.pop() {
        if PathBuf::from(path.clone()).is_dir() {
            PathBuf::from(path.clone())
        } else {
            args.push(path);
            PathBuf::from("./".to_string())
        }
    } else {
        PathBuf::from("./".to_string())
    }
    .canonicalize()
    .unwrap();
    let mut ret = Setting {
        root: root_path,
        ..Default::default()
    };

    let wtf = match decompose_arg(&args) {
        Ok(vec) => vec,
        Err(_) => {
            Setting::print_help();
            exit(0);
        }
    };
    let mut args_iter = wtf.iter().peekable();

    while args_iter.peek() != Option::None {
        let i: &String = args_iter.next().unwrap();
        match i.as_ref() {
            "a" => ret.is_all = true,
            "d" => ret.is_dir_only = true,
            "i" => ret.is_no_indentation = true,
            "v" => ret.is_sort_alphanumerically = true,
            "t" => ret.is_sort_mod_time = true,
            "U" => ret.is_unsort = true,
            "r" => ret.is_sort_reverse = true,
            "dirsfirst" => ret.is_dir_first = true,
            "noreport" => ret.is_needing_report = false,
            "ignore-case" => ret.pattern_ignore_case = true,
            "Q" => ret.is_quote = true,
            "f" => ret.is_full_path = true,
            // -n will be overwrite, no reaction
            "C" => ret.is_color = true,
            "p" => ret.need_protection = true,
            "u" => ret.need_uid = true,
            "g" => ret.need_gid = true,
            "D" => ret.need_ctime = true,
            "inodes" => ret.need_inode = true,
            "device" => ret.need_device = true,
            // -s, -h, --si will override others
            "s" => ret.need_size = 1,
            "h" => ret.need_size = 2,
            "si" => ret.need_size = 3,
            "P" => {
                ret.pattern_p = true;
                // only can exist one pattern
                if ret.pattern_p && ret.pattern_i {
                    ret.pattern_i = false;
                }
                let pattern: &str = args_iter.peek().expect("need a pattern here");
                ret.pattern = pattern.to_string();
            }
            "I" => {
                ret.pattern_i = true;
                // only can exist one pattern
                if ret.pattern_p && ret.pattern_i {
                    ret.pattern_p = false;
                }
                let pattern: &str = args_iter.peek().expect("need a pattern here");
                ret.pattern = pattern.to_string();
            }

            "help" => {
                Setting::print_help();
                exit(0);
            }
            "version" => {
                println!("wtree, version 0.2.0 by @waynexia");
                exit(0);
            }

            _ => Setting::error_report("Invalid argument: ".to_string() + i.as_ref()),
        }
        args_iter.next();
    }

    ret
}

impl Setting {
    pub fn get_root() -> PathBuf {
        SETTING.root.clone()
    }

    pub fn get_root_prefix() -> PathBuf {
        match SETTING.root.parent() {
            Some(path) => path.to_path_buf().clone(),
            _ => PathBuf::from("/"),
        }
    }

    pub fn get_pattern() -> Option<(char, String, bool)> {
        if !(SETTING.pattern_p || SETTING.pattern_i) {
            return Option::None;
        } else {
            return Some((
                if SETTING.pattern_p { 'p' } else { 'i' },
                SETTING.pattern.clone(),
                SETTING.pattern_ignore_case,
            ));
        }
    }

    /* todo: remove this hellll */

    pub fn is_all() -> bool {
        SETTING.is_all
    }

    pub fn is_dir_only() -> bool {
        SETTING.is_dir_only
    }

    pub fn is_quote() -> bool {
        SETTING.is_quote
    }

    pub fn is_full_path() -> bool {
        SETTING.is_full_path
    }

    pub fn is_no_indentation() -> bool {
        SETTING.is_no_indentation
    }

    pub fn is_sort_alphanumerically() -> bool {
        SETTING.is_sort_alphanumerically
    }

    pub fn is_sort_mod_time() -> bool {
        SETTING.is_sort_mod_time
    }

    pub fn is_unsort() -> bool {
        SETTING.is_unsort
    }

    pub fn is_sort_reverse() -> bool {
        SETTING.is_sort_reverse
    }

    pub fn is_dir_first() -> bool {
        SETTING.is_dir_first
    }

    pub fn is_needing_report() -> bool {
        SETTING.is_needing_report
    }

    pub fn is_color() -> bool {
        SETTING.is_color
    }

    pub fn need_protection() -> bool {
        SETTING.need_protection
    }

    pub fn need_uid() -> bool {
        SETTING.need_uid
    }

    pub fn need_gid() -> bool {
        SETTING.need_gid
    }

    pub fn need_size() -> u8 {
        SETTING.need_size
    }

    pub fn need_ctime() -> bool {
        SETTING.need_ctime
    }

    pub fn need_inode() -> bool {
        SETTING.need_inode
    }

    pub fn need_device() -> bool {
        SETTING.need_device
    }
}

#[test]
fn test_fn_decompose_arg() {
    let args = decompose_arg(&vec![String::from("-vga"), String::from("--inodes")]).unwrap();
    assert_eq!(
        args,
        vec![
            String::from("v"),
            String::from("g"),
            String::from("a"),
            String::from("inodes")
        ]
    )
}

#[test]
#[should_panic]
fn decompose_arg_with_incorrect_input() {
    let _args = decompose_arg(&vec![String::from("-Io")]).unwrap();
}
