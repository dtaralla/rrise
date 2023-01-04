/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

include!("unix.rs");

/// Updates build environment with required dependencies for MacOS targets
fn platform_dependencies(wwise_sdk: &PathBuf, config_folder: &str) {
    //  Add needed MacOS frameworks
    println!(
        "cargo:rustc-link-search={}",
        wwise_sdk
            .join("Mac")
            .join(config_folder)
            .join("lib")
            .into_os_string()
            .into_string()
            .unwrap()
    );
    println!(
        "cargo:rustc-link-search={}",
        wwise_sdk
            .join("Mac")
            .join(config_folder)
            .join("bin")
            .into_os_string()
            .into_string()
            .unwrap()
    );

    println!("cargo:rustc-link-lib=framework=AudioToolbox");
    println!("cargo:rustc-link-lib=framework=AudioUnit");
    println!("cargo:rustc-link-lib=framework=CoreAudio");
    println!("cargo:rustc-link-lib=framework=QuartzCore");
    println!("cargo:rustc-link-lib=framework=GLUT");
    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
}
