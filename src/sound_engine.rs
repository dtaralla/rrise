/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use crate::{
    bindings::root::{AK::SoundEngine::*, *},
    settings::AkInitSettings,
    *,
};
use ::std::convert::TryInto;
use ::std::fmt::Debug;
use ::std::marker::PhantomData;

macro_rules! link_static_plugin {
    ($feature:ident) => {
        link_static_plugin![$feature, $feature]
    };
    ($feature:ident, $global_var_name:ident) => {
        paste::paste! {
            #[cfg(feature = "" $feature)]
            {
                // If max log level doesn't include debug, need to explicitly reference this variable
                // or it won't be statically linked and the plugin won't be able to be loaded.
                #[cfg(any(
                    all(
                        debug_assertions,
                        all(not(feature = "max_level_debug"), not(feature = "max_level_trace"))
                    ),
                    all(
                        not(debug_assertions),
                        all(
                            not(feature = "release_max_level_debug"),
                            not(feature = "release_max_level_trace")
                        )
                    ),
                ))]
                ::std::convert::identity(unsafe {
                    crate::bindings_static_plugins::[<$global_var_name Registration>]
                });
                log::debug!(
                    "{} has been statically loaded successfully",
                    stringify!($feature)
                )
            }
        }
    };
}

/// Initialize the sound engine.
///
/// *Warning* This function is not thread-safe.
///
/// *Remark* The initial settings should be initialized using [AkInitSettings::default]
/// and [AkPlatformInitSettings::default] to fill the structures with their
/// default settings. This is not mandatory, but it helps avoid backward compatibility problems.
///
/// *Return*
/// > - [AK_Success](AkResult::AK_Success) if the initialization was successful
/// > - [AK_MemManagerNotInitialized](AkResult::AK_MemManagerNotInitialized) if the memory manager is not available or not properly initialized
/// > - [AK_StreamMgrNotInitialized](AkResult::AK_StreamMgrNotInitialized) if the stream manager is not available or not properly initialized
/// > - [AK_SSEInstructionsNotSupported](AkResult::AK_SSEInstructionsNotSupported) if the machine does not support SSE instruction (only on the PC)
/// > - [AK_InsufficientMemory](AkResult::AK_InsufficientMemory) or [AK_Fail](AkResult::AK_Fail) if there is not enough memory available to initialize the sound engine properly
/// > - [AK_InvalidParameter](AkResult::AK_InvalidParameter) if some parameters are invalid
/// > - [AK_Fail](AkResult::AK_Fail) if the sound engine is already initialized, or if the provided settings result in insufficient resources for the initialization.
///
/// *See also*
/// > - [term]
/// > - [AkInitSettings::default]
/// > - [AkPlatformInitSettings::default]
pub fn init(
    mut init_settings: AkInitSettings,
    mut platform_init_settings: AkPlatformInitSettings,
) -> Result<(), AkResult> {
    ak_call_result![Init(init_settings.as_ak(), &mut platform_init_settings)]?;

    link_static_plugin![AkVorbisDecoder];
    link_static_plugin![AkOggOpusDecoder]; // see Ak/Plugin/AkOpusDecoderFactory.h
    link_static_plugin![AkWemOpusDecoder]; // see Ak/Plugin/AkOpusDecoderFactory.h
    link_static_plugin![AkMeterFX];
    link_static_plugin![AkAudioInputSource];
    link_static_plugin![AkCompressorFX];
    link_static_plugin![AkDelayFX];
    link_static_plugin![AkExpanderFX];
    link_static_plugin![AkFlangerFX];
    link_static_plugin![AkGainFX];
    link_static_plugin![AkGuitarDistortionFX];
    link_static_plugin![AkHarmonizerFX];
    link_static_plugin![AkMatrixReverbFX];
    link_static_plugin![AkParametricEQFX];
    link_static_plugin![AkPeakLimiterFX];
    link_static_plugin![AkPitchShifterFX];
    link_static_plugin![AkRecorderFX];
    link_static_plugin![AkRoomVerbFX];
    link_static_plugin![AkSilenceSource];
    link_static_plugin![AkSineSource, SineSource];
    link_static_plugin![AkStereoDelayFX];
    link_static_plugin![AkSynthOneSource, AkSynthOne];
    link_static_plugin![AkTimeStretchFX];
    link_static_plugin![AkToneSource];
    link_static_plugin![AkTremoloFX];

    Ok(())
}

/// Query whether or not the sound engine has been successfully initialized.
///
/// *Warning* This function is not thread-safe. It should not be called at the same time as [init()] or [term()].
///
/// *Return* `True` if the sound engine has been initialized, `False` otherwise.
///
/// *See also*
/// > - [init]
/// > - [term]
pub fn is_initialized() -> bool {
    unsafe { IsInitialized() }
}

/// Terminates the sound engine.
///
/// If some sounds are still playing or events are still being processed when this function is
/// called, they will be stopped.
///
/// *Warning* This function is not thread-safe.
///
/// *Warning* Before calling `Term`, you must ensure that no other thread is accessing the sound engine.
///
/// *See also*
/// > - [init]
pub fn term() {
    unsafe {
        AK::SoundEngine::Term();
    }
}

/// Processes all commands in the sound engine's command queue.
///
/// This method has to be called periodically (usually once per game frame).
///
/// `allow_sync_render`: When AkInitSettings::bUseLEngineThread is false, `RenderAudio` may generate
/// an audio buffer -- unless in_bAllowSyncRender is set to false. Use in_bAllowSyncRender=false
/// when calling RenderAudio from a Sound Engine callback.
///
/// *Return* Always returns [AK_Success](AkResult::AK_Success)
///
/// *See also*
/// > - [PostEvent](struct@PostEvent)
pub fn render_audio(allow_sync_render: bool) -> Result<(), AkResult> {
    ak_call_result![RenderAudio(allow_sync_render)]
}

/// Unregister all game objects, or all game objects with a particular matching set of property flags.
///
/// This function to can be used to unregister all game objects.
///
/// *Return* AK_Success if successful
///
/// *Remark* Registering a game object twice does nothing. Unregistering it once unregisters it no
/// matter how many times it has been registered. Unregistering a game object while it is
/// in use is allowed, but the control over the parameters of this game object is lost.
/// For example, if a sound associated with this game object is a 3D moving sound, it will
/// stop moving once the game object is unregistered, and there will be no way to recover
/// the control over this game object.
///
/// *See also*
/// - [register_game_obj]
/// - [unregister_game_obj]
pub fn unregister_all_game_obj() -> Result<(), AkResult> {
    ak_call_result![UnregisterAllGameObj()]
}

/// Unregisters a game object.
///
/// *Return*
/// > - AK_Success if successful
/// > - AK_Fail if the specified AkGameObjectID is invalid (0 is an invalid ID)
///
/// *Remark* Registering a game object twice does nothing. Unregistering it once unregisters it no
/// matter how many times it has been registered. Unregistering a game object while it is
/// in use is allowed, but the control over the parameters of this game object is lost.
/// For example, say a sound associated with this game object is a 3D moving sound. This sound will
/// stop moving when the game object is unregistered, and there will be no way to regain control over the game object.
///
/// *See also*
/// > - [register_game_obj]
/// > - [unregister_all_game_obj]
pub fn register_game_obj(game_object_id: AkGameObjectID) -> Result<(), AkResult> {
    ak_call_result![RegisterGameObj(game_object_id)]
}

/// Sets the position of a game object.
///
/// *Warning* `position`'s orientation vectors must be normalized.
///
/// *Return*
/// > - [AK_Success](AkResult::AK_Success) when successful
/// > - [AK_InvalidParameter](AkResult::AK_InvalidParameter) if parameters are not valid.
pub fn set_position<T: Into<AkSoundPosition>>(
    game_object_id: AkGameObjectID,
    position: T,
) -> Result<(), AkResult> {
    ak_call_result![SetPosition(game_object_id, &position.into())]
}

/// Sets the default set of associated listeners for game objects that have not explicitly overridden their listener sets. Upon registration, all game objects reference the default listener set, until
/// a call to [add_listener], [remove_listener], [set_listeners] or [set_game_object_output_bus_volume] is made on that game object.
///
/// All default listeners that have previously been added via AddDefaultListener or set via SetDefaultListeners will be removed and replaced with the listeners in the array in_pListenerGameObjs.
///
/// *Return* Always returns [AK_Success](AkResult::AK_Success)
pub fn set_default_listeners(listener_ids: &[AkGameObjectID]) -> Result<(), AkResult> {
    ak_call_result![SetDefaultListeners(
        listener_ids.as_ptr(),
        listener_ids.len().try_into().unwrap()
    )]
}

/// Stops the current content playing associated to the specified game object ID.
///
/// If no game object is specified, all sounds will be stopped.
pub fn stop_all(game_object_id: Option<AkGameObjectID>) {
    unsafe {
        StopAll(game_object_id.unwrap_or(AK_INVALID_GAME_OBJECT));
    }
}

/// Load a bank synchronously (by Unicode string).
///
/// The bank name is passed to the Stream Manager.
///
/// A bank load request will be posted, and consumed by the Bank Manager thread.
///
/// The function returns when the request has been completely processed.
///
/// *Return*
/// The bank ID (see [get_id_from_string]). You may use this ID with [unload_bank].
/// > - [AK_Success](AkResult::AK_Success): Load or unload successful.
/// > - [AK_InsufficientMemory](AkResult::AK_InsufficientMemory): Insufficient memory to store bank data.
/// > - [AK_BankReadError](AkResult::AK_BankReadError): I/O error.
/// > - [AK_WrongBankVersion](AkResult::AK_WrongBankVersion): Invalid bank version: make sure the version of Wwise that you used to generate the SoundBanks matches that of the SDK you are currently using.
/// > - [AK_InvalidFile](AkResult::AK_InvalidFile): File specified could not be opened.
/// > - [AK_InvalidParameter](AkResult::AK_InvalidParameter): Invalid parameter, invalid memory alignment.
/// > - [AK_Fail](AkResult::AK_Fail): Load or unload failed for any other reason. (Most likely small allocation failure)
///
/// *Remarks*
/// > - The initialization bank must be loaded first.
/// > - All SoundBanks subsequently loaded must come from the same Wwise project as the
///   initialization bank. If you need to load SoundBanks from a different project, you
///   must first unload ALL banks, including the initialization bank, then load the
///   initialization bank from the other project, and finally load banks from that project.
/// > - Codecs and plug-ins must be registered before loading banks that use them.
/// > - Loading a bank referencing an unregistered plug-in or codec will result in a load bank success,
/// but the plug-ins will not be used. More specifically, playing a sound that uses an unregistered effect plug-in
/// will result in audio playback without applying the said effect. If an unregistered source plug-in is used by an event's audio objects,
/// posting the event will fail.
/// > - The sound engine internally calls get_id_from_string(name) to return the correct bank ID.
/// Therefore, in_pszString should be the real name of the SoundBank (with or without the BNK extension - it is trimmed internally),
/// not the name of the file (if you changed it), nor the full path of the file. The path should be resolved in
/// your implementation of the Stream Manager, or in the Low-Level I/O module if you use the default Stream Manager's implementation.
///
/// *See also*
/// > - [unload_bank_by_name]
/// > - [unload_bank_by_id]
/// > - [clear_banks]
/// > - [get_id_from_string]
pub fn load_bank_by_name<T: AsRef<str>>(name: T) -> Result<AkBankID, AkResult> {
    let mut bank_id = 0;
    with_cstring![name.as_ref() => cname {
        ak_call_result![LoadBank1(cname.as_ptr(), &mut bank_id) => bank_id]
    }]
}

#[derive(Debug, Copy, Clone)]
/// Helper to post events to the sound engine.
///
/// Use [PostEvent::post] to post your event to the sound engine.
///
/// The callback function can be used to be noticed when markers are reached or when the event is finished.
///
/// An array of Wave file sources can be provided to resolve External Sources triggered by the event.
///
/// *Return* The playing ID of the event launched, or [AK_INVALID_PLAYING_ID] if posting the event failed
///
/// *Remarks*
/// > - If used, the array of external sources should contain the information for each external source triggered by the
/// event. When triggering an Event with multiple external sources, you need to differentiate each source
/// by using the cookie property in the External Source in the Wwise project and in AkExternalSourceInfo.
/// > - If an event triggers the playback of more than one external source, they must be named uniquely in the project
/// (therefore have a unique cookie) in order to tell them apart when filling the AkExternalSourceInfo structures.
///
/// *See also*
/// > - [render_audio]
/// > - [get_source_play_position]
pub struct PostEvent<'a> {
    game_obj_id: AkGameObjectID,
    event_id: AkID<'a>,
    flags: AkCallbackType,
    // callback: Option<Box<dyn Fn(AkCallbackType, CallbackInfo)>>, // TODO
    // cookie: Option<Cookie>,                                      // TODO
    // external_sources: Vec<...>                                   // TODO
    playing_id: AkPlayingID,
    marker: PhantomData<&'a u8>,
}

impl<'a> PostEvent<'a> {
    /// Select an event by name or by ID, to play on a given game object.
    pub fn new<T: Into<AkID<'a>>>(game_obj_id: AkGameObjectID, event_id: T) -> PostEvent<'a> {
        PostEvent {
            game_obj_id,
            event_id: event_id.into(),
            flags: AkCallbackType(0),
            // callback: None,
            // cookie: None,
            // external_sources: ...,
            playing_id: AK_INVALID_PLAYING_ID,
            marker: PhantomData,
        }
    }

    /// Add flags before posting. Bitmask: see [AkCallbackType].
    pub fn add_flags(&mut self, flags: AkCallbackType) -> &mut Self {
        self.flags |= flags;
        self
    }

    /// Set flags before posting. Bitmask: see [AkCallbackType]
    pub fn flags(&mut self, flags: AkCallbackType) -> &mut Self {
        self.flags = flags;
        self
    }

    /// Advanced users only. Specify the playing ID to target with the event. Will Cause active
    /// actions in this event to target an existing Playing ID. Let it be [AK_INVALID_PLAYING_ID]
    /// or do not specify any for normal playback.
    pub fn playing_id(&mut self, id: AkPlayingID) -> &mut Self {
        self.playing_id = id;
        self
    }

    /// Posts the event to the sound engine.
    pub fn post(&self) -> Result<AkPlayingID, AkResult> {
        if let AkID::Name(name) = self.event_id {
            let ak_playing_id = unsafe {
                with_cstring![name => cname {
                    PostEvent2(
                        cname.as_ptr(),
                        self.game_obj_id,
                        self.flags.0 as u32,
                        None,                   // TODO
                        ::std::ptr::null_mut(), // TODO
                        0,                      // TODO
                        ::std::ptr::null_mut(), // TODO
                        self.playing_id,
                    )
                }]
            };
            if ak_playing_id == AK_INVALID_PLAYING_ID {
                Err(AkResult::AK_Fail)
            } else {
                Ok(ak_playing_id)
            }
        } else if let AkID::ID(id) = self.event_id {
            let ak_playing_id = unsafe {
                PostEvent(
                    id,
                    self.game_obj_id,
                    self.flags.0 as u32,
                    None,                   // TODO
                    ::std::ptr::null_mut(), // TODO
                    0,                      // TODO
                    ::std::ptr::null_mut(), // TODO
                    self.playing_id,
                )
            };
            if ak_playing_id == AK_INVALID_PLAYING_ID {
                Err(AkResult::AK_Fail)
            } else {
                Ok(ak_playing_id)
            }
        } else {
            panic!("need at least an event ID or and an event name to post")
        }
    }
}
