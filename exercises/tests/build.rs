//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

// build.rs
use std::env;

fn main() {
    // Set TEST_FOO environment variable for tests7.rs
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // Enable 'pass' feature for tests8.rs
    println!("cargo:rustc-cfg=feature=\"pass\"");

    // Rebuild if build script changes
    println!("cargo:rerun-if-changed=build.rs");
}
