#![feature(exit_status_error)]

use std::{path::Path, process::Command};

const SCALE_SAVE_DIR: &str = "/mnt/c/Users/Andrew/AppData/Local/HyperLightDrifter";
const NYAMI_NYAMI_SAVE_DIR: &str = "/home/deck/HLD";

fn split(save_file: &str) -> (&str, &str) {
    let (a, b) = save_file.split_once("eyAi").unwrap();
    assert_eq!(a.len(), 80);
    (a, b)
}

fn unsplit(header: &str, body: &str) -> String {
    assert_eq!(header.len(), 80);
    let mut s = String::with_capacity(header.len() + 4 + body.len());
    s.push_str(header);
    s.push_str("eyAi");
    s.push_str(body);
    s
}

fn main() -> anyhow::Result<()> {
    let from_deck = match std::env::args().nth(1).as_deref() {
        Some("from-deck") => true,
        Some("to-deck") => false,
        _ => anyhow::bail!("must specify from-deck or to-deck"),
    };

    transfer("HyperLight_RecordOfTheDrifter_0.sav", from_deck)?;
    transfer("HyperLight_RecordOfTheDrifter__Hoardes_0.sav", from_deck)?;

    Ok(())
}

fn transfer(filename: &str, drifting_from_deck: bool) -> anyhow::Result<()> {
    let scale_save_path = Path::new(SCALE_SAVE_DIR).join(filename);
    let nyami_nyami_save_path = Path::new(NYAMI_NYAMI_SAVE_DIR)
        .join(filename)
        .into_os_string()
        .into_string()
        .unwrap();

    let scale_save = std::fs::read_to_string(&scale_save_path)?;

    let nyami_nyami_save = Command::new("ssh")
        .arg("nyami-nyami")
        .args(["cat", &nyami_nyami_save_path])
        .output()?
        .stdout;

    let nyami_nyami_save = String::from_utf8(nyami_nyami_save)?;

    let (scale_header, scale_body) = split(&scale_save);
    let (nyami_nyami_header, nyami_nyami_body) = split(&nyami_nyami_save);

    if drifting_from_deck {
        let reconstructed = unsplit(scale_header, nyami_nyami_body);
        std::fs::write(scale_save_path, reconstructed.as_bytes())?;
    } else {
        let reconstructed = unsplit(nyami_nyami_header, scale_body);
        Command::new("ssh")
            .arg("nyami-nyami")
            .args(["echo", &reconstructed, ">", &nyami_nyami_save_path])
            .spawn()?
            .wait()?
            .exit_ok()?;
    }

    Ok(())
}
