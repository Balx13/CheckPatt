//! `cargo devrun [cargo build options] [-- application arguments]`
//!
//! Builds the application like `cargo run` would, but then copies the built
//! executable into the directory configured in `xtask.toml` (default: `app-test`)
//! and starts it from there, with that directory as the working directory.

use std::{
    env, fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{exit, Command, Stdio},
};

/// Contents of `xtask.toml` (in the project root).
#[derive(serde::Deserialize)]
#[serde(default, deny_unknown_fields)] // unknown keys (typos) are reported as errors
struct Config {
    /// Where the executable is copied to and run from.
    #[serde(rename = "dest-dir")]
    dest_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dest_dir: PathBuf::from("app-test"),
        }
    }
}

fn load_config(root: &Path) -> Result<Config, String> {
    let path = root.join("xtask.toml");
    match fs::read_to_string(&path) {
        Ok(text) => toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display())),
        // No config file: use the defaults.
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
        Err(e) => Err(format!("cannot read {}: {e}", path.display())),
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("devrun: {e}");
        exit(1);
    }
}

fn run() -> Result<(), String> {
    // The xtask crate lives in a subdirectory of the project root.
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("cannot find the project root")?
        .to_path_buf();

    // `join` keeps absolute paths as they are, and appends relative ones to the root.
    let dest_dir = root.join(load_config(&root)?.dest_dir);

    // Before `--`: options for `cargo build`. After `--`: arguments for the application.
    let args: Vec<String> = env::args().skip(1).collect();
    let (build_args, app_args) = match args.iter().position(|a| a == "--") {
        Some(i) => (&args[..i], &args[i + 1..]),
        None => (&args[..], &[][..]),
    };

    let built_exe = build(&root, build_args)?;

    fs::create_dir_all(&dest_dir)
        .map_err(|e| format!("cannot create {}: {e}", dest_dir.display()))?;
    let dest = dest_dir.join(built_exe.file_name().ok_or("executable has no file name")?);

    // Delete first: on Linux, overwriting a running program fails with "Text file busy".
    match fs::remove_file(&dest) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => {
            return Err(format!(
                "cannot remove {} (is the application still running?): {e}",
                dest.display()
            ))
        }
    }
    fs::copy(&built_exe, &dest).map_err(|e| format!("copy failed: {e}"))?;
    eprintln!("devrun: {} -> {}", built_exe.display(), dest.display());

    let status = Command::new(&dest)
        .args(app_args)
        .current_dir(&dest_dir)
        .status()
        .map_err(|e| format!("cannot start {}: {e}", dest.display()))?;
    exit(status.code().unwrap_or(1));
}

/// Runs `cargo build` and returns the path of the built executable.
fn build(root: &Path, build_args: &[String]) -> Result<PathBuf, String> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());

    let mut child = Command::new(cargo)
        .arg("build")
        .args(build_args)
        // Compiler messages are rendered to the terminal as usual (stderr),
        // while stdout only carries machine-readable JSON, which tells us
        // where the executable ended up.
        .arg("--message-format=json-render-diagnostics")
        .current_dir(root)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("cannot start cargo: {e}"))?;

    let mut exe = None;
    let stdout = child.stdout.take().ok_or("cannot read cargo output")?;
    for line in BufReader::new(stdout).lines() {
        let line = line.map_err(|e| e.to_string())?;
        let Ok(msg) = serde_json::from_str::<serde_json::Value>(&line) else {
            continue;
        };
        let is_bin = msg["target"]["kind"]
            .as_array()
            .is_some_and(|kinds| kinds.iter().any(|k| k.as_str() == Some("bin")));
        if msg["reason"].as_str() == Some("compiler-artifact") && is_bin {
            if let Some(path) = msg["executable"].as_str() {
                exe = Some(PathBuf::from(path));
            }
        }
    }

    let status = child.wait().map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("build failed".into());
    }
    exe.ok_or_else(|| "the build did not produce an executable".into())
}