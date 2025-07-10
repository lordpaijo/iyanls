use clap::Parser;
use std::path::PathBuf;

use crate::types::TimeFormat;

#[derive(Debug, Parser)]
#[command(
    author = "LordPaijo (Iyan)",
    version,
    about,
    long_about = "Iyan's ls tool."
)]
pub struct Args {
    pub path: Option<PathBuf>,
    #[arg(short, long, help = "Toggle deep processing", default_value = "false")]
    pub deep: bool,
    #[arg(
        short,
        long,
        help = "Filter files by pattern (matches filenames containing the pattern)"
    )]
    pub grab: Option<String>,
    #[arg(short, long, help = "Format output to JSON")]
    pub json: bool,
    #[arg(alias = "jsx", long, help = "Export JSON output to a file")]
    pub json_export: Option<PathBuf>,
    #[arg(short = 'n', long, help = "Hide line numbers")]
    pub no_line_numbers: bool,
    #[arg(short = 'o', long, help = "Show permissions in octal format")]
    pub octal_perms: bool,
    #[arg(short = 'u', long, help = "Show permissions in owner format")]
    pub owner_perms: bool,
    #[arg(
        short = 'a',
        long,
        help = "Show current directory metadata",
        default_value = "false"
    )]
    pub show_cwd: bool,
    #[arg(short, long, help = "Include files or directories")]
    pub include: Option<Vec<String>>,
    #[arg(short = 'x', long, help = "Exclude files or directories")]
    pub exclude: Option<Vec<String>>,
    #[arg(
        short = 't',
        long,
        value_enum,
        default_value = "local",
        help = "Time format for dates (utc, local, unix, iso8601, rfc3339, utf, custom)"
    )]
    pub time_format: TimeFormat,
    #[arg(
        long,
        help = "Custom time format string (used with --time-format=custom)",
        default_value = "%Y-%m-%d %H:%M:%S %Z"
    )]
    pub custom_time_format: String,
    #[arg(
        long,
        help = "Timezone for time display (e.g., US/Eastern, Europe/London, Asia/Tokyo)",
        default_value = "UTC"
    )]
    pub timezone: String,
    #[arg(long, help = "Toggle clock display", default_value = "false")]
    pub toggle_clock: bool,
    #[arg(short = 'U', long, help = "Sort files by newest modified to oldest")]
    pub up_to_date: bool,
    #[arg(short = 'D', long, help = "Sort files by oldest modified to newest")]
    pub down_to_date: bool,
    #[arg(short = 'X', long, help = "Sort files by largest size")]
    pub largest_size: bool,
    #[arg(short = 'S', long, help = "Sort files by smallest size")]
    pub smallest_size: bool,
    #[arg(short = 'A', long, help = "Sort files by alphabetical order")]
    pub alphabetical_order: bool,
    #[arg(short = 'B', long, help = "Sort files by reversed alphabetical order")]
    pub alphabetical_reverse: bool,
    #[arg(short = 'C', long, help = "Sort files by directory first")]
    pub dir_first: bool,
    #[arg(short = 'L', long, help = "Sort files by directory last")]
    pub dir_last: bool,
}
