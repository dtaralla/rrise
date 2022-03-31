/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use crate::bindings::root::AK;
use crate::settings::AkCommSettings;
use crate::{ak_call_result, AKRESULT};

pub fn init(mut settings: AkCommSettings) -> Result<(), AKRESULT> {
    ak_call_result![AK::Comm::Init(&mut settings)]
}

pub fn term() {
    unsafe {
        AK::Comm::Term();
    }
}
