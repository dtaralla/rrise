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
mod transform;

pub use transform::*;

/// Acoustic Texture ID
pub use bindings::root::AkAcousticTextureID;
/// Argument value ID
pub use bindings::root::AkArgumentValueID;
/// Audio Object ID
pub use bindings::root::AkAudioObjectID;
/// Auxilliary bus ID
pub use bindings::root::AkAuxBusID;
/// Run time bank ID
pub use bindings::root::AkBankID;
/// Channel mask (similar to WAVE_FORMAT_EXTENSIBLE). Bit values are defined in AkSpeakerConfig.h.
pub use bindings::root::AkChannelMask;
/// Codec plug-in ID
pub use bindings::root::AkCodecID;
/// Data compression format ID
pub use bindings::root::AkDataCompID;
/// Data interleaved state ID
pub use bindings::root::AkDataInterleaveID;
/// Data sample type ID
pub use bindings::root::AkDataTypeID;
/// I/O device ID
pub use bindings::root::AkDeviceID;
/// Integer-type file identifier
pub use bindings::root::AkFileID;
/// Game object ID
pub use bindings::root::AkGameObjectID;
/// Image Source ID
pub use bindings::root::AkImageSourceID;
/// Low-pass filter type
pub use bindings::root::AkLPFType;
/// Memory pool ID
pub use bindings::root::AkMemPoolId;
/// Modulator ID
pub use bindings::root::AkModulatorID;
/// Audio Output device ID
pub use bindings::root::AkOutputDeviceID;
/// Unique node (bus, voice) identifier for profiling.
pub use bindings::root::AkPipelineID;
/// Pitch value
pub use bindings::root::AkPitchValue;
/// Playing ID
pub use bindings::root::AkPlayingID;
/// Source or effect plug-in ID
pub use bindings::root::AkPluginID;
/// Source or effect plug-in parameter ID
pub use bindings::root::AkPluginParamID;
/// Port number
pub use bindings::root::AkPortNumber;
/// Priority
pub use bindings::root::AkPriority;
/// Unique (per emitter) identifier for an emitter-listener ray.
pub use bindings::root::AkRayID;
/// Real time parameter control ID
pub use bindings::root::AkRtpcID;
/// Real time parameter control value
pub use bindings::root::AkRtpcValue;
/// State group ID
pub use bindings::root::AkStateGroupID;
/// State ID
pub use bindings::root::AkStateID;
/// Switch group ID
pub use bindings::root::AkSwitchGroupID;
/// Switch ID
pub use bindings::root::AkSwitchStateID;
/// Time in ms
pub use bindings::root::AkTimeMs;
/// Trigger ID
pub use bindings::root::AkTriggerID;
/// Unique 32-bit ID
pub use bindings::root::AkUniqueID;
/// Volume value( also apply to LFE )
pub use bindings::root::AkVolumeValue;
pub use error::*;

#[doc(inline)]
pub use bindings::root::AkListenerPosition;
#[doc(inline)]
pub use bindings::root::AkSoundPosition;
#[doc(inline)]
pub use bindings::root::AkTransform;
#[doc(inline)]
pub use bindings::root::AkVector;
#[doc(inline)]
pub use bindings::root::AKRESULT as AkResult;

#[doc(inline)]
pub use bindings::root::AK_DEFAULT_BANK_IO_PRIORITY;
#[doc(inline)]
pub use bindings::root::AK_DEFAULT_BANK_THROUGHPUT;
#[doc(inline)]
pub use bindings::root::AK_DEFAULT_POOL_ID;
#[doc(inline)]
pub use bindings::root::AK_DEFAULT_PRIORITY;
#[doc(inline)]
pub use bindings::root::AK_DEFAULT_SWITCH_STATE;
#[doc(inline)]
pub use bindings::root::AK_FALLBACK_ARGUMENTVALUE_ID;
#[doc(inline)]
pub use bindings::root::AK_INVALID_AUX_ID;
#[doc(inline)]
pub use bindings::root::AK_INVALID_BANK_ID;
#[doc(inline)]
pub use bindings::root::AK_INVALID_CHANNELMASK;
#[doc(inline)]
pub use bindings::root::AK_INVALID_DEVICE_ID;
#[doc(inline)]
pub use bindings::root::AK_INVALID_FILE_ID;
#[doc(inline)]
pub use bindings::root::AK_INVALID_OUTPUT_DEVICE_ID;
#[doc(inline)]
pub use bindings::root::AK_INVALID_PIPELINE_ID;
#[doc(inline)]
pub use bindings::root::AK_INVALID_PLAYING_ID;
#[doc(inline)]
pub use bindings::root::AK_INVALID_PLUGINID;
#[doc(inline)]
pub use bindings::root::AK_INVALID_POOL_ID;
#[doc(inline)]
pub use bindings::root::AK_INVALID_RTPC_ID;
#[doc(inline)]
pub use bindings::root::AK_INVALID_UNIQUE_ID;
#[doc(inline)]
pub use bindings::root::AK_MAX_PRIORITY;
#[doc(inline)]
pub use bindings::root::AK_MIN_PRIORITY;
#[doc(inline)]
pub use bindings::root::AK_SOUNDBANK_VERSION;
#[doc(inline)]
pub use bindings::AK_INVALID_AUDIO_OBJECT_ID;
#[doc(inline)]
pub use bindings::AK_INVALID_GAME_OBJECT;

#[doc(hidden)]
pub(crate) type OsChar = crate::bindings::root::AkOSChar;

#[doc(hidden)]
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

#[doc(hidden)]
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

#[doc(hidden)]
/// Wraps an unsafe call to Wwise and match its result to a Result<(), AkResult>.
///
/// For example, `ak_call_result[RenderAudio(allow_sync_render)]` expands to
/// ```rust,ignore
/// match unsafe { RenderAudio(allow_sync_render) } {
///     AkResult::AK_Success => Ok(()),
///     error_code => Err(error_code)
/// }
/// ```
#[macro_export]
macro_rules! ak_call_result {
    ($the_call:expr) => {
        match unsafe { $the_call } {
            AkResult::AK_Success => Ok(()),
            error_code => Err(error_code)
        }
    };
    ($the_call:expr => ($($the_tupled_result:expr),*)) => {
        match unsafe { $the_call } {
            AkResult::AK_Success => Ok(($($the_tupled_result),*)),
            error_code => Err(error_code)
        }
    };
    ($the_call:expr => $the_result:expr) => {
        match unsafe { $the_call } {
            AkResult::AK_Success => Ok($the_result),
            error_code => Err(error_code)
        }
    };
}
