/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use rrise::settings::*;
use rrise::{sound_engine::*, *};
use std::path::PathBuf;

use lerp::Lerp;
use simple_logger::SimpleLogger;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

const DEFAULT_LISTENER_ID: AkGameObjectID = 1;
const THE_GAME_OBJECT: AkGameObjectID = 100;

fn main() -> Result<(), AkResult> {
    SimpleLogger::new().init().unwrap();
    let should_stop = Arc::new(AtomicBool::new(false));

    let sstop = should_stop.clone();
    ctrlc::set_handler(move || {
        sstop.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    init_sound_engine()?;

    if !is_initialized() {
        panic!("Unknown error: the sound engine didn't initialize properly");
    }

    register_game_obj(DEFAULT_LISTENER_ID)?;

    let listeners = vec![DEFAULT_LISTENER_ID];
    set_default_listeners(&listeners)?;

    register_game_obj(THE_GAME_OBJECT)?;

    if let Err(akr) = load_bank_by_name("Init.bnk") {
        panic!("Couldn't load initbank: {}", akr);
    }

    if let Err(akr) = load_bank_by_name("TheBank.bnk") {
        panic!("Couldn't load thebank: {}", akr);
    }

    if let Ok(playing_id) = PostEvent::from_event_id(THE_GAME_OBJECT, 2586140731).post() {
        println!("Successfully started event with playingID {}", playing_id)
    } else {
        panic!("Couldn't post event");
    }

    // Finally we run the standard application loop -
    let instant = std::time::Instant::now();
    loop {
        let app_time = instant.elapsed().as_secs_f32();

        const ALLOW_SYNC_RENDER: bool = true;
        render_audio(ALLOW_SYNC_RENDER)?;

        if should_stop.load(Ordering::SeqCst) {
            stop_all(None);
            unregister_all_game_obj()?;
            break;
        }

        // move sound source from -3 to 3 along X axis over the sound length (0.974s), repeatedly
        let new_p = (-3.).lerp(3., app_time % 0.974);
        set_position(THE_GAME_OBJECT, AkTransform::from([new_p, 0., 0.]))?;

        // simulate ~60 frames per second
        std::thread::sleep(Duration::from_millis(1000 / 60));
    }

    term_sound_engine()?;

    Ok(())
}

fn init_sound_engine() -> Result<(), AkResult> {
    // init memorymgr
    memory_mgr::init(AkMemSettings::default())?;
    assert!(memory_mgr::is_initialized());

    // init streamingmgr
    #[cfg(target_os = "windows")]
    let platform = "Windows";
    #[cfg(target_os = "linux")]
    let platform = "Linux";
    stream_mgr::init_default_stream_mgr(
        settings::AkStreamMgrSettings::default(),
        settings::AkDeviceSettings::default(),
        format!("examples/WwiseProject/GeneratedSoundBanks/{}", platform),
    )?;
    stream_mgr::set_current_language("English(US)")?;

    // init soundengine
    sound_engine::init(setup_example_dll_path(), AkPlatformInitSettings::default())?;

    // init comms
    #[cfg(not(wwconfig = "release"))]
    communication::init(AkCommSettings::default())?;

    Ok(())
}

fn term_sound_engine() -> Result<(), AkResult> {
    // term comms
    #[cfg(not(wwconfig = "release"))]
    communication::term();

    // term spatial

    // term music

    // term soundengine
    sound_engine::term();

    // term streamingmgr
    stream_mgr::term_default_stream_mgr();

    // term memorymgr
    memory_mgr::term();

    Ok(())
}

fn setup_example_dll_path() -> AkInitSettings {
    let wwise_sdk = PathBuf::from(std::env::var("WWISESDK").expect("env var WWISESDK not found"));

    let mut path;
    #[cfg(windows)]
    {
        path = wwise_sdk.join("x64_vc160");
        if !path.is_dir() {
            path = wwise_sdk.join("x64_vc150");
            if !path.is_dir() {
                path = wwise_sdk.join("x64_vc140");
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        path = wwise_sdk.join("Linux_x64");
    }

    path = if cfg!(wwconfig = "debug") {
        path.join("Debug")
    } else if cfg!(wwconfig = "release") {
        path.join("Release")
    } else {
        path.join("Profile")
    };

    // -- KNOWN ISSUE ON WINDOWS --
    // If WWISESDK contains spaces, the DLLs can't be discovered.
    // Help wanted!
    // Anyway, if you truly wanted to deploy something based on this crate with dynamic loading of
    // Wwise plugins, you would need to make sure to deploy any Wwise shared library (SO or DLL)
    // along your executable. You can't expect your players to have Wwise installed!
    // You can also just statically link everything, using this crate features. Enabling a feature
    // then forcing a rebuild will statically link the selected plugins instead of letting Wwise
    // look for their shared libraries at runtime.
    // Legal: Remember that Wwise is a licensed product, and you can't distribute their code,
    // statically linked or not, without a proper license.
    AkInitSettings::default()
        .with_plugin_dll_path(path.join("bin").into_os_string().into_string().unwrap())
}
