/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

//! Geometry API for early reflection processing using Wwise Reflect.

use crate::bindings::root::AK::SpatialAudio::*;
use crate::spatial_audio::{
    AkAcousticSurface, AkGeometrySetID, AkRoomID, AkTriangle, AkVertex, OUTDOOR_ROOM_ID,
};
use crate::{ak_call_result, AkResult};
use num::ToPrimitive;

#[derive(Debug)]
pub struct AkGeometryParams {
    /// Pointer to an array of AkTriangle structures.
    /// This array will be copied into spatial audio memory and will not be accessed after [`set_geometry`] returns.
    ///
    /// >- [`AkTriangle`]
    /// >- [`SetGeometry`]
    /// >- [`RemoveGeometry`]
    pub triangles: Vec<AkTriangle>,

    /// Pointer to an array of AkVertex structures.
    /// This array will be copied into spatial audio memory and will not be accessed after [`set_geometry`] returns.
    ///
    /// >- [`AkVertex`]
    /// >- [`SetGeometry`]
    /// >- [`RemoveGeometry`]
    pub vertices: Vec<AkVertex>,

    /// Pointer to an array of AkAcousticSurface structures.
    /// This array will be copied into spatial audio memory and will not be accessed after [`set_geometry`] returns.
    ///
    /// >- [`AkAcousticSurface`]
    /// >- [`SetGeometry`]
    /// >- [`RemoveGeometry`]
    pub surfaces: Vec<AkAcousticSurface>,

    /// Associate this geometry set with the room `room_id`. Associating a geometry set with a particular room will
    /// limit the scope in which the geometry is visible/accessible. `room_id` can be left as default ([OUTDOOR_ROOM_ID]), in which
    /// case this geometry set will have a global scope. It is recommended to associate geometry with a room when
    /// the geometry is
    /// 1. fully contained within the room (ie. not visible to other rooms except by portals),
    /// 2. the room does not share geometry with other rooms.
    ///
    /// Doing so reduces the search space for ray casting performed by reflection and diffraction calculations. Take
    /// note that once one or more geometry sets are associated with a room, that room will no longer be able to
    /// access geometry that is in the global scope.
    /// >- [`spatial_audio::set_room`](crate::spatial_audio::set_room)
    /// >- [`AkRoomParams`]
    pub room_id: AkRoomID,

    /// Switch to enable or disable geometric diffraction for this Geometry.
    pub enable_diffraction: bool,

    /// Switch to enable or disable geometric diffraction on boundary edges for this Geometry. Boundary edges are
    /// edges that are connected to only one triangle.
    pub enable_diffraction_on_boundary_edges: bool,

    /// Switch to enable or disable the use of the triangles for this Geometry. When enabled, the geometry triangles
    /// are indexed for ray computation and used to computed reflection and diffraction.
    ///
    /// Set `enable_triangles` to false when using a geometry set only to describe a room, and not for reflection
    /// and diffraction calculation.
    /// >- [`AkRoomParams`]
    pub enable_triangles: bool,
}

impl Default for AkGeometryParams {
    fn default() -> Self {
        Self {
            triangles: vec![],
            vertices: vec![],
            surfaces: vec![],
            room_id: OUTDOOR_ROOM_ID,
            enable_diffraction: false,
            enable_diffraction_on_boundary_edges: false,
            enable_triangles: true,
        }
    }
}

/// Add or update a set of geometry from the SpatialAudio module for geometric reflection and diffraction
/// processing. A geometry set is a logical set of vertices, triangles, and acoustic surfaces, which are
/// referenced by the same [`AkGeometrySetID`]. The ID (`geom_set_id`) must be unique and is also chosen
/// by the client in a manner similar to AkGameObjectID's.
///
/// *See also*
/// > - [`AkGeometryParam`]
/// > - [`RemoveGeometry`]
pub fn set_geometry(
    geom_set_id: AkGeometrySetID,
    mut params: AkGeometryParams,
) -> Result<(), AkResult> {
    ak_call_result![SetGeometry(
        geom_set_id.into(),
        &crate::bindings::root::AkGeometryParams {
            Triangles: params.triangles.as_mut_ptr(),
            NumTriangles: params
                .triangles
                .len()
                .to_u16()
                .unwrap_or_else(|| panic!("Too many triangles for geom_set_id {}", geom_set_id)),
            Vertices: params.vertices.as_mut_ptr(),
            NumVertices: params
                .vertices
                .len()
                .to_u16()
                .unwrap_or_else(|| panic!("Too many vertices for geom_set_id {}", geom_set_id)),
            Surfaces: params.surfaces.as_mut_ptr(),
            NumSurfaces: params
                .surfaces
                .len()
                .to_u16()
                .unwrap_or_else(|| panic!("Too many surfaces for geom_set_id {}", geom_set_id)),
            RoomID: params.room_id.into(),
            EnableDiffraction: params.enable_diffraction,
            EnableDiffractionOnBoundaryEdges: params.enable_diffraction_on_boundary_edges,
            EnableTriangles: params.enable_triangles,
        }
    )]?;
    Ok(())
}

/// Remove a set of geometry to the SpatialAudio API.
///
/// *See also*
/// > - [`set_geometry`]
pub fn remove_geometry(geom_set_id: AkGeometrySetID) -> Result<(), AkResult> {
    ak_call_result![RemoveGeometry(geom_set_id.into())]?;
    Ok(())
}
