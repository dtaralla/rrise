/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use bindgen;
use std::env;
use std::io;
use std::io::{ErrorKind, Read};
use std::path::PathBuf;
#[cfg(target_os = "windows")]
use winreg::{enums::*, RegKey, HKEY};

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=c/ak.h");
    println!("cargo:rerun-if-changed=c/utilities/default_streaming_mgr.h");
    println!("cargo:rerun-if-changed=c/utilities/default_streaming_mgr.cpp");

    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("c");
    let wwise_sdk =
        PathBuf::from(env::var("WWISESDK").expect("environment variable WWISESDK not found"));

    let config_folder = if cfg!(wwconfig = "debug") {
        "Debug"
    } else if cfg!(wwconfig = "release") {
        "Release"
    } else {
        "Profile"
    };

    println!("cargo:rustc-link-lib=dylib=AkSoundEngine");
    println!("cargo:rustc-link-lib=dylib=AkMusicEngine");
    println!("cargo:rustc-link-lib=dylib=AkSpatialAudio");
    println!("cargo:rustc-link-lib=dylib=AkMemoryMgr");
    println!("cargo:rustc-link-lib=dylib=AkStreamMgr");
    println!("cargo:rustc-link-lib=dylib=AkVorbisDecoder");
    println!("cargo:rustc-link-lib=dylib=AkOpusDecoder");
    println!("cargo:rustc-link-lib=dylib=AkAudioInputSource");
    println!("cargo:rustc-link-lib=dylib=AkCompressorFX");
    println!("cargo:rustc-link-lib=dylib=AkDelayFX");
    println!("cargo:rustc-link-lib=dylib=AkExpanderFX");
    println!("cargo:rustc-link-lib=dylib=AkFlangerFX");
    println!("cargo:rustc-link-lib=dylib=AkGainFX");
    println!("cargo:rustc-link-lib=dylib=AkGuitarDistortionFX");
    println!("cargo:rustc-link-lib=dylib=AkHarmonizerFX");
    println!("cargo:rustc-link-lib=dylib=AkMatrixReverbFX");
    println!("cargo:rustc-link-lib=dylib=AkMeterFX");
    println!("cargo:rustc-link-lib=dylib=AkParametricEQFX");
    println!("cargo:rustc-link-lib=dylib=AkPeakLimiterFX");
    println!("cargo:rustc-link-lib=dylib=AkPitchShifterFX");
    println!("cargo:rustc-link-lib=dylib=AkRecorderFX");
    println!("cargo:rustc-link-lib=dylib=AkRoomVerbFX");
    println!("cargo:rustc-link-lib=dylib=AkSilenceSource");
    println!("cargo:rustc-link-lib=dylib=AkSineSource");
    println!("cargo:rustc-link-lib=dylib=AkStereoDelayFX");
    println!("cargo:rustc-link-lib=dylib=AkSynthOneSource");
    println!("cargo:rustc-link-lib=dylib=AkTimeStretchFX");
    println!("cargo:rustc-link-lib=dylib=AkToneSource");
    println!("cargo:rustc-link-lib=dylib=AkTremoloFX");
    println!("cargo:rustc-link-lib=dylib=SFLib");

    #[cfg(target_os = "windows")]
    {
        println!(
            "cargo:rustc-link-search={}",
            wwise_sdk
                .join("x64_vc160")
                .join(config_folder)
                .join("lib")
                .into_os_string()
                .into_string()
                .unwrap()
        );
        println!(
            "cargo:rustc-link-search={}",
            wwise_sdk
                .join("x64_vc160")
                .join(config_folder)
                .join("bin")
                .into_os_string()
                .into_string()
                .unwrap()
        ); // For effect dlls

        println!("cargo:rustc-link-lib=dylib=winmm");
        println!("cargo:rustc-link-lib=dylib=dsound");
        println!("cargo:rustc-link-lib=dylib=dxguid");
        println!("cargo:rustc-link-lib=dylib=user32");
    }

    #[cfg(target_os = "linux")]
    {
        // TODO
    }

    #[cfg(target_os = "macos")]
    {
        // TODO
    }

    #[cfg(not(wwconfig = "release"))]
    {
        println!("cargo:rustc-link-lib=dylib=AkAutobahn");
        println!("cargo:rustc-link-lib=dylib=CommunicationCentral");
        println!("cargo:rustc-link-lib=dylib=ws2_32");
    }

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file(crate_dir.join(r"utilities\default_streaming_mgr.cpp"))
        .file(wwise_sdk.join(r"samples\SoundEngine\Common\AkFilePackage.cpp"))
        .file(wwise_sdk.join(r"samples\SoundEngine\Common\AkFilePackageLUT.cpp"))
        .file(wwise_sdk.join(r"samples\SoundEngine\Common\AkMultipleFileLocation.cpp"))
        .file(wwise_sdk.join(r"samples\SoundEngine\Common\AkFileLocationBase.cpp"))
        .file(wwise_sdk.join(r"samples\SoundEngine\Common\AkDefaultLowLevelIODispatcher.cpp"))
        .include(wwise_sdk.join("include"))
        .include(wwise_sdk.join("samples\\SoundEngine"))
        .define("UNICODE", None)
        .no_default_flags(true)
        .flag_if_supported("-nologo")
        .flag_if_supported("-MD")
        .flag_if_supported("-Brepro")
        .flag_if_supported("-W3");

    #[cfg(target_os = "windows")]
    {
        let msvc_include = get_msvc_include_path()?;
        let win10_sdk_path = get_win10_sdk_path()?;
        let win10_sdk_version = get_win10_sdk_version(&win10_sdk_path)?;
        let win10_sdk_lib = win10_sdk_path
            .join("Lib")
            .join(&win10_sdk_version)
            .join("um\\x64");
        let win10_sdk_include = win10_sdk_path.join("Include").join(&win10_sdk_version);

        println!(
            "cargo:rustc-link-search={}",
            win10_sdk_lib.into_os_string().into_string().unwrap()
        );

        build
            .file(wwise_sdk.join(r"samples\SoundEngine\Win32\AkDefaultIOHookBlocking.cpp"))
            .define("WIN64", None)
            .define("WIN32_LEAN_AND_MEAN", None)
            .include(msvc_include)
            .include(win10_sdk_include.join("um"))
            .include(win10_sdk_include.join("shared"))
            .include(win10_sdk_include.join("ucrt"))
            .include(wwise_sdk.join("samples\\SoundEngine\\Win32"));
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        // TODO
        build
            .file(wwise_sdk.join(r"samples\SoundEngine\POSIX\AkDefaultIOHookBlocking.cpp"))
            .include(wwise_sdk.join("samples\\SoundEngine\\POSIX"));
    }

    #[cfg(wwconfig = "debug")]
    {
        build.flag_if_supported("-Z7").define("NDEBUG", None);
    }

    #[cfg(wwconfig = "release")]
    {
        build.define("AK_OPTIMIZED", None);
    }

    build.compile("default_streaming_mgr");

    let bindings = bindgen::Builder::default()
        .header("c/ak.h")
        .header("c/utilities/default_streaming_mgr.h")
        .clang_arg(format!(
            "-I{}",
            wwise_sdk
                .join("include")
                .into_os_string()
                .into_string()
                .unwrap()
        ))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14")
        .opaque_type("AkArray")
        .opaque_type("AkListBareLight")
        .opaque_type("AkHashList")
        .opaque_type("AkHashList_HashTableArray")
        .opaque_type("AkHashListBare")
        .opaque_type("AkHashListBare_HashTableArray")
        .opaque_type("AkDbString")
        .opaque_type("AkDbString_Instance")
        .allowlist_type("AK::.*")
        .allowlist_type("Ak.*")
        .allowlist_type("AK.*")
        .allowlist_var("AK::.*")
        .allowlist_var("AK.*")
        .allowlist_function("AK::.*")
        .allowlist_function("Ak.*")
        .allowlist_function("InitDefaultStreamMgr")
        .allowlist_function("TermDefaultStreamMgr")
        .blocklist_item("AK_INVALID_GAME_OBJECT")
        .blocklist_item("AK_INVALID_AUDIO_OBJECT_ID")
        .rustified_enum("AKRESULT")
        .rustified_enum("AkGroupType")
        .rustified_enum("AkConnectionType")
        .rustified_enum("AkCurveInterpolation")
        .rustified_enum("MultiPositionType")
        .rustified_enum("AkSpeakerPanningType")
        .rustified_enum("Ak3DPositionType")
        .rustified_enum("AkPanningRule")
        .rustified_enum("Ak3DSpatializationMode")
        .rustified_enum("AkPluginType")
        .rustified_enum("AkNodeType")
        .bitfield_enum("AkAudioDeviceState")
        .bitfield_enum("AkBusHierarchyFlags")
        .bitfield_enum("AkMeteringFlags")
        .bitfield_enum("AkCallbackType")
        .must_use_type("AKRESULT")
        .enable_cxx_namespaces()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}

/// Construct the MSVC include path containing the standard library headers
#[cfg(target_os = "windows")]
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
#[cfg(target_os = "windows")]
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
#[cfg(target_os = "windows")]
fn get_win10_sdk_dir(hkey: HKEY, subkey_root: &str) -> io::Result<PathBuf> {
    let subkey = RegKey::predef(hkey).open_subkey(format!(
        "{}\\Microsoft\\Microsoft SDKs\\Windows\\v10.0",
        subkey_root
    ))?;
    let value: String = subkey.get_value("InstallationFolder")?;
    Ok(PathBuf::from(value))
}

/// Finds the most recent Windows 10 SDK version installed based on the SDKs install path.
#[cfg(target_os = "windows")]
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
