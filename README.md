# wtree
[![Build Status](https://travis-ci.com/waynexia/wtree.svg?token=NgxPfMcELg2LiMt86zCA&branch=master)](https://travis-ci.com/waynexia/wtree)

a simple `tree` command

# options
- ## Listing options
  - [x] -a            All files are listed.

  - [x] -d            List directories only.

  - [ ] -l            Follow symbolic links like directories.

  - [x] -f            Print the full path prefix for each file. *different behavier with `tree`*

  - [ ] -x            Stay on current filesystem only.

  - [ ] -L level      Descend only level directories deep.

  - [ ] -R            Rerun tree when max dir level reached.

  - [x] -P pattern    List only those files that match the pattern given. *different behavier with `tree`*

  - [x] -I pattern    Do not list files that match the given pattern. *different behavier with `tree`*

  - [x] --ignore-case Ignore case when pattern matching.

  - [ ] --matchdirs   Include directory names in -P pattern matching.

  - [x] --noreport    Turn off file/directory count at end of tree listing.

  - [ ] --charset X   Use charset X for terminal/HTML and indentation line output.

  - [ ] --filelimit # Do not descend dirs with more than # files in them.

  - [ ] --timefmt <f> Print and format time according to the format <f>.

  - [ ] -o filename   Output to file instead of stdout.

- ## File options
  - [ ] -q            Print non-printable characters as '?'.

  - [ ] -N            Print non-printable characters as is.

  - [x] -Q            Quote filenames with double quotes.

  - [ ] -p            Print the protections for each file.

  - [ ] -u            Displays file owner or UID number.

  - [ ] -g            Displays file group owner or GID number.

  - [ ] -s            Print the size in bytes of each file.

  - [ ] -h            Print the size in a more human readable way.

  - [ ] --si          Like -h, but use in SI units (powers of 1000).

  - [ ] -D            Print the date of last modification or (-c) status change.

  - [ ] -F            Appends '/', '=', '*', '@', '|' or '>' as per ls -F.

  - [ ] --inodes      Print inode number of each file.

  - [ ] --device      Print device ID number to which each file belongs.

- ## Sorting options
  - [x] -v            Sort files alphanumerically by version.

  - [x] -t            Sort files by last modification time.

  - [ ] -c            Sort files by last status change time.

  - [x] -U            Leave files unsorted.

  - [x] -r            Reverse the order of the sort.

  - [x] --dirsfirst   List directories before files (-U disables).

  - [ ] --sort X      Select sort: name,version,size,mtime,ctime.

- ## Graphics options
  - [x] -i            Don't print indentation lines.

  - [ ] -A            Print ANSI lines graphic indentation lines.

  - [ ] -S            Print with CP437 (console) graphics indentation lines.

  - [ ] -n            Turn colorization off always (-C overrides).

  - [ ] -C            Turn colorization on always.

- ## XML/HTML/JSON options
  - [ ] -X            Prints out an XML representation of the tree.

  - [ ] -J            Prints out an JSON representation of the tree.

  - [ ] -H baseHREF   Prints out HTML format with baseHREF as top directory.

  - [ ] -T string     Replace the default HTML title and H1 header with string.

  - [ ] --nolinks     Turn off hyperlinks in HTML output.

- ## Input options
  - [ ] --fromfile    Reads paths from files (.=stdin)

- ## Miscellaneous options
  - [ ] --version     Print version and exit.

  - [ ] --help        Print usage and this help message and exit.

  - [ ] --            Options processing terminator.

# bugs
- files that cannot read metadata or cannot open will not take count and print