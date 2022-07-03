/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

//! Sound Propagation API using rooms and portals.

use crate::bindings::root::AK::SpatialAudio::{
    RemovePortal, RemoveRoom, SetGameObjectInRoom, SetPortal, SetPortalObstructionAndOcclusion,
    SetRoom,
};
use crate::spatial_audio::{AkExtent, AkGeometrySetID, AkPortalID, AkRoomID, OUTDOOR_ROOM_ID};
use crate::{
    ak_call_result, to_os_char, AkAuxBusID, AkGameObjectID, AkResult, AkTransform, AkVector,
};

/// Parameters passed to [`set_room`]
#[derive(Debug)]
pub struct AkRoomParams {
    #[doc = " Room Orientation. Up and Front must be orthonormal."]
    #[doc = " Room orientation has an effect when the associated aux bus (see ReverbAuxBus) is set with 3D Spatialization in Wwise, as 3D Spatialization implements relative rotation of the emitter (room) and listener."]
    pub front: AkVector,
    #[doc = " Room Orientation. Up and Front must be orthonormal."]
    #[doc = " Room orientation has an effect when the associated aux bus (see ReverbAuxBus) is set with 3D Spatialization in Wwise, as 3D Spatialization implements relative rotation of the emitter (room) and listener."]
    pub up: AkVector,
    #[doc = " The reverb aux bus that is associated with this room."]
    #[doc = " When Spatial Audio is told that a game object is in a particular room via SetGameObjectInRoom, a send to this aux bus will be created to model the reverb of the room."]
    #[doc = " Using a combination of Rooms and Portals, Spatial Audio manages which game object the aux bus is spawned on, and what control gain is sent to the bus."]
    #[doc = " When a game object is inside a connected portal, as defined by the portal's orientation and extent vectors, both this aux send and the aux send of the adjacent room are active."]
    #[doc = " Spatial audio modulates the control value for each send based on the game object's position, in relation to the portal's z-azis and extent, to crossfade the reverb between the two rooms."]
    #[doc = " If more advanced control of reverb is desired, SetGameObjectAuxSendValues can be used to add additional sends on to a game object."]
    #[doc = " - \\ref AK::SpatialAudio::SetGameObjectInRoom"]
    #[doc = " - \\ref AK::SoundEngine::SetGameObjectAuxSendValues"]
    pub reverb_aux_bus: AkAuxBusID,
    #[doc = " The reverb control value for the send to ReverbAuxBus. Valid range: (0.f-1.f)"]
    #[doc = " Can be used to implement multiple rooms that share the same aux bus, but have different reverb levels."]
    pub reverb_level: f32,
    #[doc = " Level to set when modeling transmission through walls. Transmission is modeled only when the sound emitted enables diffraction and there is no direct line of sight from the emitter to the listener."]
    #[doc = " This transmission loss value is only applied when the listener and the emitter are in different rooms; it is taken as the maximum between the emitter's room's transmission loss value and the listener's room's transmission loss value."]
    #[doc = " If there is geometry in between the listener and the emitter, then the transmission loss value assigned to surfaces hit by the ray between the emitter and listener is also taken into account."]
    #[doc = " The maximum of all the surfaces' transmission loss value (see \\c AkAcousticSurface), and the room's transmission loss value is used to render the transmission path."]
    #[doc = " Valid range: (0.f-1.f)"]
    #[doc = " - \\ref AkAcousticSurface"]
    pub transmission_loss: f32,
    #[doc = " Name used to identify room (optional)"]
    pub name: String,
    #[doc = " Send level for sounds that are posted on the room game object; adds reverb to ambience and room tones. Valid range: (0.f-1.f).  Set to a value greater than 0 to have spatial audio create a send on the room game object,"]
    #[doc = " where the room game object itself is specified as the listener and ReverbAuxBus is specified as the aux bus. A value of 0 disables the aux send. This should not be confused with ReverbLevel, which is the send level"]
    #[doc = " for spatial audio emitters sending to the room game object."]
    #[doc = " \\aknote The room game object can be accessed though the ID that is passed to \\c SetRoom() and the \\c AkRoomID::AsGameObjectID() method.  Posting an event on the room game object leverages automatic room game object placement"]
    #[doc = "\tby spatial audio so that when the listener is inside the room, the sound comes from all around the listener, and when the listener is outside the room, the sound comes from the portal(s). Typically, this would be used for"]
    #[doc = " surround ambiance beds or room tones. Point source sounds should use separate game objects that are registered as spatial audio emitters."]
    #[doc = " \\sa"]
    #[doc = " - \\ref AkRoomParams::RoomGameObj_KeepRegistered"]
    #[doc = " - \\ref AkRoomID"]
    pub room_game_obj_aux_send_level_to_self: f32,
    #[doc = " If set to true, the room game object will be registered on calling \\c SetRoom(), and not released untill the room is deleted or removed with \\c RemoveRoom(). If set to false, spatial audio will register"]
    #[doc = " the room object only when it is needed by the sound propagation system for the purposes of reverb, and will unregister the game object when all reverb tails have finished."]
    #[doc = " If the game intends to post events on the room game object for the purpose of ambiance or room tones, RoomGameObj_KeepRegistered should be set to true."]
    #[doc = " \\aknote The room game object can be accessed though the ID that is passed to \\c SetRoom() and the \\c AkRoomID::AsGameObjectID() method.  Posting an event on the room game object leverages automatic room game object placement"]
    #[doc = "\tby spatial audio so that when the listener is inside the room, the sound comes from all around the listener, and when the listener is outside the room, the sound comes from the portal(s). Typically, this would be used for"]
    #[doc = " surround ambiance beds or room tones. Point source sounds should use separate game objects that are registered as spatial audio emitters."]
    #[doc = " \\sa"]
    #[doc = " - \\ref AkRoomParams::RoomGameObj_AuxSendLevelToSelf"]
    #[doc = " - \\ref AkRoomID"]
    pub room_game_obj_keep_registered: bool,
    #[doc = " Associate this room with the geometry set \\c GeometryID, describing the shape of the room. When a room is associated with a geometry set, the vertices are used to compute the spread value for room transmission."]
    #[doc = " The vertices are used for computing an oriented bounding box for the room where the orientation of the bounding box is given by the Up and Front vectors. The center of the room is defined as the oriented bounding box center."]
    #[doc = " The extent of the bounding box is computed from the geometry set's vertices projected on to the orientation axes."]
    #[doc = " \\aknote If the geometry set is only to be used for the room and not for reflection and diffraction calculation, then make sure to set \\c AkGeometryParams::EnableTriangles to false."]
    #[doc = " \\sa"]
    #[doc = " - \\ref spatial_audio_roomsportals_apiconfigroomgeometry"]
    #[doc = " - \\ref AkGeometryParams"]
    pub geometry_id: AkGeometrySetID,
}

/// Parameters passed to [`set_portal`]
#[derive(Debug)]
pub struct AkPortalParams {
    #[doc = " Portal's position and orientation in the 3D world."]
    #[doc = " Position vector is the center of the opening."]
    #[doc = " OrientationFront vector must be unit-length and point along the normal of the portal, and must be orthogonal to Up. It defines the local positive-Z dimension (depth/transition axis) of the portal, used by Extent."]
    #[doc = " OrientationTop vector must be unit-length and point along the top of the portal (tangent to the wall), must be orthogonal to Front. It defines the local positive-Y direction (height) of the portal, used by Extent."]
    pub transform: AkTransform,
    #[doc = " Portal extent. Defines the dimensions of the portal relative to its center; all components must be positive numbers. The local right and up dimensions are used in diffraction calculations,"]
    #[doc = " whereas the front dimension defines a depth value which is used to implement smooth transitions between rooms. It is recommended that users experiment with different portal depths to find a value"]
    #[doc = " that results in appropriately smooth transitions between rooms. Extent dimensions must be positive."]
    pub extent: AkExtent,
    #[doc = " Whether or not the portal is active/enabled. For example, this parameter may be used to simulate open/closed doors."]
    #[doc = " Portal diffraction is simulated when at least one portal exists and is active between an emitter and the listener."]
    pub enabled: bool,
    #[doc = " Name used to identify portal (optional)."]
    pub name: String,
    #[doc = " ID of the room to which the portal connects, in the direction of the Front vector.  If a room with this ID has not been added via AK::SpatialAudio::SetRoom,"]
    #[doc = " a room will be created with this ID and with default AkRoomParams.  If you would later like to update the AkRoomParams, simply call AK::SpatialAudio::SetRoom again with this same ID."]
    #[doc = "\t- \\ref AK::SpatialAudio::SetRoom"]
    #[doc = "\t- \\ref AK::SpatialAudio::RemoveRoom"]
    #[doc = " - \\ref AkRoomParams"]
    pub front_room: AkRoomID,
    #[doc = " ID of the room to which the portal connects, in the direction opposite to the Front vector. If a room with this ID has not been added via AK::SpatialAudio::SetRoom,"]
    #[doc = " a room will be created with this ID and with default AkRoomParams.  If you would later like to update the AkRoomParams, simply call AK::SpatialAudio::SetRoom again with this same ID."]
    #[doc = "\t- \\ref AK::SpatialAudio::SetRoom"]
    #[doc = "\t- \\ref AK::SpatialAudio::RemoveRoom"]
    #[doc = " - \\ref AkRoomParams"]
    pub back_room: AkRoomID,
}

impl Default for AkPortalParams {
    fn default() -> Self {
        Self {
            transform: AkTransform::default(),
            extent: AkExtent::default(),
            enabled: false,
            name: "".to_string(),
            front_room: OUTDOOR_ROOM_ID,
            back_room: OUTDOOR_ROOM_ID,
        }
    }
}

/// Add or update a room. Rooms are used to connect portals and define an orientation for oriented reverbs. This function may be called multiple times with the same ID to update the parameters of the room.
///
/// *Warning*
/// > The ID (`room_id`) must be chosen in the same manner as [`AkGameObjectID`]'s, as they are in the same ID-space. The spatial audio lib manages the
/// registration/unregistration of internal game objects for rooms that use these IDs and, therefore, must not collide.
/// Also, the room ID must not be in the reserved range `u64::MAX-32` to `u64::MAX-2` inclusively. You may, however, explicitly add the default room ID
/// [`OUTDOOR_ROOM_ID`](crate::spatial_audio::OUTDOOR_ROOM_ID) in order to customize its `AkRoomParams`, to provide a valid auxiliary bus, for example.
///
/// *See also*
/// >- [`AkRoomID`]
/// >- [`AkRoomParams`]
/// >- [`remove_room`]
pub fn set_room(room_id: AkRoomID, params: AkRoomParams) -> Result<(), AkResult> {
    let mut raw_params = unsafe { std::mem::zeroed::<crate::bindings::root::AkRoomParams>() };
    raw_params.Front = params.front;
    raw_params.Up = params.up;
    raw_params.ReverbAuxBus = params.reverb_aux_bus;
    raw_params.ReverbLevel = params.reverb_level;
    raw_params.TransmissionLoss = params.transmission_loss;
    raw_params.RoomGameObj_AuxSendLevelToSelf = params.room_game_obj_aux_send_level_to_self;
    raw_params.RoomGameObj_KeepRegistered = params.room_game_obj_keep_registered;
    raw_params.GeometryID = params.geometry_id.into();

    let pin_bytes = to_os_char(params.name.as_str());
    raw_params.strName._base._base.bOwner = false;
    raw_params.strName._base._base.pStr = pin_bytes.as_ptr();

    ak_call_result![SetRoom(room_id.into(), &raw_params)]?;
    Ok(())
}

/// Remove a room.
///
/// *See also*
/// >- [`AkRoomID`]
/// >- [`set_room`]
pub fn remove_room(room_id: AkRoomID) -> Result<(), AkResult> {
    ak_call_result![RemoveRoom(room_id.into())]?;
    Ok(())
}

/// Add or update an acoustic portal. A portal is an opening that connects two or more rooms to simulate the transmission of reverberated (indirect) sound between the rooms.
/// This function may be called multiple times with the same ID to update the parameters of the portal. The ID (`portal_id`) must be chosen in the same manner as [`AkGameObjectID`]'s,
/// as they are in the same ID-space. The spatial audio lib manages the registration/unregistration of internal game objects for portals that use these IDs, and therefore must not collide.
///
/// *See also*
/// >- [`AkPortalID`]
/// >- [`AkPortalParams`]
/// >- [`remove_portal`]
pub fn set_portal(portal_id: AkPortalID, params: AkPortalParams) -> Result<(), AkResult> {
    let mut raw_params = unsafe { std::mem::zeroed::<crate::bindings::root::AkPortalParams>() };
    raw_params.Transform = params.transform;
    raw_params.Extent = params.extent;
    raw_params.bEnabled = params.enabled;
    raw_params.FrontRoom = params.front_room.into();
    raw_params.BackRoom = params.back_room.into();

    let pin_bytes = to_os_char(params.name.as_str());
    raw_params.strName._base._base.bOwner = false;
    raw_params.strName._base._base.pStr = pin_bytes.as_ptr();

    ak_call_result![SetPortal(portal_id.into(), &raw_params)]?;
    Ok(())
}

/// Remove a portal.
///
/// *See also*
/// >- [`AkPortalID`]
/// >- [`set_portal`]
pub fn remove_portal(portal_id: AkPortalID) -> Result<(), AkResult> {
    ak_call_result![RemovePortal(portal_id.into())]?;
    Ok(())
}

pub fn set_portal_obstruction_and_occlusion(
    portal_id: AkPortalID,
    obs: f32,
    occ: f32,
) -> Result<(), AkResult> {
    ak_call_result![SetPortalObstructionAndOcclusion(portal_id.into(), obs, occ)]?;
    Ok(())
}

/// Set the room that the game object is currently located in - usually the result of a containment test performed by the client. The room must have been registered with [`set_room`].
/// Setting the room for a game object provides the basis for the sound propagation service, and also sets which room's reverb aux bus to send to.  The sound propagation service traces the path
/// of the sound from the emitter to the listener, and calculates the diffraction as the sound passes through each portal.  The portals are used to define the spatial location of the diffracted and reverberated
/// audio.
///
/// *See also*
/// >- [`set_room`]
/// >- [`remove_room`]
pub fn set_game_object_in_room(
    game_obj_id: AkGameObjectID,
    room_id: AkRoomID,
) -> Result<(), AkResult> {
    ak_call_result![SetGameObjectInRoom(game_obj_id, room_id.into())]?;
    Ok(())
}
