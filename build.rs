// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::env;

fn main() {
    let test_mode = env::var("AZURE_TEST_MODE")
        .unwrap_or_else(|_| "playback".to_string())
        .to_ascii_lowercase();

    // cspell:ignore rustc
    match test_mode.as_str() {
        "playback" | "record" | "live" => println!("cargo::rustc-cfg=test_mode=\"{test_mode}\""),
        _ => panic!("AZURE_TEST_MODE=\"{test_mode}\" not supported"),
    };

    println!("cargo::rerun-if-env-changed=AZURE_TEST_MODE");
}
