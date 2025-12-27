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

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--dry-run" => dry_run = true,
                "--apply" => dry_run = false,
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

        Ok(Self { root, dry_run })
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
    println!("Scanning {:?} (dry_run={})", cfg.root, cfg.dry_run);
    visit_dir(&cfg.root, cfg.dry_run)
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
