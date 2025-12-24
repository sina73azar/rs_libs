use std::{io, path::PathBuf};

fn main() -> io::Result<()> {
    let root = PathBuf::from(".");
    let dry_run = true;

    let cfg = Config::new(root, dry_run);
    run(cfg)?;

    Ok(())
}

struct Config {
    root: PathBuf,
    dry_run: bool,
}

impl Config {
    fn new(root: PathBuf, dry_run: bool) -> Self {
        Self { root, dry_run }
    }
}

fn run(cfg: Config) -> io::Result<()> {
    println!("root={:?}, dry_run={}", cfg.root, cfg.dry_run);
    Ok(())
}