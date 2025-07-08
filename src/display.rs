use std::io::{self, BufWriter, Write};

use std::{fs, path::PathBuf, process::exit};

use owo_colors::OwoColorize;
use tabled::{
    Table,
    settings::{
        Color, Style,
        object::{Columns, Rows},
    },
};

use crate::types::{FileEntry, TableRowNoLine, TableRowWithLine};

pub fn export_json(
    files: &[FileEntry],
    export_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let json_content = serde_json::to_string_pretty(files)?;
    fs::write(export_path, json_content)?;
    Ok(())
}

pub fn print_table_from_files(
    files: &[FileEntry],
    pattern: &Option<String>,
    show_line_numbers: bool,
) {
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
    let stdout = io::stdout();
    let mut w = BufWriter::new(stdout.lock());
    writeln!(w, "{}", table).unwrap();
    w.flush().unwrap();
}
