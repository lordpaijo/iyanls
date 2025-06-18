use chrono::{DateTime, Utc};
use chrono_tz::{Tz, UTC};
use clap::{Parser, ValueEnum};
use owo_colors::OwoColorize;
use rayon::prelude::*;
use serde::Serialize;

#[cfg(unix)]
use std::os::unix::io::AsRawFd;

use std::{cmp::Ordering, fs, path::PathBuf, process::exit};
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

#[derive(Debug, Clone, ValueEnum)]
enum TimeFormat {
    Utc,
    Local,
    Unix,
    Iso8601,
    Rfc3339,
    Utf,
    Custom,
}

#[derive(Debug, Clone, ValueEnum)]
enum SortOrder {
    #[value(name = "up-to-date")]
    UpToDate,
    #[value(name = "down-to-date")]
    DownToDate,
    #[value(name = "largest-size")]
    LargestSize,
    #[value(name = "smallest-size")]
    SmallestSize,
    #[value(name = "alphabetical-order")]
    AlphabeticalOrder,
    #[value(name = "alphabetical-reverse")]
    AlphabeticalReverse,
}

#[derive(Debug, Parser)]
#[command(
    author = "LordPaijo (Iyan)",
    version,
    about,
    long_about = "Iyan's ls tool."
)]
struct Args {
    path: Option<PathBuf>,
    #[arg(short, long, help = "Toggle deep processing", default_value = "false")]
    deep: bool,
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
    #[arg(
        short = 't',
        long,
        value_enum,
        default_value = "local",
        help = "Time format for dates (utc, local, unix, iso8601, rfc3339, utf, custom)"
    )]
    time_format: TimeFormat,
    #[arg(
        long,
        help = "Custom time format string (used with --time-format=custom)",
        default_value = "%Y-%m-%d %H:%M:%S %Z"
    )]
    custom_format: String,
    #[arg(
        long,
        help = "Timezone for time display (e.g., US/Eastern, Europe/London, Asia/Tokyo)",
        default_value = "UTC"
    )]
    timezone: String,
    #[arg(long, help = "Toggle clock display", default_value = "false")]
    toggle_clock: bool,
    #[arg(short = 'U', long, help = "Sort files by newest modified to oldest")]
    up_to_date: bool,
    #[arg(short = 'D', long, help = "Sort files by oldest modified to newest")]
    down_to_date: bool,
    #[arg(short = 'X', long, help = "Sort files by largest size")]
    largest_size: bool,
    #[arg(short = 'S', long, help = "Sort files by smallest size")]
    smallest_size: bool,
    #[arg(short = 'A', long, help = "Sort files by alphabetical order")]
    alphabetical_order: bool,
    #[arg(short = 'B', long, help = "Sort files by reversed alphabetical order")]
    alphabetical_reverse: bool,
}

#[derive(Debug, Display, Serialize, Clone)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Serialize, Clone)]
struct FileEntry {
    #[serde(skip_serializing_if = "String::is_empty")]
    line_number: String,
    name: String,
    e_type: EntryType,
    permissions: String,
    size: String,
    modified: String,
    #[serde(skip)]
    raw_size: u64,
    #[serde(skip)]
    raw_modified: std::time::SystemTime,
}

#[derive(Debug, Tabled)]
struct TableRowWithLine {
    #[tabled(rename = "#")]
    line_number: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    e_type: String,
    #[tabled(rename = "Permissions")]
    permissions: String,
    #[tabled(rename = "Size")]
    size: String,
    #[tabled(rename = "Modified Date")]
    modified: String,
}

#[derive(Debug, Tabled)]
struct TableRowNoLine {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    e_type: String,
    #[tabled(rename = "Permissions")]
    permissions: String,
    #[tabled(rename = "Size")]
    size: String,
    #[tabled(rename = "Modified Date")]
    modified: String,
}

fn main() {
    #[cfg(unix)]
    let tty_available = unsafe { libc::isatty(std::io::stdout().as_raw_fd()) == 1 };

    let args = Args::parse();

    // Determine sort order before moving args.path
    let sort_order = get_sort_order(&args);

    let path = args.path.unwrap_or(PathBuf::from("."));
    let timezone = match args.timezone.parse::<Tz>() {
        Ok(tz) => tz,
        Err(_) => {
            eprintln!(
                "{}: '{}'. Using UTC instead.",
                "Invalid timezone".red(),
                args.timezone
            );
            UTC
        }
    };

    if let Ok(exists) = fs::exists(&path) {
        if exists {
            let mut files = get_file(
                &path,
                &args.grab,
                !args.no_line_numbers,
                args.octal_perms,
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
                for (index, file) in files.iter_mut().enumerate() {
                    file.line_number = (index + 1).to_string();
                }
            }

            #[cfg(unix)]
            if !tty_available {
                for file in &files {
                    println!("{}", file.name);
                }
                exit(0);
            }

            if args.json {
                println!("{}", serde_json::to_string_pretty(&files).unwrap());
            } else {
                print_table_from_files(&files, &args.grab, !args.no_line_numbers);
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

fn get_sort_order(args: &Args) -> Option<SortOrder> {
    let sort_flags = [
        (args.up_to_date, SortOrder::UpToDate),
        (args.down_to_date, SortOrder::DownToDate),
        (args.largest_size, SortOrder::LargestSize),
        (args.smallest_size, SortOrder::SmallestSize),
        (args.alphabetical_order, SortOrder::AlphabeticalOrder),
        (args.alphabetical_reverse, SortOrder::AlphabeticalReverse),
    ];

    let active_sorts: Vec<_> = sort_flags.iter().filter(|(flag, _)| *flag).collect();

    match active_sorts.len() {
        0 => None,
        1 => Some(active_sorts[0].1.clone()),
        _ => {
            eprintln!(
                "{}",
                "Warning: Multiple sort flags specified. Using the first one found.".yellow()
            );
            Some(active_sorts[0].1.clone())
        }
    }
}

fn sort_files(files: &mut Vec<FileEntry>, sort_order: &SortOrder) {
    match sort_order {
        SortOrder::UpToDate => {
            files.sort_by(|a, b| b.raw_modified.cmp(&a.raw_modified));
        }
        SortOrder::DownToDate => {
            files.sort_by(|a, b| a.raw_modified.cmp(&b.raw_modified));
        }
        SortOrder::LargestSize => {
            files.sort_by(|a, b| b.raw_size.cmp(&a.raw_size));
        }
        SortOrder::SmallestSize => {
            files.sort_by(|a, b| a.raw_size.cmp(&b.raw_size));
        }
        SortOrder::AlphabeticalOrder => {
            files.sort_by(|a, b| {
                let name_a = a.name.trim_end_matches('/');
                let name_b = b.name.trim_end_matches('/');
                name_a.to_lowercase().cmp(&name_b.to_lowercase())
            });
        }
        SortOrder::AlphabeticalReverse => {
            files.sort_by(|a, b| {
                let name_a = a.name.trim_end_matches('/');
                let name_b = b.name.trim_end_matches('/');
                name_b.to_lowercase().cmp(&name_a.to_lowercase())
            });
        }
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

fn get_dir_size(path: &PathBuf) -> u64 {
    if let Ok(entries) = fs::read_dir(path) {
        entries
            .filter_map(Result::ok)
            .par_bridge()
            .map(|entry| {
                let meta = entry.metadata();
                if let Ok(m) = meta {
                    if m.is_dir() {
                        get_dir_size(&entry.path())
                    } else {
                        m.len()
                    }
                } else {
                    0
                }
            })
            .sum()
    } else {
        0
    }
}

fn format_datetime(
    system_time: std::time::SystemTime,
    time_format: &TimeFormat,
    timezone: &Tz,
    custom_format: &str,
    toggle_clock: bool,
) -> String {
    let datetime_utc: DateTime<Utc> = system_time.into();

    match time_format {
        TimeFormat::Utc => {
            if toggle_clock {
                format!("{}", datetime_utc.format("%a %b %e %Y %H:%M:%S UTC"))
            } else {
                format!("{}", datetime_utc.format("%a %b %e %Y"))
            }
        }
        TimeFormat::Unix => {
            if toggle_clock {
                format!(
                    "{} ({})",
                    datetime_utc.timestamp(),
                    datetime_utc.format("%a %b %e %Y %H:%M:%S UTC")
                )
            } else {
                datetime_utc.timestamp().to_string()
            }
        }
        TimeFormat::Iso8601 => {
            if toggle_clock {
                format!("{}", datetime_utc.format("%Y-%m-%dT%H:%M:%SZ"))
            } else {
                format!("{}", datetime_utc.format("%Y-%m-%d"))
            }
        }
        TimeFormat::Rfc3339 => {
            if toggle_clock {
                datetime_utc.to_rfc3339()
            } else {
                format!("{}", datetime_utc.format("%Y-%m-%d"))
            }
        }
        TimeFormat::Custom => {
            let tz_time = datetime_utc.with_timezone(timezone);
            format!("{}", tz_time.format(custom_format))
        }
        TimeFormat::Utf => {
            let tz_time = datetime_utc.with_timezone(timezone);
            if toggle_clock {
                format!("{}", tz_time.format("%a %b %e %Y %H:%M:%S %Z"))
            } else {
                format!("{}", tz_time.format("%a %b %e %Y"))
            }
        }
        TimeFormat::Local => {
            let tz_time = datetime_utc.with_timezone(timezone);
            if toggle_clock {
                format!("{}", tz_time.format("%a %b %e %Y %H:%M:%S %Z"))
            } else {
                format!("{}", tz_time.format("%a %b %e %Y"))
            }
        }
    }
}

fn print_table_from_files(files: &[FileEntry], pattern: &Option<String>, show_line_numbers: bool) {
    if files.is_empty() {
        if pattern.is_some() {
            println!("{}", "No files found matching the pattern.".red());
            exit(1);
        } else {
            println!("{}", "Directory is empty.".yellow());
        }
    } else {
        if show_line_numbers {
            let table_rows: Vec<TableRowWithLine> = files
                .iter()
                .map(|file| TableRowWithLine {
                    line_number: file.line_number.clone(),
                    name: file.name.clone(),
                    e_type: file.e_type.to_string(),
                    permissions: file.permissions.clone(),
                    size: file.size.clone(),
                    modified: file.modified.clone(),
                })
                .collect();
            print_styled_table(Table::new(&table_rows), true);
        } else {
            let table_rows: Vec<TableRowNoLine> = files
                .iter()
                .map(|file| TableRowNoLine {
                    name: file.name.clone(),
                    e_type: file.e_type.to_string(),
                    permissions: file.permissions.clone(),
                    size: file.size.clone(),
                    modified: file.modified.clone(),
                })
                .collect();
            print_styled_table(Table::new(&table_rows), false);
        }
    }
}

fn print_styled_table(mut table: Table, has_line_numbers: bool) {
    table.with(Style::rounded());
    let mut col_index = 0;
    if has_line_numbers {
        table.modify(Columns::one(col_index), Color::FG_BRIGHT_WHITE);
        col_index += 1;
    }
    table.modify(Columns::one(col_index), Color::FG_BRIGHT_CYAN); // Name
    col_index += 1;
    table.modify(Columns::one(col_index), Color::FG_BRIGHT_MAGENTA); // Type
    col_index += 1;
    table.modify(Columns::one(col_index), Color::FG_BRIGHT_MAGENTA); // Permissions
    col_index += 1;
    table.modify(Columns::one(col_index), Color::FG_BRIGHT_YELLOW); // Size
    col_index += 1;
    table.modify(Columns::one(col_index), Color::FG_BRIGHT_GREEN); // Modified Date

    table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
    println!("{}", table);
}

fn get_file(
    path: &PathBuf,
    pattern: &Option<String>,
    show_line_numbers: bool,
    octal_perms: bool,
    time_format: &TimeFormat,
    timezone: &Tz,
    custom_format: &str,
    deep: bool,
    toggle_clock: bool,
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
                        time_format,
                        timezone,
                        custom_format,
                        deep,
                        toggle_clock,
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
    time_format: &TimeFormat,
    timezone: &Tz,
    custom_format: &str,
    deep: bool,
    toggle_clock: bool,
) {
    if let Ok(meta) = fs::metadata(file.path()) {
        let file_size = if meta.is_dir() && deep {
            get_dir_size(&file.path())
        } else {
            meta.len()
        };

        let raw_modified = meta.modified().unwrap_or(std::time::UNIX_EPOCH);
        let modified_date = format_datetime(
            raw_modified,
            time_format,
            timezone,
            custom_format,
            toggle_clock,
        );

        let mut filename = file
            .file_name()
            .into_string()
            .unwrap_or("Unknown name.".into());
        if meta.is_dir() {
            filename.push('/');
        }

        data.push(FileEntry {
            line_number,
            name: filename,
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
            size: format_size(file_size),
            modified: modified_date,
            raw_size: file_size,
            raw_modified,
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
