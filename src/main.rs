use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use owo_colors::OwoColorize;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to inspect
    path: PathBuf,

    /// Limit recursion depth
    #[arg(short = 'd', long)]
    depth: Option<usize>,

    /// Show only directories
    #[arg(long)]
    only_dirs: bool,

    /// Ignore specific files or directories (can be used multiple times)
    #[arg(long)]
    ignore: Vec<String>,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,

    /// Show file sizes
    #[arg(short, long)]
    size: bool,

    /// Show Unix-style permissions
    #[arg(short, long)]
    permissions: bool,
}

fn main() {
    let args = Args::parse();

    let use_color = should_use_color(&args);

    print_root(&args.path, use_color);
    print_tree(&args.path, "", &args, 0, use_color);
}

fn should_use_color(args: &Args) -> bool {
    if args.no_color {
        return false;
    }

    use std::io::IsTerminal;
    std::io::stdout().is_terminal()
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

    let mut entries: Vec<_> = match fs::read_dir(path) {
        Ok(e) => e.filter_map(Result::ok).collect(),
        Err(_) => return,
    };

    entries.sort_by_key(|e| e.path());

    for (i, entry) in entries.iter().enumerate() {
        let path = entry.path();
        let name = match path.file_name() {
            Some(n) => n.to_string_lossy(),
            _none => continue,
        };

        if args.ignore.iter().any(|ig| ig == &name) {
            continue;
        }

        if args.only_dirs && !path.is_dir() {
            continue;
        }

        let metadata = match fs::symlink_metadata(&path) {
            Ok(m) => m,
            Err(_) => continue,
        };

        let is_last = i == entries.len() - 1;
        let connector_raw = if is_last { "└──" } else { "├──" };

        let connector = if use_color {
            connector_raw.bright_black().to_string()
        } else {
            connector_raw.to_string()
        };

        let mut columns = String::new();

        if args.permissions {
            let perms = format_permissions(&metadata);
            columns.push_str(&format!("{:<11} ", perms));
        }

        if args.size {
            let size = if metadata.is_dir() {
                human_size(dir_size(&path))
            } else {
                human_size(metadata.len())
            };
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

fn dir_size(path: &Path) -> u64 {
    let mut total = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if let Ok(metadata) = fs::symlink_metadata(&path) {
                if metadata.is_dir() {
                    total += dir_size(&path);
                } else {
                    total += metadata.len();
                }
            }
        }
    }

    total
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

        perms
    }

    #[cfg(not(unix))]
    {
        if metadata.permissions().readonly() {
            String::from("r--r--r--")
        } else {
            String::from("rw-rw-rw-")
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

    let metadata = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => return name.to_string(),
    };

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