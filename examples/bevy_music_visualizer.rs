/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use bevy::prelude::*;
use rrise::query_params::{get_rtpc_value, RtpcValueType};
use rrise::settings::*;
use rrise::{sound_engine::*, *};
use std::path::PathBuf;

const DEFAULT_LISTENER_ID: AkGameObjectID = 1;
const THE_GAME_OBJECT: AkGameObjectID = 100;

fn main() -> Result<(), AkResult> {
    // Initialize Wwise
    init_sound_engine()?;
    if !is_initialized() {
        panic!("Unknown error: the sound engine didn't initialize properly");
    }

    // Run the Bevy app
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Meters {
            meters: [
                (String::from("Meters_00"), 0.),
                (String::from("Meters_01"), 0.),
                (String::from("Meters_02"), 0.),
                (String::from("Meters_03"), 0.),
                (String::from("Meters_04"), 0.),
                (String::from("Meters_05"), 0.),
                (String::from("Meters_06"), 0.),
                (String::from("Meters_07"), 0.),
                (String::from("Meters_08"), 0.),
                (String::from("Meters_09"), 0.),
                (String::from("Meters_10"), 0.),
            ],
        })
        .add_startup_system(setup)
        .add_startup_system(setup_audio.chain(error_handler))
        .add_system_to_stage(CoreStage::PreUpdate, audio_metering.chain(error_handler))
        .add_system_to_stage(CoreStage::PostUpdate, audio_rendering.chain(error_handler))
        .add_system(visualize_music)
        .run();

    // Terminate Wwise
    stop_all(None);
    unregister_all_game_obj()?;
    term_sound_engine()?;

    Ok(())
}

fn error_handler(In(result): In<Result<(), AkResult>>) {
    if let Err(akr) = result {
        panic!("Unexpected error in system: {}", akr);
    }
}

#[derive(Component)]
struct BandMeter(usize);

type Meter = (String, AkRtpcValue);
struct Meters {
    meters: [Meter; 11],
}

fn audio_metering(mut meters: ResMut<Meters>) -> Result<(), AkResult> {
    for meter in &mut meters.meters {
        let rtpc_value = get_rtpc_value(meter.0.as_str(), None, None, RtpcValueType::Global(0.))?;
        meter.1 = match rtpc_value {
            RtpcValueType::Global(v) => v,
            _ => 0.,
        };
    }

    Ok(())
}

fn audio_rendering() -> Result<(), AkResult> {
    const ALLOW_SYNC_RENDER: bool = true;
    render_audio(ALLOW_SYNC_RENDER)
}

fn visualize_music(mut visual_meters: Query<(&mut Transform, &BandMeter)>, meters: Res<Meters>) {
    for (mut tfm, meter_index) in visual_meters.iter_mut() {
        let y_scale = 10. * (meters.meters[meter_index.0].1 + 48.) / 54.;
        tfm.scale = Vec3::new(1., y_scale, 1.);
    }
}

fn setup_audio() -> Result<(), AkResult> {
    // Setup Wwise objects and play music
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

    let mut call_count = 0; // Define some state that persists between calls to the callback
    let on_music_sync = move |cb_info: AkCallbackInfo| {
        // ! this will execute on the audio thread !

        call_count += 1;
        println!("on_music_sync was called {} times", call_count);

        if let AkCallbackInfo::MusicSync {
            game_obj_id,
            music_sync_type,
            ..
        } = cb_info
        {
            println!("GameObj: {}, SyncType: {}", game_obj_id, music_sync_type);
        } else {
            println!("Not a music sync: {:?}", cb_info);
        }
    };

    if let Ok(playing_id) = PostEvent::new(THE_GAME_OBJECT, "PlayMeteredMusic")
        .flags(AkCallbackType::AK_MusicSyncBeat | AkCallbackType::AK_MusicSyncBar)
        .post_with_callback(on_music_sync)
    {
        println!("Successfully started event with playingID {}", playing_id)
    } else {
        panic!("Couldn't post event");
    }

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Setup camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0., 0., 15.).looking_at(Vec3::default(), Vec3::Y),
        ..default()
    });

    // Setup light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: Default::default(),
        ..default()
    });

    // Setup audio band visualizers
    spawn_meter(0, "2a4858", &mut commands, &mut meshes, &mut materials);
    spawn_meter(1, "225b6c", &mut commands, &mut meshes, &mut materials);
    spawn_meter(2, "106e7c", &mut commands, &mut meshes, &mut materials);
    spawn_meter(3, "008288", &mut commands, &mut meshes, &mut materials);
    spawn_meter(4, "00968e", &mut commands, &mut meshes, &mut materials);
    spawn_meter(5, "23aa8f", &mut commands, &mut meshes, &mut materials);
    spawn_meter(6, "4abd8c", &mut commands, &mut meshes, &mut materials);
    spawn_meter(7, "72cf85", &mut commands, &mut meshes, &mut materials);
    spawn_meter(8, "9cdf7c", &mut commands, &mut meshes, &mut materials);
    spawn_meter(9, "c9ee73", &mut commands, &mut meshes, &mut materials);
    spawn_meter(10, "fafa6e", &mut commands, &mut meshes, &mut materials);
}

fn spawn_meter(
    index: usize,
    hex: &str,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: 0.0,
                min_y: 0.0,
                min_z: 0.0,
                max_x: 0.25,
                max_y: 1.0,
                max_z: 0.25,
            })),
            material: materials.add(Color::hex(hex).unwrap().into()),
            transform: Transform::from_xyz(-5. + (index as f32), -5., 0.),
            ..default()
        })
        .insert(BandMeter(index));
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

    // init musicengine
    music_engine::init(settings::AkMusicSettings::default())?;

    // init comms
    #[cfg(not(wwrelease))]
    communication::init(AkCommSettings::default())?;

    Ok(())
}

fn term_sound_engine() -> Result<(), AkResult> {
    // term comms
    #[cfg(not(wwrelease))]
    communication::term();

    // term spatial

    // term music
    music_engine::term();

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

    path = if cfg!(wwdebug) {
        path.join("Debug")
    } else if cfg!(wwrelease) {
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
