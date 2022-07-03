/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Invalid game object (may also mean all game objects)
pub const AK_INVALID_GAME_OBJECT: root::AkAudioObjectID = u64::MAX;

/// Invalid audio object ID
pub const AK_INVALID_AUDIO_OBJECT_ID: root::AkAudioObjectID = u64::MAX;

/// A game object ID that is in the reserved range, used for 'outdoor' rooms, i.e. when not in a room.
///
/// *Warning*
///
/// This `AkGameObjectID` is the underlying game object ID of the outdoor room, and should not be
/// confused with the actual outdoor room's ID, [`spatial_audio::OUTDOOR_ROOM_ID`](crate::spatial_audio::OUTDOOR_ROOM_ID).
pub const OUTDOORS_GAME_OBJECT_ID: root::AkAudioObjectID = u64::MAX - 3;
