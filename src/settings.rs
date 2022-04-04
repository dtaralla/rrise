/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

#[cfg(not(wwconfig = "release"))]
pub use crate::bindings::root::AkCommSettings;
#[cfg(not(wwconfig = "release"))]
use crate::bindings::root::AK::Comm;
use crate::bindings::root::AK::{MemoryMgr, SoundEngine, StreamMgr};
pub use crate::bindings::root::{
    AkDeviceSettings, AkMemSettings, AkPlatformInitSettings, AkStreamMgrSettings,
};
use crate::to_os_char;
use crate::OsChar;

pub struct AkInitSettings {
    settings: crate::bindings::root::AkInitSettings,
    plugin_dll_path: Vec<OsChar>,
}

impl Default for AkMemSettings {
    fn default() -> Self {
        unsafe {
            let mut ss: AkMemSettings = std::mem::zeroed();
            MemoryMgr::GetDefaultSettings(&mut ss);
            ss
        }
    }
}

impl Default for AkStreamMgrSettings {
    fn default() -> Self {
        unsafe {
            let mut ss: AkStreamMgrSettings = std::mem::zeroed();
            StreamMgr::GetDefaultSettings(&mut ss);
            ss
        }
    }
}

impl Default for AkDeviceSettings {
    fn default() -> Self {
        unsafe {
            let mut ss: AkDeviceSettings = std::mem::zeroed();
            StreamMgr::GetDefaultDeviceSettings(&mut ss);
            ss
        }
    }
}

impl Default for AkInitSettings {
    fn default() -> Self {
        unsafe {
            let mut ss = AkInitSettings {
                settings: std::mem::zeroed(),
                plugin_dll_path: vec![],
            };
            SoundEngine::GetDefaultInitSettings(&mut ss.settings);
            ss
        }
    }
}

impl Default for AkPlatformInitSettings {
    fn default() -> Self {
        unsafe {
            let mut ss: AkPlatformInitSettings = std::mem::zeroed();
            SoundEngine::GetDefaultPlatformInitSettings(&mut ss);
            ss
        }
    }
}

#[cfg(not(wwconfig = "release"))]
impl Default for AkCommSettings {
    fn default() -> Self {
        unsafe {
            let mut ss: AkCommSettings = std::mem::zeroed();
            Comm::GetDefaultInitSettings(&mut ss);
            if let Some(app_name) = app_name() {
                ss.szAppNetworkName = app_name;
            }
            ss
        }
    }
}

unsafe fn app_name() -> Option<[i8; 64]> {
    if let Some(mut name) = std::env::current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
    {
        if name.ends_with(".exe") {
            name.truncate(name.len() - 4);
        }

        if name.len() < 64 {
            name.extend(std::iter::repeat('\0').take(64 - name.len()));
        } else {
            name.truncate(64);
        }

        let mut truncated = [0_u8; 64];
        truncated.copy_from_slice(name.as_bytes());
        truncated[63] = 0;

        Some(std::mem::transmute(truncated))
    } else {
        None
    }
}

impl AkInitSettings {
    /// When using DLLs for plugins, specify their path. Leave NULL if DLLs are in the same folder as the game executable.
    /// Note that on Windows, if `path` has spaces, the DLLs won't be discovered properly.
    pub fn with_plugin_dll_path<T: AsRef<str>>(mut self, path: T) -> Self {
        self.plugin_dll_path = to_os_char(path.as_ref());
        self.settings.szPluginDLLPath = self.plugin_dll_path.as_mut_ptr();
        self
    }

    pub(crate) fn as_ak(&mut self) -> &mut crate::bindings::root::AkInitSettings {
        &mut self.settings
    }
}
