#!/usr/bin/env -S cargo +nightly -Zscript -q

//! ```cargo
//! [dependencies]
//! reqwest = { version = "0.11", features = ["blocking"] }
//! ```


fn main() {
    let response = reqwest::blocking::get("https://www.kernel.org/releases.json").unwrap();
    println!("{}", response.text().unwrap());
}
