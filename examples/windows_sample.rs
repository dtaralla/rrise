/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use rrise::settings::*;
use rrise::{sound_engine::*, *};

use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

const DEFAULT_LISTENER_ID: AkGameObjectID = 1;
const THE_GAME_OBJECT: AkGameObjectID = 100;

fn main() -> Result<(), AKRESULT> {
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
    loop {
        const ALLOW_SYNC_RENDER: bool = true;
        render_audio(ALLOW_SYNC_RENDER)?;

        if should_stop.load(Ordering::SeqCst) {
            stop_all(AK_INVALID_GAME_OBJECT);
            unregister_all_game_obj()?;
            break;
        }

        // simulate ~60 frames per second
        std::thread::sleep(Duration::from_millis(1000 / 60));
    }

    term_sound_engine()?;

    Ok(())
}

fn init_sound_engine() -> Result<(), AKRESULT> {
    // init memorymgr
    memory_mgr::init(AkMemSettings::default())?;
    assert!(memory_mgr::is_initialized());

    // init streamingmgr
    stream_mgr::init_default_stream_mgr(
        settings::AkStreamMgrSettings::default(),
        settings::AkDeviceSettings::default(),
        r"examples\WwiseProject\GeneratedSoundBanks\Windows",
    )?;
    stream_mgr::set_current_language("English(US)")?;

    // init soundengine
    sound_engine::init(AkInitSettings::default(), AkPlatformInitSettings::default())?;

    // init comms
    #[cfg(not(wwconfig = "release"))]
    communication::init(AkCommSettings::default())?;

    Ok(())
}

fn term_sound_engine() -> Result<(), AKRESULT> {
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
