static version: &str ="1.0.0";

pub fn get_help_string() -> String {
    let help_string = format!(r#"
file-renamer v{} - A file renaming utility

    Usage: 
        file-renamer -h|--help
        file-renamer -V|--version
        file-renamer [-d|--directory <dir>] -f|--filter <filter> -n|--new-name <name> [-r] [-v|--verbose]   [--dry-run] [--wildcard-char <char>] [--position-select-wrapper <chars>]

    Options:
        -h, --help                      Show this help message
        -v, --version                   Show version information
        -v, --verbose                   Enable verbose output
        --dry-run                       Run in dry-run mode aka no changes
        -r, --recursive                 Recursively process directories
        -d, --directory                 Base directory to start processing
        -f, --filter                    Filter to select filenames
        -n, --new-name                  New name for the files
        --wildcard-char                 Wildcard character for renaming
        --position-select-wrapper       Characters used to select wildcard cathched 
                                            strings by position

    Description:
        This program processes input files and a filter to include files that share specified strings and rename them into a specified name.
        The program can be run in dry-run mode to see what changes would be made without actually renaming any files.
        The program can also be run recursively to process all files in a directory and its subdirectories.
        The program supports wildcard characters to match specific parts of filenames and allows for custom selection of matched strings by position.
        The program also support use of Counters as wildcard characters to rename files in a sequence. (currently there is no way to order the files so there is no guarantie the counters will be applied in the desired order).
        The program can be run in verbose mode to see detailed output of the renaming process.

    Examples:
        file-renamer -d /path/to/directory -f "*.txt" -n "prefix_*" --wildcard-char * --position-select-wrapper ()
            result:
                file1.txt -> prefix_file1.txt
                file2.txt -> prefix_file2.txt
        file-renamer -d /path/to/directory -f "photo_file-$-$-$-.png" -n "photo_file$$$.png" --wildcard-char "$"
            result:
                photo_file-1-2-3.png -> photo_file123.png
        file-renamer -d /path/to/directory -f "{}.avi" -n "video_clips*.avi" --wildcard-char "*" 
            result:
                012014402065465842112.avi -> video_clips1.avi
                012014402065465d4211c.avi -> video_clips2.avi
                01201440206546584211a.avi -> video_clips3.avi
        file-renamer -d /photos/month-day-year/ -f "photo-*-*-*.png" -n "photo-(2)-(0)-(1)-png" --wildcard-char "*" --position-select-wrapper "()"
            result:
                photo-01-02-2021.png -> photo-2021-01-02-png
                photo-03-04-2022.png -> photo-2022-03-04-png
                photo-05-06-2024.png -> photo-2024-05-06-png
                photo-07-08-2024.png -> photo-2024-07-08-png
    "#, version, "{1:1}");
    help_string.to_string()
}

pub fn get_version() -> String {
    let version_string = format!("file-renamer v{}", version);
    version_string.to_string()
}