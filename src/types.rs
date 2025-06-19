use chrono_tz::Tz;
use serde::Serialize;
use strum::Display;
use tabled::Tabled;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum TimeFormat {
    Utc,
    Local,
    Unix,
    Iso8601,
    Rfc3339,
    Utf,
    Custom,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum SortOrder {
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
    #[value(name = "directory-first")]
    DirFirst,
    #[value(name = "directory-last")]
    DirLast,
}

#[derive(Debug, Display, Serialize, Clone)]
pub enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Serialize, Clone)]
pub struct FileEntry {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub line_number: String,
    pub name: String,
    pub e_type: EntryType,
    pub permissions: String,
    pub size: String,
    pub modified: String,
    #[serde(skip)]
    pub raw_size: u64,
    #[serde(skip)]
    pub raw_modified: std::time::SystemTime,
}

pub struct FileMetadata {
    pub size: u64,
    pub modified: std::time::SystemTime,
    pub is_dir: bool,
    pub metadata: std::fs::Metadata,
}

#[derive(Debug, Tabled)]
pub struct TableRowWithLine {
    #[tabled(rename = "#")]
    pub line_number: String,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Type")]
    pub e_type: String,
    #[tabled(rename = "Permissions")]
    pub permissions: String,
    #[tabled(rename = "Size")]
    pub size: String,
    #[tabled(rename = "Modified Date")]
    pub modified: String,
}

#[derive(Debug, Tabled)]
pub struct TableRowNoLine {
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Type")]
    pub e_type: String,
    #[tabled(rename = "Permissions")]
    pub permissions: String,
    #[tabled(rename = "Size")]
    pub size: String,
    #[tabled(rename = "Modified Date")]
    pub modified: String,
}
