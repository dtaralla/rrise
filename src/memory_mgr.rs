/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use crate::bindings::root::AK;
use crate::settings::AkMemSettings;
use crate::{ak_call_result, AKRESULT};

pub fn init(mut settings: AkMemSettings) -> Result<(), AKRESULT> {
    ak_call_result![AK::MemoryMgr::Init(&mut settings)]
}

pub fn is_initialized() -> bool {
    unsafe { AK::MemoryMgr::IsInitialized() }
}

pub fn term() {
    unsafe {
        AK::MemoryMgr::Term();
    }
}
