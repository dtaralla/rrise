/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use crate::{
    bindings::root::{AK::SoundEngine::*, *},
    *,
};
use ::std::fmt::Debug;
use ::std::marker::PhantomData;
use std::convert::TryInto;

pub fn init(
    mut init_settings: AkInitSettings,
    mut platform_init_settings: AkPlatformInitSettings,
) -> Result<(), AKRESULT> {
    ak_call_result![Init(&mut init_settings, &mut platform_init_settings)]
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

pub fn set_default_listeners(listener_ids: &Vec<AkGameObjectID>) -> Result<(), AKRESULT> {
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
    ak_call_result![LoadBank(ak_text![&name], &mut bank_id) => bank_id]
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
                PostEvent1(
                    ak_text![name],
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
