//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::env;

fn main() {
    // Get the current timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Set the TEST_FOO environment variable to the current timestamp
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // Other build script functionality can go here
    println!("cargo:rerun-if-changed=build.rs");
}