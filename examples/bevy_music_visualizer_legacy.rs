/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use rrise::{
    query_params::{get_rtpc_value, RtpcValueType},
    settings::*,
    sound_engine::*,
    *,
};
use rrise_headers::rr;

use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};
use std::path::PathBuf;

#[cfg(windows)]
use cc;

const DEFAULT_LISTENER_ID: AkGameObjectID = 1;
const THE_GAME_OBJECT: AkGameObjectID = 100;

fn main() -> Result<(), AkResult> {
    // Run the Bevy app
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Meters {
            meters: [
                (rr::xbus::Meters_00, 0.),
                (rr::xbus::Meters_01, 0.),
                (rr::xbus::Meters_02, 0.),
                (rr::xbus::Meters_03, 0.),
                (rr::xbus::Meters_04, 0.),
                (rr::xbus::Meters_05, 0.),
                (rr::xbus::Meters_06, 0.),
                (rr::xbus::Meters_07, 0.),
                (rr::xbus::Meters_08, 0.),
                (rr::xbus::Meters_09, 0.),
                (rr::xbus::Meters_10, 0.),
            ],
        })
        .insert_resource(match crossbeam_channel::unbounded() {
            (sender, receiver) => CallbackChannel { sender, receiver },
        })
        .add_startup_system(init_sound_engine.pipe(system_adapter::unwrap))
        .add_startup_system_to_stage(
            StartupStage::PostStartup,
            setup_audio.pipe(system_adapter::unwrap),
        ) // PostStartup so that Bevy Window already initialized
        .add_startup_system(setup)
        .add_system_to_stage(
            CoreStage::PreUpdate,
            audio_metering.pipe(system_adapter::unwrap),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            audio_rendering.pipe(system_adapter::unwrap),
        )
        .add_system(visualize_music)
        .add_system(process_callbacks)
        .run();

    // Terminate Wwise
    stop_all(None);
    unregister_all_game_obj()?;
    term_sound_engine()?;

    Ok(())
}

#[derive(Component)]
struct BandMeter(usize);

#[derive(Component)]
struct BeatBarText;

type Meter = (AkUniqueID, AkRtpcValue);

#[derive(Resource)]
struct Meters {
    meters: [Meter; 11],
}

#[derive(Clone, Resource)]
struct CallbackChannel {
    sender: Sender<AkCallbackInfo>,
    receiver: Receiver<AkCallbackInfo>,
}

fn audio_metering(mut meters: ResMut<Meters>) -> Result<(), AkResult> {
    for meter in &mut meters.meters {
        let rtpc_value = get_rtpc_value(meter.0, None, None, RtpcValueType::Global(0.))?;
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

fn process_callbacks(
    mut beat_bar_text: Query<&mut Text, With<BeatBarText>>,
    time: Res<Time>,
    callback_channel: Res<CallbackChannel>,
) {
    let mut beat_bar_text = beat_bar_text.single_mut();
    while let Ok(cb_info) = callback_channel.receiver.try_recv() {
        match cb_info {
            AkCallbackInfo::MusicSync {
                music_sync_type: AkCallbackType::AK_MusicSyncBar,
                ..
            } => beat_bar_text.sections[1].value = format!("{:.1}s", time.elapsed_seconds()),
            AkCallbackInfo::MusicSync {
                music_sync_type: AkCallbackType::AK_MusicSyncBeat,
                ..
            } => beat_bar_text.sections[4].value = format!("{:.1}s", time.elapsed_seconds()),
            _ => (),
        };
    }
}

fn setup_audio(callback_channel: Res<CallbackChannel>) -> Result<(), AkResult> {
    // Setup Wwise objects and play music
    register_game_obj(DEFAULT_LISTENER_ID)?;
    add_default_listener(DEFAULT_LISTENER_ID)?;

    register_game_obj(THE_GAME_OBJECT)?;

    if let Err(akr) = load_bank_by_name(rr::bnk::Init) {
        panic!("Couldn't load initbank: {}", akr);
    }

    if let Err(akr) = load_bank_by_name(rr::bnk::TheBank) {
        panic!("Couldn't load thebank: {}", akr);
    }

    let mut call_count = 0; // Define some state that persists between calls to the callback
    let captured_callback_channel = callback_channel.clone();
    let on_music_sync = move |cb_info: AkCallbackInfo| {
        // ! this will execute on the audio thread !

        // Tell the game thread we received a callback, for safe game code execution related to it
        captured_callback_channel
            .sender
            .try_send(cb_info.clone())
            .unwrap_or(());

        // Diagnostic message: persisting state can be updated
        call_count += 1;
        println!("on_music_sync was called {} times", call_count);

        // Some more diagnostic messages: describe what was callback was received
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

    if let Ok(playing_id) = PostEvent::new(THE_GAME_OBJECT, rr::ev::PlayMeteredMusic)
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
    asset_server: Res<AssetServer>,
) {
    // Setup cameras
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 15.).looking_at(Vec3::default(), Vec3::Y),
        ..default()
    });

    // Setup light
    commands.spawn(DirectionalLightBundle {
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

    // Setup beat/bar displays
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands.spawn((
        TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(8.0),
                    right: Val::Px(8.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                alignment: TextAlignment {
                    horizontal: HorizontalAlign::Right,
                    ..default()
                },
                sections: vec![
                    TextSection {
                        value: "Last beat: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            ..default()
                        },
                    },
                    TextSection {
                        value: "0.0s".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            ..default()
                        },
                    },
                    TextSection {
                        value: "\n".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            ..default()
                        },
                    },
                    TextSection {
                        value: "Last bar: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            ..default()
                        },
                    },
                    TextSection {
                        value: "0.0s".to_string(),
                        style: TextStyle { font, ..default() },
                    },
                ],
            },
            ..default()
        },
        BeatBarText,
    ));
}

fn spawn_meter(
    index: usize,
    hex: &str,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
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
        },
        BandMeter(index),
    ));
}

#[cfg_attr(target_os = "linux", allow(unused_variables))]
fn init_sound_engine() -> Result<(), AkResult> {
    // init memorymgr
    memory_mgr::init(&mut AkMemSettings::default())?;
    assert!(memory_mgr::is_initialized());

    // init streamingmgr
    #[cfg(target_os = "windows")]
    let platform = "Windows";
    #[cfg(target_os = "linux")]
    let platform = "Linux";
    stream_mgr::init_default_stream_mgr(
        &AkStreamMgrSettings::default(),
        &mut AkDeviceSettings::default(),
        format!("assets/soundbanks/{}", platform),
    )?;
    stream_mgr::set_current_language("English(US)")?;

    // init soundengine
    {
        let mut platform_settings = AkPlatformInitSettings::default();
        sound_engine::init(&mut setup_example_dll_path(), &mut platform_settings)?;
    }

    // init musicengine
    music_engine::init(&mut settings::AkMusicSettings::default())?;

    // init comms
    #[cfg(not(wwrelease))]
    communication::init(&AkCommSettings::default())?;

    if !is_initialized() {
        panic!("Unknown error: the sound engine didn't initialize properly");
    }

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
        let vs_version = cc::windows_registry::find_vs_version().expect("No MSVC install found");

        let wwise_vc = match vs_version {
            cc::windows_registry::VsVers::Vs14 => "x64_vc140",
            cc::windows_registry::VsVers::Vs15 => "x64_vc150",
            cc::windows_registry::VsVers::Vs16 => "x64_vc160",
            cc::windows_registry::VsVers::Vs17 => "x64_vc170",
            _ => panic!("Unsupported MSVC version: {:?}", vs_version),
        };
        path = wwise_sdk.join(wwise_vc);

        if !path.exists() {
            panic!(
                "Could not find {}.\n\
                You are using MSVC {:?} but the {} Wwise SDK target probably wasn't installed or \
                doesn't exist for this version of Wwise.\n\
                Note that Vs17 (Visual Studio 2022) is supported since Wwise 2021.1.10 only.",
                path.to_str().unwrap(),
                vs_version,
                wwise_vc
            )
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
