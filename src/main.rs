//! Get the CWD of the most recently focused Alacritty window, in niri.

use std::{env, fs, path::PathBuf, process::Command};

use anyhow::{Context, Error, Result, bail, ensure};
use niri_ipc::{Request, Response, Timestamp, socket::Socket};

fn main() {
    println!("{}", get_cwd().display());
}

fn get_cwd() -> PathBuf {
    let e = match try_get_cwd() {
        Ok(path) => return path,
        Err(e) => e,
    };

    eprintln!("{e}");
    eprintln!("Falling back to home directory");

    if let Some(path) = env::home_dir() {
        return path;
    };

    eprintln!("Falling back to /");

    "/".into()
}

fn try_get_cwd() -> Result<PathBuf> {
    let response = Socket::connect()?
        .send(Request::Windows)?
        .map_err(Error::msg)?;
    let Response::Windows(mut windows) = response else {
        bail!("expected Response::Windows, got {response:?}");
    };

    // Get most recently focused alacritty window.
    windows.retain(|w| w.app_id.as_ref().is_some_and(|id| id == "Alacritty"));
    windows.sort_by_key(|w| {
        w.focus_timestamp
            .map(|Timestamp { secs, nanos }| (secs, nanos))
    });
    let pid = windows
        .last()
        .context("no Alacritty windows")?
        .pid
        .context("no pid")?;

    // Get child pid.
    let pgrep = Command::new("pgrep")
        .args(["--newest", "--parent", &format!("{pid}")])
        .output()?;
    ensure!(pgrep.status.success());
    let child_pid: i32 = String::from_utf8(pgrep.stdout)?.trim().parse()?;

    // Get cwd.
    let cwd = fs::read_link(format!("/proc/{child_pid}/cwd"))?;

    Ok(cwd)
}
