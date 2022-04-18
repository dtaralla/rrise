# Rrise

[![Crates.io](https://img.shields.io/crates/v/rrise.svg)](https://crates.io/crates/rrise)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/d/rrise.svg)](https://crates.io/crates/rrise)

## What is Rrise?
Rrise is a Rust binding for [Wwise](https://www.audiokinetic.com/en/products/wwise). It is _not_ and *does not want* to 
be a complete game engine integration, but rather a starting point for other crates leveraging the binding.

The end goal is to provide game engines written in Rust like [Bevy](https://github.com/bevyengine/bevy) with a safe 
Wwise API, without having to tinker with the FFI world.

### About your expectations...
This is planned to become a rather advanced crate, that paves the way for exciting sound engine work in established Rust
game engines. That said, I'm definitely not the most proficient in Rust. If you notice some questionable implementation 
or architectural choices, please reach out to improve the crate!

Pull requests are more than welcome: **they are encouraged**!

## Capabilities
- Build & run on Windows 10+
- Build & run on WSL[^0]/Linux (on distros where Wwise is supported)
- (AD)PCM, Vorbis & Opus playback
- Initialize/Update/Terminate sound & music engines
- Post simple events (no callback/external source support yet)
- Set source position
- RTPCs, switches, states and triggers
- Default streaming manager leveraging Wwise's sample streaming manager
- Profiling from the Wwise authoring tool
- Dynamic & static linking of Wwise plugins through cargo features

[^0]: WSL2 on Windows 11 is required to run the `bevy_music_visualizer` example

### Examples
- Minimal example showcasing how to initialize the sound engine, start a moving source and terminate it
- Doppler effect example showcasing RTPCs
- Music engine example with frequency visualizer

<p align="center">
  <a href="https://www.youtube.com/watch?v=W_GUGTm2BcA" title="Music engine example with frequency visualizer" rel="noreferrer noopener" target="_blank">
    <img src="images/bevy_music_visualizer.png" alt="Bevy visualizer in video (click to play)" style="max-width: 100%;" />
  </a>
</p>

### Logging
Rrise uses the [log](https://docs.rs/log/latest/log/index.html) crate for all its logging needs. Refer to `log`'s 
docs for how to use it.

### Accepted configs
You can set the following `cfg` flags through the `RUSTFLAGS` environment variable before building to chose which 
configuration of Wwise you wish to link against:

|                           | Flag                               | `AK_OPTIMIZED` | Comms | Description                                        |
|---------------------------|------------------------------------|:--------------:|:-----:|----------------------------------------------------|
| **Debug**                 | `--cfg wwdebug`                    |       ✕        |   ✔   | Useful to debug with AudioKinetic support folks    |
| **Profile**<br/>(default) | `--cfg wwprofile`<br/>(or nothing) |       ✕        |   ✔   | That's what you should be using during development |
| **Release**               | `--cfg wwrelease`                  |       ✔        |   ✕   | Optimized for final retail builds                  |

If **Comms** are disabled, it means you can't connect to the running game from Wwise Authoring app.

For more info, check the [AudioKinetic docs](https://www.audiokinetic.com/library/edge/?source=SDK&id=goingfurther_builds.html#wwise_sdk_build_config).

### Wwise Plugins
You can [choose](https://www.audiokinetic.com/library/edge/?source=SDK&id=soundengine_integration_plugins.html) to 
either link statically or dynamically to the Wwise plugins.

Note that some plugins like _AkMeter_, _AkVorbisDecoder_ and _AkOpusDecoder_ can only be statically linked and are not 
available for dynamic linking.

See [this page](https://www.audiokinetic.com/library/edge/?source=SDK&id=goingfurther_builds.html#wwise_sdk_lib_dependency_requirements_plugins)
for a list of plugins supported by Wwise, per platform.

#### Dynamic linking
This is the default behavior. Wwise plugins like _AkRoomVerb_, _AkParametricEQ_, _Motion_ etc. will be loaded at 
runtime from their respective shared library as needed.

Any project relying on dynamic linking for some plugins needs to also deploy their respective *licensed* shared 
libraries along the final executable (you can do this with a 
[build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html) for instance).

You can find these shared libraries in `$WWISESDK/[platform]/[config]/bin`.

#### Static linking
You might want to statically link some Wwise plugins instead of loading them at runtime from a shared libary. In 
this case, you can leverage Rrise's cargo features to enable static linking of such plugins.

For example, if you want to statically link the _AkRoomVerb_ plugin, just build with the `AkRoomVerbFX` feature 
enabled. When your project runs, you can check that the static version of the plugin was loaded in the debug log:
```
AkRoomVerbFX has been statically loaded successfully
```
**Note:** If you already built your project once, you need to make Rrise's build script rerun to enable static 
linking of your features. You can change the value of the `RRISE_RERUN_BUILD` environment variable before building to 
force a rerun of Rrise's build script. You can also force a full rebuild with `cargo clean & cargo build 
--features=The,Plugin,List`. 

### Known issues & limitations
- If you dynamically link Wwise effect plugins (default behavior), there is an issue on Windows where if the path given
to `AkInitSettings::with_plugin_dll_path` contains spaces, the DLLs in that folder won't be discoverable by Wwise.
- On Linux, when connecting the profiler, you will get those messages in the console (they seem totally harmless):
```
.../SDK/Linux_x64/Profile/bin/libDefaultConversions.so: cannot open shared object file: No such file or directory
.../SDK/Linux_x64/Profile/bin/libAkSoundEngineDLL.so: cannot open shared object file: No such file or directory
```

## Requirements
- Rust nightly 1.61+ (to support [-wholearchive](https://github.com/rust-lang/rust/pull/93901))
- The `bindgen` crate [requirements](https://github.com/rust-lang/rust-bindgen/blob/master/book/src/requirements.md)
- A licensed (free, trial, commercial,...) version of Wwise installed (2021.1.7, untested on others)
  - Wwise itself
  - Wwise SDK (C++)
- **On Windows: `MSVC`**[^1]
  - Windows 10 SDK
  - Build tools (same as Rust, for the `cc` crate)
  - `cl.exe` must be in the PATH[^2]
  - Wwise support for any Visual Studio 20XX deployment platform
  - Make sure the `WWISESDK` environment variable is set to the SDK folder of your Wwise installation
- **On Linux: `clang`**
  - `g++` (for `libstdc++`)
  - Copy the SDK folder from a Windows[^3] install of Wwise on your Linux workstation (for instance in /opt/wwise)
    - Make sure the `WWISESDK` environment variable is set to that folder

[^1]: Not tested on other compilers like MinGW or Clang

[^2]: Current limitation of the build script. I want to improve MSVC path discovery in the future to remove this 
requirement.

[^3]: AudioKinetic doesn't provide direct downloads to their SDK: you can only install it through their launcher. 
However, this launcher being only available on Windows and MacOS, you'll need to install it on a VM or similar before 
you can work with this crate on Linux.

## Short-term roadmap
- Add callback and user data support for PostEvent
- Spatial module basic API and example
- Review/Improve architecture

### Legal stuff
Wwise and the Wwise logo are trademarks of Audiokinetic Inc., registered in the U.S. and other countries.

This project is in no way affiliated to Audiokinetic.

You still need a licensed version of Wwise installed to compile and run this project. You need a valid Wwise license 
to distribute any project based on this crate.