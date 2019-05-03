use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;

#[macro_use]
lazy_static! {
    static ref setting: Setting = parase_parameter();
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

    pub pattern: String,
    pub root: PathBuf,
}

fn parase_parameter() -> Setting {
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
    }.canonicalize().unwrap();

    //println!("{:?}",root_path);
    //panic!();

    let mut ret = Setting {
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
        is_full_path: false,
        pattern_i: false,
        pattern_p: false,
        pattern_ignore_case: false,
        pattern: "".to_string(),

        root: root_path.canonicalize().unwrap(),
    };

    let mut args_iter = args.iter().peekable();
    while args_iter.peek() != Option::None {
        let i: &String = args_iter.next().unwrap();
        match i.as_ref() {
            "-a" => ret.is_all = true,
            "-d" => ret.is_dir_only = true,
            "-i" => ret.is_no_indentation = true,
            "-v" => ret.is_sort_alphanumerically = true,
            "-t" => ret.is_sort_mod_time = true,
            "-U" => ret.is_unsort = true,
            "-r" => ret.is_sort_reverse = true,
            "--dirsfirst" => ret.is_dir_first = true,
            "--noreport" => ret.is_needing_report = false,
            "--ignore-case" => ret.pattern_ignore_case = true,
            "-Q" => ret.is_quote = true,
            "-f" => ret.is_full_path = true,
            "-P" => {
                ret.pattern_p = true;
                // only can exist one pattern
                if ret.pattern_p && ret.pattern_i {
                    ret.pattern_i = false;
                }
                let pattern: &str = args_iter.peek().expect("need a pattern here");
                ret.pattern = pattern.to_string();
            }
            "-I" => {
                ret.pattern_i = true;
                // only can exist one pattern
                if ret.pattern_p && ret.pattern_i {
                    ret.pattern_p = false;
                }
                let pattern: &str = args_iter.peek().expect("need a pattern here");
                ret.pattern = pattern.to_string();
            }

            _ => Setting::error_report("Invalid argument: ".to_string() + i.as_ref()),
        }
        args_iter.next();
    }

    ret
}

impl Setting {
    fn error_report(hint: String) {
        Setting::print_help();

        // print hint
        println!("{}", hint);
        panic!();
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
  -g            Displays file group owner or GID number.
  -s            Print the size in bytes of each file.
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

impl Setting {
    pub fn get_root() -> PathBuf {
        setting.root.clone()
    }

    pub fn get_root_prefix() ->PathBuf{
        match setting.root.parent(){
            Some(path) => path.to_path_buf().clone(),
            _ => PathBuf::from("/")
        }
    }

    pub fn get_pattern() -> Option<(char, String, bool)> {
        if !(setting.pattern_p || setting.pattern_i) {
            return Option::None;
        } else {
            return Some((
                if setting.pattern_p { 'p' } else { 'i' },
                setting.pattern.clone(),
                setting.pattern_ignore_case,
            ));
        }
    }

    pub fn is_all() -> bool {
        setting.is_all
    }

    pub fn is_dir_only() -> bool {
        setting.is_dir_only
    }

    pub fn is_quote() -> bool {
        setting.is_quote
    }

    pub fn is_full_path() -> bool {
        setting.is_full_path
    }

    pub fn is_no_indentation() -> bool {
        setting.is_no_indentation
    }

    pub fn is_sort_alphanumerically() -> bool {
        setting.is_sort_alphanumerically
    }

    pub fn is_sort_mod_time() -> bool {
        setting.is_sort_mod_time
    }

    pub fn is_unsort() -> bool {
        setting.is_unsort
    }

    pub fn is_sort_reverse() -> bool {
        setting.is_sort_reverse
    }

    pub fn is_dir_first() -> bool {
        setting.is_dir_first
    }

    pub fn is_needing_report() -> bool {
        setting.is_needing_report
    }
}
