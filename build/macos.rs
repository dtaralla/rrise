/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

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

/// Updates the default stream manager cc build specs for MacOS targets
fn stream_cc_platform_specifics(build: &mut cc::Build, wwise_sdk: &PathBuf) -> io::Result<()> {
    build
        .compiler("clang")
        .shared_flag(true)
        .opt_level(2)
        .flag("-std=c++17")
        .flag("-MMD")
        .flag("-MP")
        .flag("-fPIC")
        .flag("-g")
        .flag("-Wno-invalid-offsetof")
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .define("AUDIOKINETIC", None)
        .file(
            wwise_sdk
                .join("samples")
                .join("SoundEngine")
                .join("POSIX")
                .join("AkDefaultIOHookBlocking.cpp"),
        )
        .include(wwise_sdk.join("samples").join("SoundEngine").join("POSIX"));

    Ok(())
}
