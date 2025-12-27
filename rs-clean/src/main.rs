use std::{
    env,
    fs,
    io,
    path::{Path, PathBuf},
};

fn main() -> io::Result<()> {
    let cfg = Config::from_args(env::args())?;
    run(cfg)
}

struct Config {
    root: PathBuf,
    dry_run: bool,
    force: bool,
}

impl Config {
    /// Parse CLI args:
    ///   rs-clean [--root <path>] [--dry-run | --apply]
    ///
    /// Defaults:
    ///   root = "."
    ///   dry_run = true
    fn from_args(mut args: env::Args) -> io::Result<Self> {
        // skip program name
        let _ = args.next();

        let mut root = PathBuf::from(".");
        let mut dry_run = true;
        let mut force = false;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--dry-run" => dry_run = true,
                "--apply" => dry_run = false,
                "--force" => force = true,
                "--root" => {
                    let Some(p) = args.next() else {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "missing value after --root",
                        ));
                    };
                    root = PathBuf::from(p);
                }
                "--help" | "-h" => {
                    print_help();
                    // Exit successfully without doing work
                    std::process::exit(0);
                }
                unknown => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("unknown argument: {unknown} (try --help)"),
                    ));
                }
            }
        }

        Ok(Self {
            root,
            dry_run,
            force,
        })
    }
}

fn print_help() {
    println!(
        "\
rs-clean - delete Rust target/ directories safely

USAGE:
  rs-clean [--root <path>] [--dry-run | --apply]

OPTIONS:
  --root <path>   Root directory to scan (default: .)
  --dry-run       Show what would be deleted (default)
  --apply         Actually delete matched target/ directories
  -h, --help      Print help
"
    );
}

fn run(cfg: Config) -> io::Result<()> {
    let root_canon = cfg.root.canonicalize().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid --root {:?}: {e}", cfg.root),
        )
    })?;

    if !cfg.dry_run {
        enforce_safety_guard(&root_canon, cfg.force)?;
    }

    println!(
        "Scanning {:?} (dry_run={}, force={})",
        root_canon, cfg.dry_run, cfg.force
    );
    visit_dir(&cfg.root, cfg.dry_run)
}

fn enforce_safety_guard(root: &Path, force: bool) -> io::Result<()> {
    let is_root = root == Path::new("/");

    // HOME is the most common catastrophic target (lots of projects under it).
    // Refuse deleting under HOME unless --force is provided.
    let home = env::var_os("HOME").map(PathBuf::from);
    let is_home = home
        .as_ref()
        .and_then(|h| h.canonicalize().ok())
        .map(|h| h == root)
        .unwrap_or(false);

    if (is_root || is_home) && !force {
        let what = if is_root { "/" } else { "$HOME" };
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!(
                "refusing to run with --apply on dangerous root ({what}). Re-run with --force if you are sure."
            ),
        ));
    }

    Ok(())
}

fn visit_dir(path: &Path, dry_run: bool) -> io::Result<()> {
    if !path.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            if entry_path.file_name() == Some("target".as_ref()) {
                if dry_run {
                    println!("[dry-run] would delete {:?}", entry_path);
                } else {
                    println!("deleting {:?}", entry_path);
                    fs::remove_dir_all(&entry_path)?;
                }
            } else {
                visit_dir(&entry_path, dry_run)?;
            }
        }
    }

    Ok(())
}
