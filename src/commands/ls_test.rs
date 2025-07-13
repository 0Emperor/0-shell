use chrono::{DateTime, Local};
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use terminal_size::{terminal_size, Width};
use users::{get_group_by_gid, get_user_by_uid};
use xattr;

struct FileInfo {
    permissions: String,
    links: u64,
    user: String,
    group: String,
    size_or_device: SizeOrDevice,
    modified_time: String,
    name: String,
}

enum SizeOrDevice {
    Size(u64),
    Device { major: u64, minor: u64 },
}

pub fn ls(args: Vec<String>) -> io::Result<()> {
    let (directories, show_hidden, long_format, classify) = match filter_flags(args.clone()) {
        Some(result) => result,
        None => {
            println!("ls: invalid flag");
            return Ok(());
        }
    };

    let mut effective_dirs = directories.clone();
    if effective_dirs.is_empty() {
        effective_dirs.push(".".to_string());
    }

    let mut output_sections = Vec::new();
    let mut error_messages = Vec::new();

    for (i, dir_path) in effective_dirs.iter().enumerate() {
        if effective_dirs.len() > 1 {
            if i > 0 {
                output_sections.push(String::new());
            }
            output_sections.push(format!("{}:", dir_path));
        }

        match fs::read_dir(dir_path) {
            Ok(entries) => {
                let mut paths: Vec<PathBuf> =
                    entries.filter_map(Result::ok).map(|e| e.path()).collect();

       paths.sort_by(|a, b| {
    let a_name = a.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase())
        .unwrap_or_default();
    let b_name = b.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase())
        .unwrap_or_default();
    
    a_name.cmp(&b_name).then_with(|| {
        a.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_lowercase()
            .cmp(
                &b.file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default()
                    .to_lowercase()
            )
    })
});

                let mut file_infos = Vec::new();
                if show_hidden {
                    if long_format {
                        if let Ok(info) = get_file_info(&Path::new(dir_path).join("."), classify) {
                            file_infos.push(info);
                        }
                        if let Ok(info) = get_file_info(&Path::new(dir_path).join(".."), classify) {
                            file_infos.push(info);
                        }
                    }
                }

                for path in &paths {
                    if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                        if !show_hidden && filename.starts_with('.') {
                            continue;
                        }
                        if let Ok(info) = get_file_info(path, classify) {
                            file_infos.push(info);
                        }
                    }
                }

                if long_format {
                    let total_blocks: u64 = file_infos
                        .iter()
                        .filter_map(|fi| {
                            let first_part = fi.name.split(' ').next().unwrap_or("");
                            fs::symlink_metadata(Path::new(dir_path).join(first_part)).ok()
                        })
                        .map(|m| m.blocks())
                        .sum();

                    output_sections.push(format!("total {}", total_blocks / 2));
                    output_sections.push(format_long_columns(file_infos));
                } else {
                    let names = file_infos.into_iter().map(|fi| fi.name).collect();
                    output_sections.push(format_columns(names));
                }
            }
            Err(e) => {
                error_messages.push(format!("ls: cannot access '{}': {}", dir_path, e.kind()));
            }
        }
    }

    if !error_messages.is_empty() {
        println!("{}", error_messages.join("\n"));
    }
    if !output_sections.is_empty() {
        println!("{}", output_sections.join("\n"));
    }

    Ok(())
}

fn get_file_info(path: &Path, classify: bool) -> io::Result<FileInfo> {
    let meta = fs::symlink_metadata(path)?;
    let file_type = meta.file_type();

    let name_os_str = path.file_name();
    let mut name = name_os_str.and_then(|s| s.to_str()).unwrap_or("").to_string();

    if path.to_str() == Some(".") { name = ".".to_string(); }
    if path.to_str() == Some("..") { name = "..".to_string(); }


    if classify {
        name.push_str(&classify_suffix(&file_type, &meta));
    }

    if file_type.is_symlink() {
        if let Ok(target) = fs::read_link(path) {
            name.push_str(&format!(" -> {}", target.display()));
        }
    }

    let size_or_device = if file_type.is_block_device() || file_type.is_char_device() {
        SizeOrDevice::Device {
            major: major(meta.rdev()),
            minor: minor(meta.rdev()),
        }
    } else {
        SizeOrDevice::Size(meta.len())
    };

    let modified: DateTime<Local> = DateTime::from(meta.modified()?);

    Ok(FileInfo {
        permissions: mode_string(&meta, path),
        links: meta.nlink(),
        user: get_user_by_uid(meta.uid())
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| meta.uid().to_string()),
        group: get_group_by_gid(meta.gid())
            .map(|g| g.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| meta.gid().to_string()),
        size_or_device,
        modified_time: modified.format("%b %e %H:%M").to_string(),
        name,
    })
}

fn format_long_columns(infos: Vec<FileInfo>) -> String {
    if infos.is_empty() {
        return String::new();
    }

    let mut max_links_width = 0;
    let mut max_user_width = 0;
    let mut max_group_width = 0;
    let mut max_major_width = 0;
    let mut max_minor_width = 0;
    let mut max_size_width = 0;

    for info in &infos {
        max_links_width = max_links_width.max(info.links.to_string().len());
        max_user_width = max_user_width.max(info.user.len());
        max_group_width = max_group_width.max(info.group.len());

        match info.size_or_device {
            SizeOrDevice::Size(size) => {
                max_size_width = max_size_width.max(size.to_string().len());
            }
            SizeOrDevice::Device { major, minor } => {
                max_major_width = max_major_width.max(major.to_string().len());
                max_minor_width = max_minor_width.max(minor.to_string().len());
            }
        }
    }

    let dev_width = max_major_width + 2 + max_minor_width;
    let size_col_width = max_size_width.max(dev_width);

    let mut output = String::new();
    for info in infos {
        output.push_str(&format!(
            "{} {:>links_w$} {:<user_w$} {:<group_w$} ",
            info.permissions,
            info.links,
            info.user,
            info.group,
            links_w = max_links_width,
            user_w = max_user_width,
            group_w = max_group_width
        ));

        let size_str = match info.size_or_device {
            SizeOrDevice::Size(size) => format!("{:>width$}", size, width = size_col_width),
            SizeOrDevice::Device { major, minor } => {
                let dev_str = format!("{},", major);
                let combined = format!(
                    "{:>maj_w$} {:>min_w$}",
                    dev_str,
                    minor,
                    maj_w = max_major_width + 1,
                    min_w = max_minor_width
                );
                format!("{:>width$}", combined, width = size_col_width)
            }
        };
        output.push_str(&size_str);

        output.push_str(&format!(" {} {}\n", info.modified_time, info.name));
    }

    output.trim_end().to_string()
}

fn mode_string(meta: &std::fs::Metadata, path: &Path) -> String {
    let mode = meta.mode();
    let file_type = meta.file_type();
    let file_type_char = if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
        'l'
    } else if file_type.is_fifo() {
        'p'
    } else if file_type.is_socket() {
        's'
    } else if file_type.is_block_device() {
        'b'
    } else if file_type.is_char_device() {
        'c'
    } else {
        '-'
    };

    let usr = (mode >> 6) & 0b111;
    let grp = (mode >> 3) & 0b111;
    let oth = mode & 0b111;

    let suid = (mode & 0o4000) != 0;
    let sgid = (mode & 0o2000) != 0;
    let sticky = (mode & 0o1000) != 0;

    let ur = if usr & 0b100 != 0 { 'r' } else { '-' };
    let uw = if usr & 0b010 != 0 { 'w' } else { '-' };
    let ux = match (usr & 0b001 != 0, suid) {
        (true, true) => 's',
        (false, true) => 'S',
        (true, false) => 'x',
        (false, false) => '-',
    };

    let gr = if grp & 0b100 != 0 { 'r' } else { '-' };
    let gw = if grp & 0b010 != 0 { 'w' } else { '-' };
    let gx = match (grp & 0b001 != 0, sgid) {
        (true, true) => 's',
        (false, true) => 'S',
        (true, false) => 'x',
        (false, false) => '-',
    };

    let or = if oth & 0b100 != 0 { 'r' } else { '-' };
    let ow = if oth & 0b010 != 0 { 'w' } else { '-' };
    let ox = match (oth & 0b001 != 0, sticky) {
        (true, true) => 't',
        (false, true) => 'T',
        (true, false) => 'x',
        (false, false) => '-',
    };

    let base_mode = format!(
        "{}{}{}{}{}{}{}{}{}{}",
        file_type_char, ur, uw, ux, gr, gw, gx, or, ow, ox
    );

    let has_acl = xattr::get(path, "system.posix_acl_access").is_ok_and(|v| v.is_some());

    let acl_char = if has_acl { "+" } else { " " };

    format!("{}{}", base_mode, acl_char)
}

fn classify_suffix(file_type: &std::fs::FileType, meta: &std::fs::Metadata) -> String {
    if file_type.is_dir() {
        "/".to_string()
    } else if file_type.is_symlink() {
        "".to_string()
    } else if file_type.is_fifo() {
        "|".to_string()
    } else if file_type.is_socket() {
        "=".to_string()
    } else if meta.permissions().mode() & 0o111 != 0 {
        "*".to_string()
    } else {
        "".to_string()
    }
}

pub fn filter_flags(args: Vec<String>) -> Option<(Vec<String>, bool, bool, bool)> {
    let mut directories = vec![];
    let mut show_hidden = false;
    let mut long_format = false;
    let mut classify = false;

    for arg in args {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => show_hidden = true,
                    'l' => long_format = true,
                    'F' => classify = true,
                    _ => return None,
                }
            }
        } else {
            directories.push(arg);
        }
    }
    Some((directories, show_hidden, long_format, classify))
}

fn major(dev: u64) -> u64 {
    (dev >> 8) & 0xfff
}
fn minor(dev: u64) -> u64 {
    (dev & 0xff) | ((dev >> 12) & 0xfff00)
}

fn format_columns(items: Vec<String>) -> String {
    if items.is_empty() {
        return String::new();
    }
    let term_width = terminal_size().map_or(80, |(Width(w), _)| w as usize);
    let n_items = items.len();

    let mut best_cols = 1;
    for cols in (1..=n_items).rev() {
        let rows = (n_items + cols - 1) / cols;
        let mut col_widths = vec![0; cols];
        let mut total_width = 0;
        let mut possible = true;
        for col in 0..cols {
            for row in 0..rows {
                let i = col * rows + row;
                if i < n_items {
                    col_widths[col] = col_widths[col].max(items[i].len());
                }
            }
            total_width += col_widths[col];
            if col > 0 {
                total_width += 2;
            }
            if total_width > term_width {
                possible = false;
                break;
            }
        }
        if possible {
            best_cols = cols;
            break;
        }
    }

    let rows = (n_items + best_cols - 1) / best_cols;
    let mut col_widths = vec![0; best_cols];
    for col in 0..best_cols {
        for row in 0..rows {
            let i = col * rows + row;
            if i < n_items {
                col_widths[col] = col_widths[col].max(items[i].len());
            }
        }
    }

    let mut output = String::new();
    for row in 0..rows {
        for col in 0..best_cols {
            let i = col * rows + row;
            if i < n_items {
                let s = &items[i];
                output.push_str(s);
                if col < best_cols - 1 {
                    let padding = col_widths[col] - s.len();
                    output.push_str(&" ".repeat(padding + 2));
                }
            }
        }
        output.push('\n');
    }
    output.trim_end().to_string()
}