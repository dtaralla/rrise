/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

mod common;

use common::*;
use rrise::AKRESULT;

#[test]
fn static_link_all() -> Result<(), AKRESULT> {
    init_sound_engine()?;
    rrise::sound_engine::render_audio(false)?;
    term_sound_engine()?;
    Ok(())
}
