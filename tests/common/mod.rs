/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use rrise::settings::{AkCommSettings, AkInitSettings, AkMemSettings, AkPlatformInitSettings};
use rrise::{communication, memory_mgr, settings, sound_engine, stream_mgr, AKRESULT};

pub fn init_sound_engine() -> Result<(), AKRESULT> {
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
    sound_engine::init(AkInitSettings::default(), AkPlatformInitSettings::default())?;

    // init comms
    #[cfg(not(wwconfig = "release"))]
    communication::init(AkCommSettings::default())?;

    Ok(())
}

pub fn term_sound_engine() -> Result<(), AKRESULT> {
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
