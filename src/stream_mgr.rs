/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use crate::bindings::root::{InitDefaultStreamMgr, TermDefaultStreamMgr, AK};
use crate::settings::{AkDeviceSettings, AkStreamMgrSettings};
use crate::{ak_call_result, to_os_char, AKRESULT};

pub fn init(mut settings: AkStreamMgrSettings) -> Result<(), AKRESULT> {
    let addr = unsafe { AK::StreamMgr::Create(&mut settings) };
    if addr == std::ptr::null_mut() {
        Err(AKRESULT::AK_Fail)
    } else {
        Ok(())
    }
}

pub fn init_default_stream_mgr<T: AsRef<str>>(
    stream_mgr_settings: AkStreamMgrSettings,
    mut device_settings: AkDeviceSettings,
    bank_location: T,
) -> Result<(), AKRESULT> {
    init(stream_mgr_settings)?;
    device_settings.bUseStreamCache = true;

    let pin_bytes = to_os_char(&bank_location);
    ak_call_result![InitDefaultStreamMgr(&device_settings, pin_bytes.as_ptr())]
}

pub fn term_default_stream_mgr() {
    unsafe {
        TermDefaultStreamMgr();
    }
}

pub fn set_current_language<T: AsRef<str>>(language_name: T) -> Result<(), AKRESULT> {
    let pin_bytes = to_os_char(&language_name);
    ak_call_result![AK::StreamMgr::SetCurrentLanguage(pin_bytes.as_ptr())]
}
