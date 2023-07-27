#!/usr/bin/env -S cargo +nightly -Zscript -q

//! ```cargo
//! [dependencies]
//! reqwest = { version = "0.11", features = ["blocking", "json"] }
//! serde = { version = "1", features = ["derive"] }
//! ```

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Stable {
    version: String,
}

#[derive(Debug, Deserialize)]
struct Released {
    timestamp: u64,
    isodate: String,
}

#[derive(Debug, Deserialize)]
struct Patch {
    full: Option<String>,
    incremental: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Release {
    iseol: bool,
    version: String,
    moniker: String,
    source: Option<String>,
    pgp: Option<String>,
    released: Released,
    gitweb: String,
    changelog: Option<String>,
    diffview: Option<String>,
    patch: Patch,
}

#[derive(Debug, Deserialize)]
struct Response {
    latest_stable: Stable,
    releases: Vec<Release>,
}

fn main() {
    let response = reqwest::blocking::get("https://www.kernel.org/releases.json").unwrap();
    let var: Response = response.json().unwrap();
    println!("{:?}", var);
}
