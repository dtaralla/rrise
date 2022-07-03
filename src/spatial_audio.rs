/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use crate::bindings::root::AK::SpatialAudio::*;
use crate::settings::AkSpatialAudioInitSettings;
use crate::{ak_call_result, AkGameObjectID, AkResult};

pub mod geometry;
pub mod image_source;
pub mod room;

pub use crate::bindings::root::AkAcousticSurface;
pub use crate::bindings::root::AkExtent;
pub use crate::bindings::root::AkImageSourceID;
pub use crate::bindings::root::AkImageSourceParams;
pub use crate::bindings::root::AkImageSourceTexture;
pub use crate::bindings::root::AkTriangle;
pub use crate::bindings::root::AkVertex;

/// The outdoor room ID. This room is created automatically and is typically used for outdoors, i.e.
/// when not in a room.
pub const OUTDOOR_ROOM_ID: AkRoomID = u64::MAX;

pub type AkSpatialAudioID = u64;
pub type AkPortalID = u64;
pub type AkGeometrySetID = u64;
impl From<AkSpatialAudioID> for crate::bindings::root::AkSpatialAudioID {
    fn from(id: AkSpatialAudioID) -> Self {
        crate::bindings::root::AkSpatialAudioID { id }
    }
}

pub type AkRoomID = AkSpatialAudioID;
impl From<AkRoomID> for crate::bindings::root::AkRoomID {
    fn from(id: AkRoomID) -> Self {
        crate::bindings::root::AkRoomID {
            _base: crate::bindings::root::AkSpatialAudioID { id },
        }
    }
}

impl Default for AkExtent {
    fn default() -> Self {
        Self {
            halfWidth: 0.,
            halfHeight: 0.,
            halfDepth: 0.,
        }
    }
}

/// Initialize the SpatialAudio API.
pub fn init(init_settings: &AkSpatialAudioInitSettings) -> Result<(), AkResult> {
    ak_call_result![Init(init_settings)]?;
    Ok(())
}

/// Terminates Spatial Audio.
pub fn term() {}

/// Assign a game object as the Spatial Audio listener. There can be only one Spatial Audio listener registered at any given time; `game_object_id` will replace any previously set Spatial Audio listener.
/// The game object passed in must be registered by the client, at some point, for sound to be heard.  It is not necessary to be registered at the time of calling this function.
/// If not listener is explicitly registered to spatial audio, then a default listener (set via [`sound_engine::set_default_listeners`](crate::sound_engine::set_default_listeners)) is selected. If the are no default listeners, or there are more than one
/// default listeners, then it is necessary to call `register_listener()` to specify which listener to use with Spatial Audio.
pub fn register_listener(game_object_id: AkGameObjectID) -> Result<(), AkResult> {
    ak_call_result![RegisterListener(game_object_id)]?;
    Ok(())
}

/// Unregister a game object as a listener in the SpatialAudio API; clean up Spatial Audio listener data associated with game_object_id.  
/// If game_object_id is the current registered listener, calling this function will clear the Spatial Audio listener and
/// Spatial Audio features will be disabled until another listener is registered.
/// This function is optional - listener are automatically unregistered when their game object is deleted in the sound engine.
///
/// *See also*
/// > - [`register_listener`]
pub fn unregister_listener(game_object_id: AkGameObjectID) -> Result<(), AkResult> {
    ak_call_result![UnregisterListener(game_object_id)]?;
    Ok(())
}

/// Define a inner and outer radius around each sound position for a specified game object.
/// The radii are used in spread and distance calculations, simulating a radial sound source.
/// When applying attenuation curves, the distance between the listener and the inner sphere (defined by the sound position and `inner_radius`) is used.
/// The spread for each sound position is calculated as follows:
/// > - If the listener is outside the outer radius, then the spread is defined by the area that the sphere takes in the listener field of view. Specifically, this angle is calculated as `2.0 * asinf(outer_radius / distance)`, where distance is the distance between the listener and the sound position.
/// > - When the listener intersects the outer radius (the listener is exactly `outer_radius` units away from the sound position), the spread is exactly 50%.
/// > - When the listener is in between the inner and outer radius, the spread interpolates linearly from 50% to 100% as the listener transitions from the outer radius towards the inner radius.
/// > - If the listener is inside the inner radius, the spread is 100%.
///
/// *Note*
///
/// Transmission and diffraction calculations in Spatial Audio always use the center of the sphere (the position(s) passed into [`sound_engine::set_position`](crate::sound_engine::set_position) or [`sound_engine::set_multiple_positions`](crate::sound_engine::set_multiple_positions) for raycasting.
/// To obtain accurate diffraction and transmission calculations for radial sources, where different parts of the volume may take different paths through or around geometry,
/// it is necessary to pass multiple sound positions into [`sound_engine::set_multiple_positions`](crate::sound_engine::set_multiple_positions) to allow the engine to 'sample' the area at different points.
///
/// *See also*
/// > - [`sound_engine::set_position`](crate::sound_engine::set_position)
/// > - [`sound_engine::set_multiple_positions`](crate::sound_engine::set_multiple_positions)
pub fn set_game_object_radius(
    game_object_id: AkGameObjectID,
    outer_radius: f32,
    inner_radius: f32,
) -> Result<(), AkResult> {
    ak_call_result![SetGameObjectRadius(
        game_object_id,
        outer_radius,
        inner_radius
    )]?;
    Ok(())
}
