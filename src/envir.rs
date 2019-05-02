use std::env;
use lazy_static::lazy_static;

#[macro_use]
lazy_static! {
    static ref setting: Setting = parase_parameter();
}

pub struct Setting {
    pub is_all: bool,
    pub is_dir_only: bool,
    pub is_no_indentation: bool,
    pub root: String,
}

fn parase_parameter() -> Setting {

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    println!("{:?}", args);
    let mut ret = Setting {
        is_all: false,
        is_dir_only: false,
        is_no_indentation: false,
        root: "./".to_string(),
    };
    for i in args.iter() {
        match i.as_ref() {
            "-a" => ret.is_all = true,
            "-d" => ret.is_dir_only = true,
            "-i" => ret.is_no_indentation = true,

            _ => Setting::error_report("Invalid argument: ".to_string() + i.as_ref()),
        }
    }

    ret
}

impl Setting {
    fn error_report(hint: String) {
        Setting::print_help();

        // print hint
        println!("{}", hint);
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
    pub fn is_no_indentation() -> bool {
        if setting.is_no_indentation {
            true
        } else {
            false
        }
    }
}
