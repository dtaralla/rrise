# Changelog

## 📝 0.3.0 <small>(Unreleased)</small>
- Basic support for Wwise 2022.1 (auto-defined soundbanks are not supported yet)
- Necessary adjustments to keep supporting Wwise 2021.1
- Duplicate examples & example Wwise project to keep supporting both major versions
- Developing...

#### ⚠️ Breaking changes
- To keep working with a Wwise 2021.1 project & SDK, you need to enable the `legacy` feature.
- `AkSoundPosition` is now an `AkWorldTransform` instead of `AkTransform`. The only difference lays in the way the
position is stored: as an `AkVector64` instead of an `AkVector`. 
You have to adapt your calls to `set_position()`.

## 📦 0.2.2 <small>(Nov 12, 2022)</small>
- `rrise_headers` crate added
- Use `cc` crate to detect MSVC `cl.exe`, include paths, lib paths, as well as SDK dir of Wwise SDK on Windows. This effectively adds support for Visual Studio 2022.
- Tested against Wwise 2021.1.9 & 2021.1.10
- Tested with Rust 1.65.0, 1.67.0
- Fix `AkInitSettings::default()` crashing the app if `with_plugin_dll_path()` wasn't used
- Updated music visualizer example to Bevy 0.9

## 📦 0.2.0 <small>(Apr 25, 2022)</small>
- Default impl for `AkCallbackType`
- Implement `Display` for `AkID`
- Added `register_named_game_obj()` for monitoring
- Added `add_listener()`, `remove_listener()` and `set_listeners()`
- Added `unregister_game_obj()`
- Added `add_default_listener()` and `remove_default_listener()`
- Fix missing `callback_type` info in `AkCallbackInfo::Event` and `AkCallbackInfo::Default`

#### ⚠️ Breaking changes
- Settings were reworked to be more rust-safe and rust-friendly

## 📦 0.1.8 <small>(Apr 23, 2022)</small>
- Support for `PostEvent()` callbacks (closures & function pointers)

## 📦 0.1.7 <small>(Apr 18, 2022)</small>
- Music engine support
- `query_params` module

## 📦 0.1.6 <small>(Apr 17, 2022)</small>
- Basic game syncs (RTPCs, triggers, states, switches)
- Fix Wwise config
- Wwise ID Ergonomics

## 📦 0.1.5 <small>(Apr 5, 2022)</small>
- Add documentation
- Support moving audio sources (ffi: `SetPosition()`)

#### ⚠️ Breaking changes
- `sound_engine::stop_all()` now takes an `Option`

## 📦 0.1.4 <small>(Apr 4, 2022)</small>
- Opus is now available
- Solved remaining static linking issues
- Added first integration test

## 📦 0.1.3 <small>(Apr 3, 2022)</small>
- Linux build & run support
- Modularize and cleanup build script
- Vorbis playback support
- Cargo features for static linking of Wwise plugins
- Fix unsafe memory accesses when converting to `AkOsChar*` and `char*`
- Example refactored to be platform agnostic
- Example demonstrates static/dynamic plugin registration for Vorbis compression (static), _AkMeterFX_ (static) and _AkRoomVerbFX_ (dynamic)

## 📦 0.1.0 <small>(Mar 31, 2022)</small>
Very first push of this crate. Currently, this crate can do very little, and only on Windows platforms:

- Windows 10+ support
- Initialize/Update/Terminate a minimal sound engine
- Post simple events (no callback/external source support yet)
- Default streaming manager leveraging Wwise's sample streaming manager
- Profiling from the Wwise authoring tool.
- Minimal example showcasing how to initialize the sound engine, interact with it and terminate it.
