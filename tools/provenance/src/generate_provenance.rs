use std::fs;
use std::process::Command;
use serde_json::Value;
use chrono::Utc;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let commit_message = args.get(1).expect("Missing commit message");

    let grammar: Value = serde_json::from_str(
        &fs::read_to_string("../ontology/visual-grammar.json").unwrap()
    ).unwrap();

    let gitmoji = &commit_message[0..2];

    let mut altitude = "UNKNOWN";
    let mut membrane = "UNKNOWN";

    if let Some(altitudes) = grammar.get("altitudes") {
        for (alt, data) in altitudes.as_object().unwrap() {
            if data["gitmoji"].as_array().unwrap()
                .iter()
                .any(|g| g.as_str().unwrap() == gitmoji)
            {
                altitude = alt;
                membrane = data["membrane"].as_str().unwrap();
            }
        }
    }

    let hash = String::from_utf8(
        Command::new("git").args(["rev-parse", "HEAD"]).output().unwrap().stdout
    ).unwrap().trim().to_string();

    let timestamp = Utc::now().to_rfc3339();

    let provenance = format!(
        "Commit: {}\nMessage: {}\nAltitude: {}\nMembrane: {}\nTimestamp: {}\nSeal: MAC_v1_0\n",
        hash, commit_message, altitude, membrane, timestamp
    );

    let out_path = format!("provenance/commit_log/{}_provenance.txt", hash);
    fs::write(&out_path, provenance).unwrap();

    println!("Generated provenance log: {}", out_path);
}
