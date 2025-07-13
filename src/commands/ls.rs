use chrono::{DateTime, Local};
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::{fs, io};
use terminal_size::{terminal_size, Width};
use users::{get_group_by_gid, get_user_by_uid};

pub fn ls(dirs: Vec<String>) -> io::Result<()> {
    let mut rs = vec![];
    let mut ers = vec![];
    let (mut arguments, flags) = filter_flags(dirs.clone());
    let f = flags[0];
    let a = flags[1];
    let l = flags[2];

    if arguments.is_empty() {
        arguments.push(".".to_string());
    }

    for dir in &arguments {
        match fs::read_dir(&dir) {
            Ok(entries) => {
                let mut paths = entries
                    .map(|res| res.map(|e| e.path()))
                    .collect::<Result<Vec<_>, io::Error>>()?;
               paths.sort_by(|a, b| {
    a.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .cmp(
            &b.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_ascii_lowercase(),
        )
});
                let total_blocks: u64 = paths
                    .iter()
                    .filter(|entry| {
                        if let Some(name) = entry.file_name().and_then(|s| s.to_str()) {
                            a || !name.starts_with('.')
                        } else {
                            false
                        }
                    })
                    .filter_map(|entry| fs::symlink_metadata(entry).ok())
                    .map(|meta| meta.blocks() / 2)
                    .sum();
                if l && arguments.len() == 1 {
                    println!("total {}", total_blocks);
                }
                let mut files: Vec<String> = vec![];
                for entry in paths {
                    if let Some(filename) = entry.file_name() {
                        let name = filename.to_string_lossy();
                        if !a && name.starts_with('.') {
                            continue;
                        }

                        let mut display = if f {
                            let meta = fs::symlink_metadata(&entry)?;
                            let file_type = meta.file_type();
                            format!("{}{}", name, classify_suffix(&file_type, &meta, l))
                        } else {
                            name.to_string()
                        };
                        if l {
                            let meta = fs::symlink_metadata(&entry)?;
                            let mode = mode_string(&meta);
                            let nlink = meta.nlink();
                            let uid = meta.uid();
                            let gid = meta.gid();
                            let size = meta.len();
                            let modified: DateTime<Local> = DateTime::from(meta.modified()?);
                            let time_str = modified.format("%b %e %H:%M").to_string();
                            let user = get_user_by_uid(uid)
                                .map(|u| u.name().to_string_lossy().to_string())
                                .unwrap_or(uid.to_string());
                            let group = get_group_by_gid(gid)
                                .map(|g| g.name().to_string_lossy().to_string())
                                .unwrap_or(gid.to_string());
                                if meta.file_type().is_symlink() {
                                    if let Ok(target) = fs::read_link(&entry) {
                                        display.push_str(&format!(" -> {}",target.display()));
                                    }
                                }
                            display = format!(
                                "{} {} {} {} {} {} {}",
                                mode, nlink, user, group, size, time_str, display
                            );
                        }
                        files.push(display);
                    }
                }
                if a {
                    let mut cur = ".".to_string();
                    let mut par = "..".to_string();
                    if f {
                        cur.push('/');
                        par.push('/');
                    }
                    if l {
                        par = getl(par);
                        cur = getl(cur);
                    }
                    files.insert(0, par);
                    files.insert(0, cur);
                }
                if files.is_empty() && arguments.len()!= 1 {
                    rs.push(format!("{}:", dir));
                    continue;
                }
                let mut output = String::new();
                if arguments.len() != 1 {
                    if l {
                        output.push_str(&format!("{}:\ntotal: {}\n", dir, total_blocks));
                    } else {
                    }
                }
                if l {
                    let mut v = splitforl(files);

                    output.push_str(&formatls(&mut v).trim_start());
                } else {
                    output.push_str(&format_columns(files).trim_start());
                }
                rs.push(output.trim().to_string());
            }
            Err(e) => {
                let msg = e.to_string();
                let trimmed = msg
                    .split_once('(')
                    .map(|(before, _)| before.trim())
                    .unwrap_or(&msg);
                ers.push(format!("ls: cannot access '{}': {}", dir, trimmed));
            }
        }
    }

    if dirs.len() == 1 {
        if !ers.is_empty() {
            println!("{}", ers[0]);
        } else if !rs.is_empty() {
            println!("{}", rs[0]);
        }
    } else {
        if !ers.is_empty() {
            println!("{}\n", ers.join("\n"));
        }
        if !rs.is_empty() && rs[0] != ""{
            print!("{}\n", rs.join("\n\n"));
        }
    }
    Ok(())
}

fn formatls(v: &mut Vec<Vec<String>>) -> String {
    let maxwidths = maxwidths(v);
    let lines: Vec<String> = v
        .into_iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .map(|(i, word)| {
                    if i == 4 || i == 1 {
                        format!("{:>width$}", word, width = maxwidths[i])
                    } else {
                        if i < 9 {
                            format!("{:<width$}", word, width = maxwidths[i])
                        } else {
                            format!("{:<width$}", word, width = 1)
                        }
                    }
                })
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect();
    lines.join("\n")
}

fn maxwidths(v: &mut Vec<Vec<String>>) -> Vec<usize> {
    let mut r = vec![0; 9];
    for line in v {
        for (i, col) in line.clone().iter().enumerate() {
            if i == 9 {
                if let Some(extra) = line.pop() {
                    line[8].push_str(&format!(" {}", extra));
                }
            }
            if i < r.len() {
                r[i] = r[i].max(col.len());
            }
        }
    }
    r
}

fn format_columns(items: Vec<String>) -> String {
    if items.is_empty() {
        return String::new();
    }
    if items[0].ends_with("\n") {
        return items.join("");
    }
    let term_width = terminal_size().map_or(80, |(Width(w), _)| w as usize);
    let n_items = items.len();

    let mut cols = 1;
    let mut rows = n_items;
    for c in (1..=n_items).rev() {
        let r = (n_items + c - 1) / c;
        let mut col_widths = vec![0; c];
        for col in 0..c {
            for row in 0..r {
                let i = row + col * r;
                if i < n_items {
                    col_widths[col] = col_widths[col].max(items[i].len());
                }
            }
        }
        let total_width: usize = col_widths.iter().sum::<usize>() + 2 * (c - 1);
        if total_width <= term_width {
            cols = c;
            rows = r;
            break;
        }
    }

    let mut col_widths = vec![0; cols];
    for col in 0..cols {
        for row in 0..rows {
            let i = row + col * rows;
            if i < n_items {
                col_widths[col] = col_widths[col].max(items[i].len());
            }
        }
    }

    let mut output = String::new();
    for row in 0..rows {
        for col in 0..cols {
            let i = row + col * rows;
            if i < n_items {
                let s = &items[i];
                let padding = col_widths[col].saturating_sub(s.len());
                output.push_str(s);
                if col < cols - 1 {
                    output.push_str(&" ".repeat(padding + 2));
                }
            }
        }
        output.push('\n');
    }

    output.trim_end().to_string()
}

pub fn filter_flags(dirs: Vec<String>) -> (Vec<String>, Vec<bool>) {
    let mut args = vec![];
    let mut flags = vec![false; 3];
    for arg in dirs {
        if !arg.starts_with('-') {
            args.push(arg);
        } else {
            if arg.contains('F') {
                flags[0] = true;
            }
            if arg.contains('a') {
                flags[1] = true;
            }
            if arg.contains('l') {
                flags[2] = true;
            }
        }
    }
    (args, flags)
}

fn mode_string(meta: &std::fs::Metadata) -> String {
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

    format!("{}{}{}{}{}{}{}{}{}{}", file_type_char, ur, uw, ux, gr, gw, gx, or, ow, ox)
}

fn classify_suffix(file_type: &std::fs::FileType, meta: &std::fs::Metadata, l: bool) -> String {
    if file_type.is_dir() {
        "/".to_string()
    } else if file_type.is_symlink() {
        if l {}
        "@".to_string()
    } else if file_type.is_fifo() {
        "|".to_string()
    } else if file_type.is_socket() {
        "=".to_string()
    } else if file_type.is_file() && meta.permissions().mode() & 0o111 != 0 {
        "*".to_string()
    } else {
        "".to_string()
    }
}

fn getl(s: String) -> String {
    let meta = fs::symlink_metadata(&s).unwrap();
    let mode = mode_string(&meta);
    let nlink = meta.nlink();
    let uid = meta.uid();
    let gid = meta.gid();
    let size = meta.len();
    let modified: DateTime<Local> = DateTime::from(meta.modified().unwrap());
    let time_str = modified.format("%b %e %H:%M").to_string();
    let user = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().to_string())
        .unwrap_or(uid.to_string());
    let group = get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().to_string())
        .unwrap_or(gid.to_string());
    format!(
        "{} {} {} {} {} {} {}",
        mode, nlink, user, group, size, time_str, s
    )
    .to_string()
}
fn splitforl(files: Vec<String>)-> Vec<Vec<String>>{
    let mut r = vec![];
    for file in files{
        let mut chob = vec![];
        let mut st = String::new();
        for (i,c) in file.chars().enumerate(){
            if chob.len() == 8{
                chob.push(file[i..].to_string());
                break
            }
            if c == ' ' {
                chob.push(st);
                st = String::new();
            }else{
                st.push(c);
            }
        }
        r.push(chob)
    }
    r
 } 