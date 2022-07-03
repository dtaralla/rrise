/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use bevy::prelude::*;
use bevy::render::camera::Camera3d;
use bevy::render::mesh::Indices::U32;
use bevy::render::mesh::VertexAttributeValues;
use rrise::settings::*;
use rrise::spatial_audio::geometry::AkGeometryParams;
use rrise::spatial_audio::room::{
    set_game_object_in_room, set_portal, set_portal_obstruction_and_occlusion, AkPortalParams,
    AkRoomParams,
};
use rrise::spatial_audio::{
    AkExtent, AkGeometrySetID, AkRoomID, AkTriangle, AkVertex, OUTDOOR_ROOM_ID,
};
use rrise::{sound_engine::*, *};
use rrise_headers::rr;
use smooth_bevy_cameras::controllers::unreal::{
    UnrealCameraBundle, UnrealCameraController, UnrealCameraPlugin,
};
use smooth_bevy_cameras::LookTransformPlugin;
use std::f32::consts::PI;
use std::path::PathBuf;

const DEFAULT_LISTENER_ID: AkGameObjectID = 1;
const THE_GAME_OBJECT: AkGameObjectID = 100;
const BIG_ROOM: AkRoomID = 200;
const SMALL_ROOM: AkRoomID = 201;
const INNER_PORTAL: AkRoomID = 202;
const OUTER_PORTAL: AkRoomID = 203;
const BIG_ROOM_GEOM: AkGeometrySetID = 300;
const SMALL_ROOM_GEOM: AkGeometrySetID = 301;

fn main() -> Result<(), AkResult> {
    // Run the Bevy app
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_plugin(UnrealCameraPlugin::default())
        .add_startup_system_to_stage(
            StartupStage::PostStartup,
            init_sound_engine.chain(error_handler).label("init"),
        )
        .add_startup_system_to_stage(
            StartupStage::PostStartup,
            setup.chain(error_handler).after("init"),
        )
        .add_system_to_stage(CoreStage::PostUpdate, audio_rendering.chain(error_handler))
        .add_system(update_listener_position.chain(error_handler))
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

fn audio_rendering() -> Result<(), AkResult> {
    const ALLOW_SYNC_RENDER: bool = true;
    render_audio(ALLOW_SYNC_RENDER)
}

fn to_ak_transform(tfm: &Transform) -> AkTransform {
    let mut pos = tfm.translation.to_array();
    pos[2] = -pos[2];
    let mut ak_tfm = AkTransform::from_position(pos);
    let mut front = tfm.forward().to_array();
    front[2] = -front[2];
    ak_tfm.orientationFront = front.into();
    let mut up = tfm.up().to_array();
    up[2] = -up[2];
    ak_tfm.orientationTop = up.into();
    ak_tfm
}

fn update_listener_position(camera: Query<&Transform, With<Camera3d>>) -> Result<(), AkResult> {
    let camera = camera.single();
    set_position(DEFAULT_LISTENER_ID, to_ak_transform(camera))
        .unwrap_or_else(|e| println!("Can't update listener this frame: {}", e));

    if -7. < camera.translation.x
        && camera.translation.x < 0.
        && 0. < camera.translation.y
        && camera.translation.y < 3.
        && -2.5 < camera.translation.z
        && camera.translation.z < 2.5
    {
        set_game_object_in_room(DEFAULT_LISTENER_ID, BIG_ROOM)?;
    } else if 0. < camera.translation.x
        && camera.translation.x < 3.
        && 0. < camera.translation.y
        && camera.translation.y < 3.
        && -2.5 < camera.translation.z
        && camera.translation.z < 2.5
    {
        set_game_object_in_room(DEFAULT_LISTENER_ID, SMALL_ROOM)?;
    } else {
        set_game_object_in_room(DEFAULT_LISTENER_ID, OUTDOOR_ROOM_ID)?;
    }

    Ok(())
}

fn register_room(
    mesh: &Mesh,
    tfm: &Transform,
    id: AkRoomID,
    geom_id: AkGeometrySetID,
    xbus_id: AkAuxBusID,
) -> Result<(), AkResult> {
    let mut geom = AkGeometryParams {
        triangles: vec![],
        vertices: vec![],
        surfaces: vec![],
        room_id: OUTDOOR_ROOM_ID,
        enable_diffraction: false,
        enable_diffraction_on_boundary_edges: false,
        enable_triangles: false,
    };

    // Register room geometry
    if let VertexAttributeValues::Float32x3(values) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
    {
        for v in values {
            geom.vertices.push(AkVertex {
                X: tfm.translation.x + v[0],
                Y: tfm.translation.y + v[1],
                Z: -tfm.translation.z + -v[2],
            });
        }

        if let U32(idxs) = mesh.indices().unwrap() {
            let mut i = idxs.into_iter();
            while let Some(idx) = i.next() {
                geom.triangles.push(AkTriangle {
                    point0: *idx as u16,
                    point1: *i.next().unwrap() as u16,
                    point2: *i.next().unwrap() as u16,
                    surface: u16::MAX,
                });
            }
        } else {
            panic!("Unexpected mesh given: no indices");
        }
    } else {
        panic!("Unexpected mesh given: no vertices");
    }
    spatial_audio::geometry::set_geometry(geom_id, geom)?;

    // Register room
    let mut front = tfm.forward().to_array();
    front[2] = -front[2];
    let mut up = tfm.up().to_array();
    up[2] = -up[2];
    spatial_audio::room::set_room(
        id,
        AkRoomParams {
            front: front.into(),
            up: up.into(),
            reverb_aux_bus: xbus_id,
            reverb_level: 0.0,
            transmission_loss: 0.9,
            name: format!("Room {}", id),
            room_game_obj_aux_send_level_to_self: 0.0,
            room_game_obj_keep_registered: false,
            geometry_id: geom_id,
        },
    )?;

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Result<(), AkResult> {
    if let Err(akr) = load_bank_by_name(rr::bnk::Init) {
        panic!("Couldn't load initbank: {}", akr);
    }

    if let Err(akr) = load_bank_by_name(rr::bnk::TheBank) {
        panic!("Couldn't load thebank: {}", akr);
    }

    // Setup Wwise objects and play music
    register_game_obj(DEFAULT_LISTENER_ID)?;
    add_default_listener(DEFAULT_LISTENER_ID)?;
    spatial_audio::register_listener(DEFAULT_LISTENER_ID)?;

    register_game_obj(THE_GAME_OBJECT)?;
    set_position(
        THE_GAME_OBJECT,
        to_ak_transform(&Transform::from_xyz(2., 1.5, 1.5)),
    )?;

    if let Ok(playing_id) = PostEvent::new(THE_GAME_OBJECT, rr::ev::Play3DMusic).post() {
        println!("Successfully started event with playingID {}", playing_id)
    } else {
        panic!("Couldn't post event");
    }

    // Flying camera
    commands.spawn_bundle(UnrealCameraBundle::new(
        UnrealCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(2.0, 5.0, -5.0),
        Vec3::new(0., 0., 0.),
    ));

    let room_mat = StandardMaterial {
        base_color: Color::GRAY,
        double_sided: true,
        cull_mode: None,
        ..default()
    };

    // Big room
    let big_room = Mesh::from(shape::Box::new(7., 3., 5.));
    let big_room_tfm = Transform::from_xyz(-3.5, 1.5, 0.);
    register_room(
        &big_room,
        &big_room_tfm,
        BIG_ROOM,
        BIG_ROOM_GEOM,
        rr::xbus::BigRoom,
    )?;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(big_room),
        material: materials.add(room_mat.clone()),
        transform: big_room_tfm,
        ..default()
    });

    // Small room
    let small_room = Mesh::from(shape::Box::new(3., 3., 5.));
    let small_room_tfm = Transform::from_xyz(1.5, 1.5, 0.);
    register_room(
        &small_room,
        &small_room_tfm,
        SMALL_ROOM,
        SMALL_ROOM_GEOM,
        rr::xbus::SmallRoom,
    )?;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(small_room),
        material: materials.add(room_mat),
        transform: small_room_tfm,
        ..default()
    });

    // Portal in between
    let portal = Mesh::from(shape::Box::new(0.75, 1.5, 1.));
    let tfm =
        Transform::from_xyz(0., 0.75, 0.).with_rotation(Quat::from_axis_angle(Vec3::Y, PI / 2.));
    set_portal(
        INNER_PORTAL,
        AkPortalParams {
            transform: to_ak_transform(&tfm),
            extent: AkExtent {
                halfWidth: 0.5,
                halfHeight: 0.75,
                halfDepth: 0.375,
            },
            enabled: true,
            name: "InnerPortal".to_string(),
            front_room: BIG_ROOM,
            back_room: SMALL_ROOM,
        },
    )?;
    set_portal_obstruction_and_occlusion(INNER_PORTAL, 0., 0.)?;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(portal),
        material: materials.add(Color::GREEN.into()),
        transform: tfm,
        ..default()
    });

    // Portal on big room to outdoor
    let portal = Mesh::from(shape::Box::new(1., 1.5, 0.75));
    let tfm = Transform::from_xyz(-6., 0.75, -2.5);
    set_portal(
        OUTER_PORTAL,
        AkPortalParams {
            transform: to_ak_transform(&tfm),
            extent: AkExtent {
                halfWidth: 0.5,
                halfHeight: 0.75,
                halfDepth: 0.375,
            },
            enabled: true,
            name: "OuterPortal".to_string(),
            front_room: BIG_ROOM,
            back_room: OUTDOOR_ROOM_ID,
        },
    )?;
    set_portal_obstruction_and_occlusion(OUTER_PORTAL, 0., 0.)?;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(portal),
        material: materials.add(Color::GREEN.into()),
        transform: tfm,
        ..default()
    });

    set_game_object_in_room(THE_GAME_OBJECT, SMALL_ROOM)?;

    // Emitter mesh
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 0.2,
            ..default()
        })),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_xyz(2., 1.5, 1.5),
        ..default()
    });

    // Setup light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: Default::default(),
        ..default()
    });

    Ok(())
}

#[cfg_attr(target_os = "linux", allow(unused_variables))]
fn init_sound_engine(windows: Res<Windows>) -> Result<(), AkResult> {
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
        format!("examples/WwiseProject/GeneratedSoundBanks/{}", platform),
    )?;
    stream_mgr::set_current_language("English(US)")?;

    // init soundengine
    {
        let mut platform_settings = AkPlatformInitSettings::default();

        #[cfg(windows)]
        // Find the Bevy window and register it as owner of the sound engine
        if let Some(w) = windows.iter().next() {
            use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

            platform_settings.h_wnd.store(
                match unsafe { w.raw_window_handle().get_handle().raw_window_handle() } {
                    #[cfg(windows)]
                    RawWindowHandle::Win32(h) => h.hwnd,
                    other => {
                        panic!("Unexpected window handle: {:?}", other)
                    }
                },
                std::sync::atomic::Ordering::SeqCst,
            );
        }

        sound_engine::init(&mut setup_example_dll_path(), &mut platform_settings)?;
    }

    // init music
    music_engine::init(&mut settings::AkMusicSettings::default())?;

    // init spatial
    spatial_audio::init(&AkSpatialAudioInitSettings::default())?;

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
    spatial_audio::term();

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
        .with_plugin_dll_path(path.join("binnnn").into_os_string().into_string().unwrap())
}
