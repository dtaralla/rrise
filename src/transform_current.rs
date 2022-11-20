/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use crate::{AkVector64, AkWorldTransform};

impl From<[f64; 3]> for AkVector64 {
    /// In AkVectors, Y is the up component and Z is the forward component.
    ///
    /// Assumes values in `v` are in XYZ order.
    fn from(v: [f64; 3]) -> Self {
        Self {
            X: v[0],
            Y: v[1],
            Z: v[2],
        }
    }
}

impl From<[f64; 3]> for AkWorldTransform {
    /// Creates an AkWorldTransform at position `p` with default orientation (up pointing up, forward
    /// pointing forward).
    ///
    /// Assumes values in `v` are in XYZ order.
    fn from(p: [f64; 3]) -> Self {
        Self {
            position: AkVector64::from(p),
            ..Default::default()
        }
    }
}

impl From<AkVector64> for AkWorldTransform {
    /// Creates an AkWorldTransform at position `p` with default orientation (up pointing up, forward
    /// pointing forward).
    fn from(p: AkVector64) -> Self {
        Self {
            position: p,
            ..Default::default()
        }
    }
}

impl Default for AkVector64 {
    /// The nul vector `[0, 0, 0]`.
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl AkVector64 {
    /// The nul vector `[0, 0, 0]`.
    pub fn new() -> Self {
        Self::default()
    }

    /// The vector `[value, value, value]`.
    pub fn splat<T: Into<f64> + Copy>(value: T) -> Self {
        Self {
            X: value.into(),
            Y: value.into(),
            Z: value.into(),
        }
    }
}

impl Default for AkWorldTransform {
    /// Creates an AkTransform at `[0, 0, 0]` with default orientation (up pointing up, forward
    /// pointing forward).
    ///
    /// *See also*
    /// > - [AkWorldTransform::new]
    fn default() -> Self {
        Self {
            position: AkVector64::default(),
            orientationFront: AkVector::from([0., 0., 1.]),
            orientationTop: AkVector::from([0., 1., 0.]),
        }
    }
}

impl AkWorldTransform {
    /// Creates the default AkWorldTransform.
    ///
    /// *See also*
    /// > - [AkWorldTransform::default]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an AkWorldTransform at position `p` with default orientation (up pointing up, forward
    /// pointing forward).
    pub fn from_position<T: Into<AkVector64>>(p: T) -> Self {
        AkWorldTransform::from(p.into())
    }
}
