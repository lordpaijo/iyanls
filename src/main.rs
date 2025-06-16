use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
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
}

#[derive(Debug, Display)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled)]
struct FileEntry {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    e_type: EntryType,
    #[tabled(rename = "Permissions")]
    permissions: String,
    #[tabled(rename = "Size (b)")]
    size: u64,
    #[tabled(rename = "Modified Date")]
    modified: String,
}

fn main() {
    let args = Args::parse();
    let path = args.path.unwrap_or(PathBuf::from("."));

    if let Ok(exists) = fs::exists(&path) {
        if exists {
            let pattern = args.grab;
            let get_files = get_file(&path, &pattern);

            if get_files.is_empty() {
                if pattern.is_some() {
                    println!("{}", "No files found matching the pattern.".red());
                    exit(1);
                } else {
                    println!("{}", "Directory is empty.".yellow());
                }
                return;
            }

            let mut table = Table::new(get_files);
            table.with(Style::rounded());
            table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
            table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
            table.modify(Columns::one(3), Color::FG_BRIGHT_MAGENTA);
            table.modify(Columns::one(4), Color::FG_BRIGHT_YELLOW);
            table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
            println!("{}", table);
        } else {
            eprintln!("{}", "Path does not exist.".red());
            exit(1);
        }
    } else {
        eprintln!("{}", "Error checking path.".red());
        exit(1);
    }
}

fn get_file(path: &PathBuf, pattern: &Option<String>) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                if should_include_file(&file, pattern) {
                    map_data(file, &mut data);
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

fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(file.path()) {
        data.push(FileEntry {
            name: file
                .file_name()
                .into_string()
                .unwrap_or("Unknown name.".into()),
            e_type: if meta.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            },
            permissions: format_permissions(&meta),
            size: meta.len(),
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
fn format_permissions(metadata: &fs::Metadata) -> String {
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

#[cfg(windows)]
fn format_permissions(metadata: &fs::Metadata) -> String {
    let permissions = metadata.permissions();

    if permissions.readonly() {
        "r--r--r--".to_string()
    } else {
        "rw-rw-rw-".to_string()
    }
}
