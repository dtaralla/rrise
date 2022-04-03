/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

#[cfg(not(wwconfig = "release"))]
pub mod communication;
pub mod memory_mgr;
pub mod settings;
pub mod sound_engine;
pub mod stream_mgr;

mod bindings;
mod bindings_static_plugins;
mod error;

pub use error::*;

pub use bindings::root::AkAcousticTextureID;
pub use bindings::root::AkArgumentValueID;
pub use bindings::root::AkAudioObjectID;
pub use bindings::root::AkAuxBusID;
pub use bindings::root::AkBankID;
pub use bindings::root::AkCallbackType;
pub use bindings::root::AkChannelMask;
pub use bindings::root::AkCodecID;
pub use bindings::root::AkDataCompID;
pub use bindings::root::AkDataInterleaveID;
pub use bindings::root::AkDataTypeID;
pub use bindings::root::AkDeviceID;
pub use bindings::root::AkFileID;
pub use bindings::root::AkGameObjectID;
pub use bindings::root::AkImageSourceID;
pub use bindings::root::AkLPFType;
pub use bindings::root::AkMemPoolId;
pub use bindings::root::AkModulatorID;
pub use bindings::root::AkOutputDeviceID;
pub use bindings::root::AkPipelineID;
pub use bindings::root::AkPitchValue;
pub use bindings::root::AkPlayingID;
pub use bindings::root::AkPluginID;
pub use bindings::root::AkPluginParamID;
pub use bindings::root::AkPortNumber;
pub use bindings::root::AkPriority;
pub use bindings::root::AkRayID;
pub use bindings::root::AkRtpcID;
pub use bindings::root::AkRtpcValue;
pub use bindings::root::AkStateGroupID;
pub use bindings::root::AkStateID;
pub use bindings::root::AkSwitchGroupID;
pub use bindings::root::AkSwitchStateID;
pub use bindings::root::AkTimeMs;
pub use bindings::root::AkTriggerID;
pub use bindings::root::AkUniqueID;
pub use bindings::root::AkVolumeValue;
pub use bindings::root::AKRESULT;

pub use bindings::root::AK_DEFAULT_BANK_IO_PRIORITY;
pub use bindings::root::AK_DEFAULT_BANK_THROUGHPUT;
pub use bindings::root::AK_DEFAULT_POOL_ID;
pub use bindings::root::AK_DEFAULT_PRIORITY;
pub use bindings::root::AK_DEFAULT_SWITCH_STATE;
pub use bindings::root::AK_FALLBACK_ARGUMENTVALUE_ID;
pub use bindings::root::AK_INVALID_AUX_ID;
pub use bindings::root::AK_INVALID_BANK_ID;
pub use bindings::root::AK_INVALID_CHANNELMASK;
pub use bindings::root::AK_INVALID_DEVICE_ID;
pub use bindings::root::AK_INVALID_FILE_ID;
pub use bindings::root::AK_INVALID_OUTPUT_DEVICE_ID;
pub use bindings::root::AK_INVALID_PIPELINE_ID;
pub use bindings::root::AK_INVALID_PLAYING_ID;
pub use bindings::root::AK_INVALID_PLUGINID;
pub use bindings::root::AK_INVALID_POOL_ID;
pub use bindings::root::AK_INVALID_RTPC_ID;
pub use bindings::root::AK_INVALID_UNIQUE_ID;
pub use bindings::root::AK_MAX_PRIORITY;
pub use bindings::root::AK_MIN_PRIORITY;
pub use bindings::root::AK_SOUNDBANK_VERSION;
pub use bindings::AK_INVALID_AUDIO_OBJECT_ID;
pub use bindings::AK_INVALID_GAME_OBJECT;

// #[cfg(windows)]
pub(crate) type OsChar = crate::bindings::root::AkOSChar;
// #[cfg(not(windows))]
// pub(crate) type OsChar = std::os::raw::c_char;

#[macro_export]
macro_rules! with_cstring {
    ($text:expr => $tmp:ident { $($stmt:stmt)+ }) => {
        {
            use ::std::ffi::CString;
            let $tmp = CString::new($text).expect("text shouldn't contain null bytes");
            $($stmt)+
        }
    };
}

/// Create a copy of str as a vector of OsChar (u16 on Windows, i8 == c_char on other platforms).
pub(crate) fn to_os_char<T: AsRef<str>>(str: T) -> Vec<OsChar> {
    #[cfg(windows)]
    {
        // On windows, AkOsChar* ~~ Vec<u16>
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        OsStr::new(str.as_ref())
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect()
    }

    #[cfg(not(windows))]
    {
        use ::std::ffi::CString;
        CString::new(str.as_ref())
            .expect("str shouldn't contain null bytes")
            .as_bytes_with_nul()
            .iter()
            .map(|c| *c as OsChar)
            .collect()
    }
}

/// Wraps an unsafe call to Wwise and match its result to a Result<(), AKRESULT>.
///
/// For example, `ak_call_result[RenderAudio(allow_sync_render)]` expands to
/// ```rust
/// match unsafe { RenderAudio(allow_sync_render) } {
///     AKRESULT::AK_Success => Ok(()),
///     error_code => Err(error_code)
/// }
/// ```
#[macro_export]
macro_rules! ak_call_result {
    ($the_call:expr) => {
        match unsafe { $the_call } {
            AKRESULT::AK_Success => Ok(()),
            error_code => Err(error_code)
        }
    };
    ($the_call:expr => ($($the_tupled_result:expr),*)) => {
        match unsafe { $the_call } {
            AKRESULT::AK_Success => Ok(($($the_tupled_result),*)),
            error_code => Err(error_code)
        }
    };
    ($the_call:expr => $the_result:expr) => {
        match unsafe { $the_call } {
            AKRESULT::AK_Success => Ok($the_result),
            error_code => Err(error_code)
        }
    };
}
