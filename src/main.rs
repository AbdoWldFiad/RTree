use clap::Parser;
use std::fs;
use std::path::PathBuf;
use owo_colors::OwoColorize;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    path: PathBuf,

    #[arg(long)]
    depth: Option<usize>,

    #[arg(long)]
    only_dirs: bool,

    #[arg(long)]
    ignore: Option<String>,

    #[arg(long)]
    no_color: bool,

    /// Show file size
    #[arg(long)]
    size: bool,

    /// Show Unix-style permissions
    #[arg(long)]
    permissions: bool,
}

fn main() {
    let args = Args::parse();

    let use_color = !args.no_color;

    print_root(&args.path, use_color);
    print_tree(&args.path, "", &args, 0, use_color);
}

fn print_root(path: &PathBuf, use_color: bool) {
    let name = path.display().to_string();
    if use_color {
        println!("{}", name.green().bold());
    } else {
        println!("{}", name);
    }
}

fn print_tree(
    path: &PathBuf,
    prefix: &str,
    args: &Args,
    current_depth: usize,
    use_color: bool,
) {
    if let Some(max_depth) = args.depth {
        if current_depth >= max_depth {
            return;
        }
    }

    let mut entries: Vec<_> = fs::read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    entries.sort_by_key(|e| e.path());

    for (i, entry) in entries.iter().enumerate() {
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();

        if let Some(ignore) = &args.ignore {
            if name == *ignore {
                continue;
            }
        }

        if args.only_dirs && !path.is_dir() {
            continue;
        }

        let metadata = fs::symlink_metadata(&path).unwrap();

        let is_last = i == entries.len() - 1;
        let connector = if is_last { "└──" } else { "├──" };

        let connector = if use_color {
            connector.bright_black().to_string()
        } else {
            connector.to_string()
        };

        // Optional columns
        let mut columns = String::new();

        if args.permissions {
            let perms = format_permissions(&metadata);
            columns.push_str(&format!("{:<11} ", perms));
        }

        if args.size {
            let size = human_size(metadata.len());
            columns.push_str(&format!("{:>8} ", size));
        }

        let styled_name = style_entry(&path, &name, use_color);

        println!("{}{} {}{}", prefix, connector, columns, styled_name);

        if metadata.is_dir() {
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            print_tree(&path, &new_prefix.as_str(), args, current_depth + 1, use_color);
        }
    }
}
fn format_permissions(metadata: &fs::Metadata) -> String {
    #[cfg(unix)]
    {
        let mode = metadata.permissions().mode();
        let file_type = if metadata.is_dir() { 'd' } else { '-' };

        let mut perms = String::new();
        perms.push(file_type);

        for i in (0..3).rev() {
            let shift = i * 3;
            perms.push(if mode & (0o4 << shift) != 0 { 'r' } else { '-' });
            perms.push(if mode & (0o2 << shift) != 0 { 'w' } else { '-' });
            perms.push(if mode & (0o1 << shift) != 0 { 'x' } else { '-' });
        }

        return perms;
    }

    #[cfg(not(unix))]
    {
        // Windows fallback
        if metadata.permissions().readonly() {
            return String::from("r--r--r--");
        } else {
            return String::from("rw-rw-rw-");
        }
    }
}
fn human_size(size: u64) -> String {
    const UNITS: [&str; 5] = ["B", "K", "M", "G", "T"];
    let mut size = size as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    format!("{:.1}{}", size, UNITS[unit])
}

fn style_entry(path: &PathBuf, name: &str, use_color: bool) -> String {
    if !use_color {
        return name.to_string();
    }

    let metadata = fs::symlink_metadata(path).unwrap();

    if metadata.file_type().is_symlink() {
        return name.cyan().to_string();
    }

    if metadata.is_dir() {
        return name.blue().bold().to_string();
    }

    #[cfg(unix)]
    {
        if metadata.permissions().mode() & 0o111 != 0 {
            return name.green().to_string();
        }
    }

    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        match ext {
            "rs" => return name.bright_magenta().to_string(),
            "png" | "jpg" | "jpeg" | "gif" | "webp" => {
                return name.yellow().to_string()
            }
            _ => {}
        }
    }

    if name.starts_with('.') {
        return name.dimmed().to_string();
    }

    name.to_string()
}