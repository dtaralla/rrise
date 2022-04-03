/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use std::io::{ErrorKind, Read};
use winreg::{enums::*, RegKey, HKEY};

fn get_msvc_include_path() -> io::Result<PathBuf> {
    let mut cl_exe_path = String::new();
    std::process::Command::new("where")
        .arg("cl.exe")
        .output()
        .expect("No output for where cl.exe")
        .stdout
        .as_slice()
        .read_to_string(&mut cl_exe_path)
        .unwrap();

    let cl_exe_path = PathBuf::from(cl_exe_path).parent().unwrap().to_path_buf();
    let msvc_include = cl_exe_path
        .join(r"..\..\..\include")
        .canonicalize()
        .unwrap();

    if msvc_include.is_dir() {
        io::Result::Ok(msvc_include)
    } else {
        io::Result::Err(io::Error::new(
            ErrorKind::Unsupported,
            format!(
                "Can't find MSVC includes at {} based on cl.exe location {}",
                msvc_include.into_os_string().into_string().unwrap(),
                cl_exe_path.into_os_string().into_string().unwrap()
            ),
        ))
    }
}

/// Construct the Windows 10 SDK install path.
fn get_win10_sdk_path() -> io::Result<PathBuf> {
    match get_win10_sdk_dir(HKEY_LOCAL_MACHINE, "SOFTWARE\\Wow6432Node") {
        Ok(path) => Ok(path),
        Err(_) => match get_win10_sdk_dir(HKEY_CURRENT_USER, "SOFTWARE\\Wow6432Node") {
            Ok(path) => Ok(path),
            Err(_) => match get_win10_sdk_dir(HKEY_LOCAL_MACHINE, "SOFTWARE") {
                Ok(path) => Ok(path),
                Err(_) => match get_win10_sdk_dir(HKEY_CURRENT_USER, "SOFTWARE") {
                    Ok(path) => Ok(path),
                    Err(_) => io::Result::Err(io::Error::new(
                        ErrorKind::Unsupported,
                        "Could not find Windows 10 SDK in registry, is is properly installed?",
                    )),
                },
            },
        },
    }
}

/// Parse registry to find the Windows 10 SDKs install path.
fn get_win10_sdk_dir(hkey: HKEY, subkey_root: &str) -> io::Result<PathBuf> {
    let subkey = RegKey::predef(hkey).open_subkey(format!(
        "{}\\Microsoft\\Microsoft SDKs\\Windows\\v10.0",
        subkey_root
    ))?;
    let value: String = subkey.get_value("InstallationFolder")?;
    Ok(PathBuf::from(value))
}

/// Finds the most recent Windows 10 SDK version installed based on the SDKs install path.
fn get_win10_sdk_version(path: &PathBuf) -> io::Result<String> {
    let include_path = path.join("include\\");
    if let Ok(child_dirs) = include_path.read_dir() {
        let mut found: Option<String> = None;
        for p in child_dirs.filter_map(Result::ok) {
            let folder_name = p
                .file_name()
                .into_string()
                .expect("Unreadable SDK folder name");
            if p.path().join("um\\winsdkver.h").is_file() && folder_name.starts_with("10.") {
                found = match found {
                    Some(s) => {
                        if s < folder_name {
                            Some(folder_name)
                        } else {
                            Some(s)
                        }
                    }
                    None => Some(folder_name),
                };
            }
        }

        match found {
            None => io::Result::Err(io::Error::new(
                ErrorKind::Unsupported,
                "Can't deduce SDK version, reinstall Windows 10 SDK",
            )),
            Some(s) => Ok(s),
        }
    } else {
        io::Result::Err(io::Error::new(
            ErrorKind::Unsupported,
            format!(
                "SDK folder corrupted; {:?} doesn't exist; reinstall Windows 10 SDK",
                include_path
            ),
        ))
    }
}

/// Updates the default stream manager cc build specs for Windows target
fn stream_cc_platform_specifics(build: &mut Build, wwise_sdk: &PathBuf) -> io::Result<()> {
    let msvc_include = get_msvc_include_path()?;
    let win10_sdk_path = get_win10_sdk_path()?;
    let win10_sdk_version = get_win10_sdk_version(&win10_sdk_path)?;
    let win10_sdk_lib = win10_sdk_path
        .join("Lib")
        .join(&win10_sdk_version)
        .join("um")
        .join("x64");
    let win10_sdk_include = win10_sdk_path.join("Include").join(&win10_sdk_version);

    println!(
        "cargo:rustc-link-search={}",
        win10_sdk_lib.into_os_string().into_string().unwrap()
    );

    build
        .file(wwise_sdk.join(r"samples\SoundEngine\Win32\AkDefaultIOHookBlocking.cpp"))
        .flag("-MD")
        .flag("-MP")
        .define("WIN64", None)
        .define("WIN32_LEAN_AND_MEAN", None)
        .include(msvc_include)
        .include(win10_sdk_include.join("um"))
        .include(win10_sdk_include.join("shared"))
        .include(win10_sdk_include.join("ucrt"))
        .include(wwise_sdk.join(r"samples\SoundEngine\Win32"));

    Ok(())
}

/// Updates build environment with required dependencies for Windows target
fn platform_dependencies(wwise_sdk: &PathBuf, config_folder: &str) {
    let mut path = wwise_sdk.join("x64_vc160");
    if !path.is_dir() {
        path = wwise_sdk.join("x64_vc150");
        if !path.is_dir() {
            path = wwise_sdk.join("x64_vc140");
        }
    }
    println!(
        "cargo:rustc-link-search={}",
        path.join(config_folder)
            .join("lib")
            .into_os_string()
            .into_string()
            .unwrap()
    );
    println!(
        "cargo:rustc-link-search={}",
        path.join(config_folder)
            .join("bin")
            .into_os_string()
            .into_string()
            .unwrap()
    ); // For effect dlls

    println!("cargo:rustc-link-lib=dylib=winmm");
    println!("cargo:rustc-link-lib=dylib=dsound");
    println!("cargo:rustc-link-lib=dylib=dxguid");
    println!("cargo:rustc-link-lib=dylib=XInput");
    println!("cargo:rustc-link-lib=dylib=user32");

    #[cfg(not(wwconfig = "release"))]
    {
        println!("cargo:rustc-link-lib=dylib=AkAutobahn"); // for WAAPI support in game editors
        println!("cargo:rustc-link-lib=dylib=ws2_32"); // for profiling networking
    }
}
