use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{fs, path::PathBuf, process::exit};
use strum::Display;
use tabled::{
    Table, Tabled,
    settings::{
        Color, Style,
        object::{Columns, Rows},
    },
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, Parser)]
#[command(
    author = "LordPaijo (Iyan)",
    version,
    about,
    long_about = "Iyan's ls tool."
)]
struct Args {
    path: Option<PathBuf>,
    #[arg(
        short,
        long,
        help = "Filter files by pattern (like grep - matches filenames containing the pattern)"
    )]
    grab: Option<String>,
    #[arg(short, long, help = "Format output to JSON")]
    json: bool,
    #[arg(alias = "jsx", long, help = "Export JSON output to a file")]
    json_export: Option<PathBuf>,
    #[arg(short = 'n', long, help = "Hide line numbers")]
    no_line_numbers: bool,
    #[arg(short = 'o', long, help = "Show permissions in octal format")]
    octal_perms: bool,
    #[arg(short = 'r', long, help = "Show raw file sizes in bytes only")]
    raw_size: bool,
}

#[derive(Debug, Display, Serialize)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled(rename = "#")]
    #[serde(skip_serializing_if = "String::is_empty")]
    line_number: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    e_type: EntryType,
    #[tabled(rename = "Permissions")]
    permissions: String,
    #[tabled(rename = "Size")]
    size: String,
    #[tabled(rename = "Modified Date")]
    modified: String,
    #[tabled(skip, rename = "Raw Size")]
    #[serde(rename = "raw_size")]
    raw_size: u64,
}

fn main() {
    let args = Args::parse();
    let path = args.path.unwrap_or(PathBuf::from("."));

    if let Ok(exists) = fs::exists(&path) {
        if exists {
            let files = get_file(
                &path,
                &args.grab,
                !args.no_line_numbers,
                args.octal_perms,
                args.raw_size,
            );
            if args.json {
                println!("{}", serde_json::to_string_pretty(&files).unwrap());
            } else {
                print_table_from_files(&files, &args.grab, args.raw_size);
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

fn export_json(
    files: &[FileEntry],
    export_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let json_content = serde_json::to_string_pretty(files)?;
    fs::write(export_path, json_content)?;
    Ok(())
}

fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    const THRESHOLD: f64 = 1000.0;

    if size == 0 {
        return "   0 B".to_string();
    }

    let size_f = size as f64;
    let unit_index = (size_f.log10() / THRESHOLD.log10()).floor() as usize;
    let unit_index = unit_index.min(UNITS.len() - 1);

    if unit_index == 0 {
        format!("{:>4} B", size)
    } else {
        let scaled_size = size_f / THRESHOLD.powi(unit_index as i32);
        let formatted_number = if scaled_size >= 100.0 {
            format!("{:.0}", scaled_size)
        } else if scaled_size >= 10.0 {
            format!("{:.1}", scaled_size)
        } else {
            format!("{:.2}", scaled_size)
        };

        format!("{:>4} {}", formatted_number, UNITS[unit_index])
    }
}

fn print_table_from_files(files: &[FileEntry], pattern: &Option<String>, raw_size: bool) {
    if files.is_empty() {
        if pattern.is_some() {
            println!("{}", "No files found matching the pattern.".red());
            exit(1);
        } else {
            println!("{}", "Directory is empty.".yellow());
        }
    } else {
        let mut table = Table::new(files);
        table.with(Style::rounded());

        let has_line_numbers = files.first().map_or(false, |f| !f.line_number.is_empty());

        if has_line_numbers {
            table.modify(Columns::first(), Color::FG_BRIGHT_WHITE);
            table.modify(Columns::one(1), Color::FG_BRIGHT_CYAN);
            table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
            table.modify(Columns::one(3), Color::FG_BRIGHT_MAGENTA);
            table.modify(Columns::one(4), Color::FG_BRIGHT_YELLOW);
            if raw_size {
                table.modify(Columns::one(6), Color::FG_BRIGHT_YELLOW);
            }
        } else {
            table.modify(Columns::one(0), Color::FG_BRIGHT_CYAN);
            table.modify(Columns::one(1), Color::FG_BRIGHT_MAGENTA);
            table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
            table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
            if raw_size {
                table.modify(Columns::one(5), Color::FG_BRIGHT_YELLOW);
            }
        }
        table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
        println!("{}", table);
    }
}

fn get_file(
    path: &PathBuf,
    pattern: &Option<String>,
    show_line_numbers: bool,
    octal_perms: bool,
    raw_size: bool,
) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        let mut line_number = 1;
        for entry in read_dir {
            if let Ok(file) = entry {
                if should_include_file(&file, pattern) {
                    map_data(
                        file,
                        &mut data,
                        if show_line_numbers {
                            line_number.to_string()
                        } else {
                            String::new()
                        },
                        octal_perms,
                        raw_size,
                    );
                    line_number += 1;
                }
            }
        }
    }
    data
}

fn should_include_file(file: &fs::DirEntry, pattern: &Option<String>) -> bool {
    let Some(search_pattern) = pattern else {
        return true;
    };
    let filename = file.file_name().to_string_lossy().to_lowercase();
    filename.contains(&search_pattern.to_lowercase())
}

fn map_data(
    file: fs::DirEntry,
    data: &mut Vec<FileEntry>,
    line_number: String,
    octal_perms: bool,
    raw_size: bool,
) {
    if let Ok(meta) = fs::metadata(file.path()) {
        let file_size = meta.len();
        data.push(FileEntry {
            line_number,
            name: file
                .file_name()
                .into_string()
                .unwrap_or("Unknown name.".into()),
            e_type: if meta.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            },
            permissions: if octal_perms {
                format_permissions_octal(&meta)
            } else {
                format_permissions_rwx(&meta)
            },
            size: if raw_size {
                file_size.to_string()
            } else {
                format_size(file_size)
            },
            raw_size: file_size,
            modified: if let Ok(modi) = meta.modified() {
                let date: DateTime<Utc> = modi.into();
                format!("{}", date.format("%a %b %e %Y"))
            } else {
                String::default()
            },
        });
    }
}

#[cfg(unix)]
fn format_permissions_rwx(metadata: &fs::Metadata) -> String {
    let mode = metadata.permissions().mode();
    let permissions = mode & 0o777;
    let mut perm_str = String::with_capacity(9);

    // Owner permissions
    perm_str.push(if permissions & 0o400 != 0 { 'r' } else { '-' });
    perm_str.push(if permissions & 0o200 != 0 { 'w' } else { '-' });
    perm_str.push(if permissions & 0o100 != 0 { 'x' } else { '-' });

    // Group permissions
    perm_str.push(if permissions & 0o040 != 0 { 'r' } else { '-' });
    perm_str.push(if permissions & 0o020 != 0 { 'w' } else { '-' });
    perm_str.push(if permissions & 0o010 != 0 { 'x' } else { '-' });

    // Other permissions
    perm_str.push(if permissions & 0o004 != 0 { 'r' } else { '-' });
    perm_str.push(if permissions & 0o002 != 0 { 'w' } else { '-' });
    perm_str.push(if permissions & 0o001 != 0 { 'x' } else { '-' });

    perm_str
}

#[cfg(unix)]
fn format_permissions_octal(metadata: &fs::Metadata) -> String {
    let mode = metadata.permissions().mode();
    let permissions = mode & 0o777;
    format!("{:03o}", permissions)
}

#[cfg(windows)]
fn format_permissions_rwx(metadata: &fs::Metadata) -> String {
    let permissions = metadata.permissions();

    if permissions.readonly() {
        "r--r--r--".to_string()
    } else {
        "rw-rw-rw-".to_string()
    }
}

#[cfg(windows)]
fn format_permissions_octal(metadata: &fs::Metadata) -> String {
    let permissions = metadata.permissions();

    if permissions.readonly() {
        "444".to_string()
    } else {
        "666".to_string()
    }
}
