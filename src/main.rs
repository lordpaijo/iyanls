use clap::Parser;
use owo_colors::OwoColorize;
use std::{fs, path::PathBuf, process::exit};

mod cli;
mod display;
mod file_ops;
mod sorting;
mod types;
mod utils;

use cli::Args;
use display::{export_json, print_table_from_files};
use file_ops::get_file;
use sorting::{get_sort_order, sort_files};
use std::os::unix::io::AsRawFd;
use types::FileEntry;

fn main() {
    let tty_available = unsafe { libc::isatty(std::io::stdout().as_raw_fd()) == 1 };
    let args = Args::parse();
    let sort_order = get_sort_order(&args);

    let path = args.path.unwrap_or(PathBuf::from("."));
    let timezone = utils::parse_timezone(&args.timezone);

    if let Ok(exists) = fs::exists(&path) {
        if exists {
            let mut files = get_file(
                &path,
                &args.grab,
                &args.re_grab,
                !args.no_line_numbers,
                args.octal_perms,
                args.owner_type,
                &args.time_format,
                &timezone,
                &args.custom_format,
                args.deep,
                args.toggle_clock,
            );

            if let Some(order) = sort_order {
                sort_files(&mut files, &order);
            }

            if !args.no_line_numbers {
                add_line_numbers(&mut files);
            }

            if !tty_available {
                print_names_only(&files);
                exit(0);
            }

            if args.json {
                println!("{}", serde_json::to_string_pretty(&files).unwrap());
            } else if args.re_grab.is_none() {
                print_table_from_files(&files, &args.grab, !args.no_line_numbers);
            } else {
                print_table_from_files(&files, &args.re_grab, !args.no_line_numbers);
            }

            if let Some(export_path) = &args.json_export {
                if let Err(e) = export_json(&files, export_path) {
                    eprintln!("{}: {}", "Error writing JSON file".red(), e);
                    exit(1);
                }
                println!("{} {}", "JSON exported to:".green(), export_path.display());
            }
        } else {
            eprintln!("{}", "Path does not exist.".red());
            exit(1);
        }
    } else {
        eprintln!("{}", "Error checking path.".red());
        exit(1);
    }
}

fn add_line_numbers(files: &mut [FileEntry]) {
    for (index, file) in files.iter_mut().enumerate() {
        file.line_number = (index + 1).to_string();
    }
}

fn print_names_only(files: &[FileEntry]) {
    for file in files {
        println!("{}", file.name);
    }
}
