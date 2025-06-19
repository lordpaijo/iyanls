use std::fs;

use chrono::{DateTime, Utc};
use chrono_tz::{Tz, UTC};
use owo_colors::OwoColorize;

use crate::types::TimeFormat;

pub fn parse_timezone(timezone_str: &str) -> Tz {
    match timezone_str.parse::<Tz>() {
        Ok(tz) => tz,
        Err(_) => {
            eprintln!(
                "{}: '{}'. Using UTC instead.",
                "Invalid timezone".red(),
                timezone_str
            );
            UTC
        }
    }
}

pub fn format_size(size: u64) -> String {
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

pub fn format_datetime(
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

#[cfg(unix)]
pub fn format_permissions_rwx(metadata: &fs::Metadata) -> String {
    use std::os::unix::fs::PermissionsExt;

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
pub fn format_permissions_octal(metadata: &fs::Metadata) -> String {
    use std::os::unix::fs::PermissionsExt;

    let mode = metadata.permissions().mode();
    let permissions = mode & 0o777;
    format!("{:03o}", permissions)
}
