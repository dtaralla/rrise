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
                    crate::bindings_static_plugins::[<$feature Registration>]
                });
                log::debug!(
                    "{} has been statically loaded successfully",
                    stringify!($feature)
                )
            }
        }
    };
}

pub fn init(
    mut init_settings: AkInitSettings,
    mut platform_init_settings: AkPlatformInitSettings,
) -> Result<(), AKRESULT> {
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
    link_static_plugin![AkSineSource];
    link_static_plugin![AkStereoDelayFX];
    link_static_plugin![AkSynthOneSource];
    link_static_plugin![AkTimeStretchFX];
    link_static_plugin![AkToneSource];
    link_static_plugin![AkTremoloFX];

    Ok(())
}

pub fn is_initialized() -> bool {
    return unsafe { IsInitialized() };
}

pub fn term() {
    unsafe {
        AK::SoundEngine::Term();
    }
}

pub fn render_audio(allow_sync_render: bool) -> Result<(), AKRESULT> {
    ak_call_result![RenderAudio(allow_sync_render)]
}

pub fn unregister_all_game_obj() -> Result<(), AKRESULT> {
    ak_call_result![UnregisterAllGameObj()]
}

pub fn register_game_obj(game_object_id: AkGameObjectID) -> Result<(), AKRESULT> {
    ak_call_result![RegisterGameObj(game_object_id)]
}

pub fn set_default_listeners(listener_ids: &[AkGameObjectID]) -> Result<(), AKRESULT> {
    ak_call_result![SetDefaultListeners(
        listener_ids.as_ptr(),
        listener_ids.len().try_into().unwrap()
    )]
}

pub fn stop_all(game_object_id: AkGameObjectID) {
    unsafe {
        StopAll(game_object_id);
    }
}

pub fn load_bank_by_name<T: AsRef<str>>(name: T) -> Result<AkBankID, AKRESULT> {
    let mut bank_id = 0;
    with_cstring![name.as_ref() => cname {
        ak_call_result![LoadBank1(cname.as_ptr(), &mut bank_id) => bank_id]
    }]
}

#[derive(Debug, Copy, Clone)]
pub struct PostEvent<'a> {
    game_obj_id: AkGameObjectID,
    event_id: Option<AkUniqueID>,
    event_name: Option<&'a str>,
    flags: AkCallbackType,
    // callback: Option<Box<dyn Fn(AkCallbackType, CallbackInfo)>>, // TODO
    // cookie: Option<Cookie>,                                      // TODO
    // external_sources: Vec<...>                                   // TODO
    playing_id: AkPlayingID,
    marker: PhantomData<&'a u8>,
}

impl<'a> PostEvent<'a> {
    fn new(
        game_obj_id: AkGameObjectID,
        event_id: Option<AkUniqueID>,
        event_name: Option<&'a str>,
    ) -> PostEvent<'a> {
        PostEvent {
            game_obj_id,
            event_id,
            event_name,
            flags: AkCallbackType(0),
            // callback: None,
            // cookie: None,
            // external_sources: ...,
            playing_id: AK_INVALID_PLAYING_ID,
            marker: PhantomData,
        }
    }

    pub fn from_event_name(game_obj_id: AkGameObjectID, name: &'a str) -> PostEvent {
        Self::new(game_obj_id, None, Some(name))
    }

    pub fn from_event_id(game_obj_id: AkGameObjectID, id: AkUniqueID) -> PostEvent<'a> {
        Self::new(game_obj_id, Some(id), None)
    }

    pub fn add_flags(&mut self, flags: AkCallbackType) -> &mut Self {
        self.flags |= flags;
        self
    }

    pub fn flags(&mut self, flags: AkCallbackType) -> &mut Self {
        self.flags = flags;
        self
    }

    pub fn playing_id(&mut self, id: AkPlayingID) -> &mut Self {
        self.playing_id = id;
        self
    }

    pub fn post(&self) -> Result<AkPlayingID, AKRESULT> {
        if let Some(name) = self.event_name {
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
                Err(AKRESULT::AK_Fail)
            } else {
                Ok(ak_playing_id)
            }
        } else if let Some(id) = self.event_id {
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
                Err(AKRESULT::AK_Fail)
            } else {
                Ok(ak_playing_id)
            }
        } else {
            panic!("need at least an event ID or and an event name to post")
        }
    }
}
