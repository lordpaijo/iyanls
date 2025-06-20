use std::{fs, path::PathBuf};

use chrono_tz::Tz;
use rayon::prelude::*;

use crate::types::{EntryType, FileEntry, TimeFormat};
use crate::utils::{
    format_datetime, format_permissions_octal, format_permissions_owner_type,
    format_permissions_rwx, format_size,
};

pub fn get_files_from_directories(
    base_path: &PathBuf,
    include_dirs: &Option<Vec<String>>,
    exclude_dirs: &Option<Vec<String>>,
    pattern: &Option<String>,
    re_grab_pattern: &Option<String>,
    show_line_numbers: bool,
    octal_perms: bool,
    owner_type: bool,
    time_format: &TimeFormat,
    timezone: &Tz,
    custom_format: &str,
    deep: bool,
    toggle_clock: bool,
) -> Vec<FileEntry> {
    let mut all_entries = Vec::new();
    let mut directories_to_scan = Vec::new();
    directories_to_scan.push(base_path.clone());

    if let Some(include_list) = include_dirs {
        for dir_str in include_list {
            let dir_path = PathBuf::from(dir_str);
            if dir_path.exists() && dir_path.is_dir() {
                directories_to_scan.push(dir_path);
            } else {
                eprintln!(
                    "Warning: Directory '{}' does not exist or is not a directory",
                    dir_str
                );
            }
        }
    }

    if let Some(exclude_list) = exclude_dirs {
        directories_to_scan.retain(|path| {
            let path_str = path.to_string_lossy();
            !exclude_list
                .iter()
                .any(|exclude| path_str.contains(exclude) || path.ends_with(exclude))
        });
    }

    for dir_path in directories_to_scan {
        let mut dir_entries = get_file_from_single_directory(
            &dir_path,
            pattern,
            re_grab_pattern,
            show_line_numbers,
            octal_perms,
            owner_type,
            time_format,
            timezone,
            custom_format,
            deep,
            toggle_clock,
        );

        if include_dirs.is_some() && !include_dirs.as_ref().unwrap().is_empty() {
            for entry in &mut dir_entries {
                entry.name = format!("{}/{}", dir_path.display(), entry.name);
            }
        }

        all_entries.extend(dir_entries);
    }

    all_entries
}

pub fn get_file_from_single_directory(
    path: &PathBuf,
    pattern: &Option<String>,
    re_grab_pattern: &Option<String>,
    show_line_numbers: bool,
    octal_perms: bool,
    owner_type: bool,
    time_format: &TimeFormat,
    timezone: &Tz,
    custom_format: &str,
    deep: bool,
    toggle_clock: bool,
) -> Vec<FileEntry> {
    let mut data = Vec::new();
    if let Ok(read_dir) = fs::read_dir(path) {
        let mut line_number = 1;
        for entry in read_dir {
            if let Ok(file) = entry {
                if should_include_file(&file, pattern, re_grab_pattern) {
                    map_data(
                        file,
                        &mut data,
                        if show_line_numbers {
                            line_number.to_string()
                        } else {
                            String::new()
                        },
                        octal_perms,
                        owner_type,
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

pub fn get_file(
    path: &PathBuf,
    pattern: &Option<String>,
    re_grab_pattern: &Option<String>,
    include_dirs: &Option<Vec<String>>,
    exclude_dirs: &Option<Vec<String>>,
    show_line_numbers: bool,
    octal_perms: bool,
    owner_type: bool,
    time_format: &TimeFormat,
    timezone: &Tz,
    custom_format: &str,
    deep: bool,
    toggle_clock: bool,
) -> Vec<FileEntry> {
    let mut all_entries = Vec::new();
    let mut directories_to_scan = Vec::new();
    directories_to_scan.push(path.clone());

    if let Some(include_list) = include_dirs {
        for dir_str in include_list {
            let dir_path = PathBuf::from(dir_str);
            if dir_path.exists() && dir_path.is_dir() {
                directories_to_scan.push(dir_path);
            } else {
                eprintln!(
                    "Warning: Directory '{}' does not exist or is not a directory",
                    dir_str
                );
            }
        }
    }

    for dir_path in directories_to_scan {
        let mut dir_entries = scan_single_directory(
            &dir_path,
            pattern,
            re_grab_pattern,
            exclude_dirs,
            show_line_numbers,
            octal_perms,
            owner_type,
            time_format,
            timezone,
            custom_format,
            deep,
            toggle_clock,
        );

        let scanning_multiple = include_dirs.is_some() && include_dirs.as_ref().unwrap().len() > 0;

        if scanning_multiple {
            for entry in &mut dir_entries {
                if dir_path != *path {
                    entry.name = format!("{}/{}", dir_path.display(), entry.name);
                }
            }
        }

        all_entries.extend(dir_entries);
    }

    all_entries
}

fn scan_single_directory(
    path: &PathBuf,
    pattern: &Option<String>,
    re_grab_pattern: &Option<String>,
    exclude_patterns: &Option<Vec<String>>,
    show_line_numbers: bool,
    octal_perms: bool,
    owner_type: bool,
    time_format: &TimeFormat,
    timezone: &Tz,
    custom_format: &str,
    deep: bool,
    toggle_clock: bool,
) -> Vec<FileEntry> {
    let mut data = Vec::new();
    if let Ok(read_dir) = fs::read_dir(path) {
        let mut line_number = 1;
        for entry in read_dir {
            if let Ok(file) = entry {
                if should_include_file(&file, pattern, re_grab_pattern)
                    && !should_exclude_file(&file, exclude_patterns)
                {
                    map_data(
                        file,
                        &mut data,
                        if show_line_numbers {
                            line_number.to_string()
                        } else {
                            String::new()
                        },
                        octal_perms,
                        owner_type,
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

fn should_exclude_file(file: &fs::DirEntry, exclude_patterns: &Option<Vec<String>>) -> bool {
    if let Some(exclude_list) = exclude_patterns {
        let binding = file.file_name();
        let filename = binding.to_string_lossy();

        for pattern in exclude_list {
            if filename.contains(pattern) {
                return true;
            }
        }
    }
    false
}

fn map_data(
    file: fs::DirEntry,
    data: &mut Vec<FileEntry>,
    line_number: String,
    octal_perms: bool,
    owner_type: bool,
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
            .unwrap_or_else(|_| "Unknown name.".to_string());
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
            } else if owner_type {
                format_permissions_owner_type(&meta)
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

fn should_include_file(
    file: &fs::DirEntry,
    pattern: &Option<String>,
    re_grab_pattern: &Option<String>,
) -> bool {
    let filename = file.file_name().to_string_lossy().to_lowercase();

    if let Some(exclude_pattern) = re_grab_pattern {
        if filename.contains(&exclude_pattern.to_lowercase()) {
            return false;
        }
    }

    let Some(search_pattern) = pattern else {
        return true;
    };

    filename.contains(&search_pattern.to_lowercase())
}

pub fn get_dir_size(path: &PathBuf) -> u64 {
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
