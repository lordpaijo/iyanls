use crate::cli::Args;
use crate::types::{FileEntry, SortOrder};
use owo_colors::OwoColorize;

pub fn get_sort_order(args: &Args) -> Option<SortOrder> {
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

pub fn sort_files(files: &mut Vec<FileEntry>, sort_order: &SortOrder) {
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
