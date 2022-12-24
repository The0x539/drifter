#![feature(exit_status_error)]

const SCALE_SAVE_PATH: &str =
    "/mnt/c/Users/Andrew/AppData/Local/HyperLightDrifter/HyperLight_RecordOfTheDrifter_0.sav";

const NYAMI_NYAMI_SAVE_PATH: &str = "/home/deck/HLD/HyperLight_RecordOfTheDrifter_0.sav";

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

fn main() {
    let drifting_from_deck = match std::env::args().nth(1).as_deref() {
        Some("from-deck") => true,
        Some("to-deck") => false,
        _ => panic!(),
    };

    let scale_save = std::fs::read_to_string(SCALE_SAVE_PATH).unwrap();
    let nyami_nyami_save = std::process::Command::new("ssh")
        .arg("nyami-nyami")
        .args(["cat", NYAMI_NYAMI_SAVE_PATH])
        .output()
        .unwrap()
        .stdout;

    let nyami_nyami_save = String::from_utf8(nyami_nyami_save).unwrap();

    let (scale_header, scale_body) = split(&scale_save);
    let (nyami_nyami_header, nyami_nyami_body) = split(&nyami_nyami_save);

    if drifting_from_deck {
        let reconstructed = unsplit(scale_header, nyami_nyami_body);
        std::fs::write(SCALE_SAVE_PATH, reconstructed.as_bytes()).unwrap();
    } else {
        let reconstructed = unsplit(nyami_nyami_header, scale_body);
        std::process::Command::new("ssh")
            .arg("nyami-nyami")
            .args(["echo", &reconstructed, ">", NYAMI_NYAMI_SAVE_PATH])
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .exit_ok()
            .unwrap();
    }
}
