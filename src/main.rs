mod cli;
mod filters;
mod renamer;
mod help;

use std::env;
use std::fs;
use std::path;

use colored::Colorize;


fn main() {
    let args: Vec<String> = env::args().collect();
    let cli = cli::CliParameters::new(args);
    if cli.Help {
        println!("{}", help::get_help_string());
        return;
    }

    if cli.Version {
        println!("{}", help::get_version());
        return;
    }
    
    let mut filter = filters::RenameFilter::new(
        cli.Filter.clone(), 
        cli.WildcardChar.clone()
    );
    let renamer = renamer::Renamer::new(
        cli.Output.clone(), 
        cli.WildcardChar.clone(), 
        cli.PositionSelectWrapper.clone()
    );

    if cli.DryRun {
        println!("Running in dry-run mode. No changes will be made.");
    }

    if cli.Verbose {
        println!("Verbose mode enabled.");
    }

    let mut directories:Vec<std::path::PathBuf> = vec![];
    directories.push(cli.Directory.clone()); 

    while !directories.is_empty(){
        let current_directory = directories.pop().unwrap();
        let abs_path = path::absolute(&current_directory).unwrap();
        println!("{}", &abs_path.to_str().unwrap().on_blue());
        for files in std::fs::read_dir(current_directory).unwrap() {
            match files {
                Ok(file)=>{
                    let file_path = file.path();
                    if file_path.is_dir() {
                        if cli.Recursive{
                            directories.push(file_path.clone());
                        }
                        continue;
                    }
                    let file_name_str = file_path.file_name().unwrap().to_str().unwrap();
                    if !filter.does_fulfill(&file_name_str){
                        continue;
                    }
                    let wildcard_catched = filter.collect_wildcards(&file_name_str);
                    let new_filename = renamer.generate_rename_filename(&wildcard_catched);
                    if cli.Verbose{
                        println!("\t{} -> {}", file_name_str.on_red(), new_filename.on_green());
                    }
                    if cli.DryRun{
                        continue;
                    }
                    let full_old_path = file_path.clone();
                    let full_new_path = file_path.parent().unwrap().join(new_filename);
                    _ = fs::rename(full_old_path, full_new_path); 
                },
                Err(e)=>{
                    println!("Error reading directory: {}", e);
                    continue;
                }
            }
        }
    }
}
