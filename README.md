# Rrise

[![Crates.io](https://img.shields.io/crates/v/rrise.svg)](https://crates.io/crates/rrise)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](./LICENSE)

## What is Rrise?
Rrise is a Rust binding for [Wwise](https://www.audiokinetic.com/en/products/wwise). It is _not_ and *does not want* to be a complete game engine integration, but rather 
a starting point for other crates leveraging the binding.

The end goal is to provide game engines written in Rust like [Bevy](https://github.com/bevyengine/bevy) and
[Amethyst](https://github.com/amethyst/amethyst) with a safe Wwise API, without having to tinker with the FFI
world.

### About your expectations...
This is planned to become a rather advanced crate, that paves the way for exciting sound engine work in established Rust
game engines. That said, I'm definitely not the most proficient in Rust. If you notice some questionable implementation 
or architectural choices, please reach out to improve the crate!

Pull requests are more than welcome: **they are encouraged**!

## Features
Currently, this crate can do very little, and only on Windows platforms:
- Windows 10+ support
- Initialize/Update/Terminate a minimal sound engine
- Post simple events (no callback/external source support yet)
- Default streaming manager leveraging Wwise's sample streaming manager
- Profiling from the Wwise authoring tool.
- Minimal example showcasing how to initialize the sound engine, interact with it and terminate it.

## Requirements
- A licensed (free, trial, commercial,...) version of Wwise installed
  - Wwise itself
  - Wwise SDK (C++)
  - Wwise support for any Visual Studio 20XX deployment platform
  - Make sure the `WWISESDK` environment is set to the SDK folder of your Wwise installation
- MSVC[^1]
  - Windows 10 SDK
  - Build tools (same as Rust, for the `cc` crate)
  - `cl.exe` must be in the PATH (current limitation of the build script)
- The `bindgen` crate [requirements](https://github.com/rust-lang/rust-bindgen/blob/master/book/src/requirements.md)
[^1]: Not tested on other compilers like MinGW or Clang

## Short-term roadmap
- Add support for Linux/WSL
- Spatial module basic API and example
- Modularize with features (especially profiling)
- Add callback and user data support for PostEvent
- Review/Improve architecture

### Legal stuff
Wwise and the Wwise logo are trademarks of Audiokinetic Inc., registered in the U.S. and other countries.

This project is in no way affiliated to Audiokinetic.

You still need a licensed version of Wwise installed to compile and run this project.