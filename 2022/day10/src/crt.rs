use std::fmt::Display;

use log::{debug, trace};

const DARK: char = '.';
const LIT: char = '#';
const SPRITE_SIZE: u32 = 3;
const LINE_LENGTH: u32 = 40;
const HEIGHT: u32 = 6;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum BeamSymbol {
    Dark,
    Lit,
}
impl From<BeamSymbol> for char {
    fn from(s: BeamSymbol) -> Self {
        match s {
            BeamSymbol::Dark => DARK,
            BeamSymbol::Lit => LIT,
        }
    }
}
impl Display for BeamSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BeamSymbol::Dark => DARK,
                BeamSymbol::Lit => LIT,
            }
        )
    }
}

// A real CRT just blindly traces a beam across the tube and persistence of vision does the rest
// Since our program doesn't have human eyes, let's write the Beam values into a framebuffer.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Crt {
    beam_pos: u32,
    line_pos: u32,
    fb: Vec<BeamSymbol>,
}
impl Crt {
    pub fn new() -> Self {
        Crt {
            beam_pos: 0,
            line_pos: 0,
            fb: vec![BeamSymbol::Dark; (LINE_LENGTH * HEIGHT) as usize],
        }
    }
    pub fn tick(&mut self, sprite_pos: i32) {
        debug!(
            "Sprite range: {}->{}, beam at {}",
            sprite_pos - (SPRITE_SIZE / 2) as i32,
            sprite_pos + (SPRITE_SIZE / 2) as i32,
            self.beam_pos
        );
        let fb_pos = (self.beam_pos + (self.line_pos * LINE_LENGTH)) as usize;
        if sprite_pos.abs_diff(self.beam_pos as i32) <= SPRITE_SIZE / 2 {
            trace!("Lighting up framebuffer position: {}", fb_pos);
            let _ = std::mem::replace(&mut self.fb[fb_pos], BeamSymbol::Lit);
        } else {
            trace!("Darkening up framebuffer position: {}", fb_pos);
            let _ = std::mem::replace(&mut self.fb[fb_pos], BeamSymbol::Dark);
        }
        let old_beam = self.beam_pos;
        self.beam_pos = (self.beam_pos + 1) % LINE_LENGTH;
        if old_beam > self.beam_pos {
            // Wrapped End of line
            self.line_pos = (self.line_pos + 1) % HEIGHT;
        }
    }
}
impl Default for Crt {
    fn default() -> Self {
        Self::new()
    }
}
impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.fb.chunks_exact(LINE_LENGTH as usize) {
            write!(f, "|")?;
            for c in line {
                write!(f, "{}", c)?
            }
            writeln!(f, "|")?
        }
        Ok(())
    }
}
