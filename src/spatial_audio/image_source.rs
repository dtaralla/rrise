/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

//! Helper functions for passing game data to the Wwise Reflect plug-in.
//! Use this API for detailed placement of reflection image sources.
//!
//! These functions are low-level and useful when your game engine already implements a geometrical
//! approach to sound propagation such as an image-source or a ray tracing algorithm.
//!
//! Functions of [geometry](crate::spatial_audio::geometry) are preferred and easier to use with the Wwise Reflect plug-in.

use crate::bindings::root::AK::SpatialAudio::*;
use crate::spatial_audio::{AkImageSourceID, AkImageSourceParams, AkImageSourceTexture, AkRoomID};
use crate::{ak_call_result, AkAuxBusID, AkGameObjectID, AkResult};

#[derive(Debug)]
pub struct AkImageSourceSettings {
    /// Image source parameters.
    pub params: AkImageSourceParams,
    /// Acoustic texture that goes with this image source.
    pub texture: AkImageSourceTexture,
}

/// Add or update an individual image source for processing via the AkReflect plug-in.  Use this API for detailed placement of
/// reflection image sources, whose positions have been determined by the client, such as from the results of a ray cast, computation or by manual placement.  One possible
/// use case is generating reflections that originate far enough away that they can be modeled as a static point source, for example, off of a distant mountain.
/// The SpatialAudio API manages image sources added via [`set_image_source()`] and sends them to the AkReflect plug-in that is on the aux bus with ID `ak_reflect_aux_bux_id`.
/// The image source will apply only to the the game object specified by `game_obj_id`.
/// [`set_image_source()`] takes a room ID to indicate which room the reflection is logically part of, even though the position of the image source may be outside of the extents of the room.  
/// This ID is used as a filter, so that it is not possible to hear reflections for rooms that the emitter and listener are not both inside.  To use this feature, the emitter's and listener's rooms must also be
/// specified using [`set_game_object_in_room`](crate::spatial_audio::room::set_game_object_in_room). If you are not using the rooms and portals API, or the image source is not associated with a room, pass [`OUTDOORS_ROOM_ID`].
///
/// *Notes*
/// >- The [`AkImageSourceSettings`] struct passed in `info` must contain a unique image source ID to be able to identify this image source across frames and when updating and/or removing it later.  
/// Each instance of AkReflect has its own set of data, so you may reuse ID, if desired, as long as [`game_obj_id`] and [`ak_reflect_aux_bux_id`] are different.
/// >- Early reflection send level and bus in the authoring tool do not apply to image sources set with [`set_image_source()`]. When using this function, the Reflect bus and send level
/// may only be set programmatically. Also, it is not possible to use the geometric reflections API on the same aux bus and game object. If using the geometric reflections API and the [`set_image_source()`] API in conjunction, be sure to specify an
/// aux bus to [`set_image_source()`] that is unique from the aux bus(es) defined in the authoring tool, and from those passed to [`set_early_reflection_aux_sends`](crate::spatial_audio::room::set_early_reflection_aux_sends).
/// >- For proper operation with AkReflect and the SpatialAudio API, any aux bus using AkReflect should have 'Listener Relative Routing' checked and the 3D Spatialization set to None in the Wwise authoring tool.
///
/// *See also*
/// >- [`remove_image_source`]
/// >- [`clear_image_sources`]
/// >- [`set_game_object_in_room`](crate::spatial_audio::room::set_game_object_in_room)
/// >- [`set_early_reflection_aux_sends`](crate::spatial_audio::room::set_early_reflection_aux_sends)
pub fn set_image_source(
    src_id: AkImageSourceID,
    info: AkImageSourceSettings,
    ak_reflect_aux_bux_id: AkAuxBusID,
    room_id: AkRoomID,
    game_obj_id: AkGameObjectID,
) -> Result<(), AkResult> {
    let mut raw_info =
        unsafe { std::mem::zeroed::<crate::bindings::root::AkImageSourceSettings>() };
    raw_info.params = info.params;
    raw_info.texture = info.texture;

    ak_call_result![SetImageSource(
        src_id,
        &raw_info,
        ak_reflect_aux_bux_id,
        room_id.into(),
        game_obj_id
    )]?;
    Ok(())
}

/// Remove an individual reflection image source that was previously added via [`set_image_source()`].
///
/// *See also*
/// >- [`set_image_source`]
/// >- [`clear_image_sources`]
pub fn remove_image_source(
    src_id: AkImageSourceID,
    ak_reflect_aux_bux_id: AkAuxBusID,
    game_obj_id: AkGameObjectID,
) -> Result<(), AkResult> {
    ak_call_result![RemoveImageSource(
        src_id,
        ak_reflect_aux_bux_id,
        game_obj_id
    )]?;
    Ok(())
}

/// Remove all image sources matching `ak_reflect_aux_bux_id` and `game_obj_id` that were previously added via [`set_image_source`].
/// Both `ak_reflect_aux_bux_id` and `game_obj_id` can be treated as wild cards matching all aux buses and/or all game object, by
/// passing [`AK_INVALID_AUX_ID`](crate::AK_INVALID_AUX_ID) and/or [`AK_INVALID_GAME_OBJECT`](crate::AK_INVALID_GAME_OBJECT), respectively.
///
/// *See also*
/// >- [`set_image_source`]
/// >- [`remove_image_source`]
pub fn clear_image_sources(
    ak_reflect_aux_bux_id: AkAuxBusID,
    game_obj_id: AkGameObjectID,
) -> Result<(), AkResult> {
    ak_call_result![ClearImageSources(ak_reflect_aux_bux_id, game_obj_id)]?;
    Ok(())
}
