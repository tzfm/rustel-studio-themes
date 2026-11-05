//! The theme values the engines read.
//!
//! The studio keeps the theme document. These types are the part a theme
//! file can name and the part the engines paint from.

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

/// A native cell effect available to a theme.
///
/// This is a closed vocabulary on purpose. A downloaded theme may select a
/// renderer and tune it, but it cannot execute native code inside the studio.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CellEffect {
    Beams,
    BinaryPath,
    Blackhole,
    BouncyBalls,
    Bubbles,
    Burn,
    ColorShift,
    Crumble,
    Cyberpunk,
    Decrypt,
    Dvd,
    ErrorCorrect,
    Expand,
    Fireworks,
    Highlight,
    LaserEtch,
    Matrix,
    MiddleOut,
    OrbittingVolley,
    Overflow,
    #[serde(alias = "pour")]
    Leak,
    /// A sun, planets on their orbits, and stars drifting past.
    Space,
    /// A fire between logs, a tent beside it, the night over both.
    Campfire,
    /// A lava sea, flames over it, embers up and ash down, a sky that beats.
    Hell,
    /// A player shooting at a hoop, making some and missing some.
    Basketball,
    /// A flat picture in perspective, driven toward the viewer: a SNES track.
    Mode7,
    /// A clearing: trees, flowers, animals dancing to the music.
    Forest,
    /// A bear asleep against a tree, until the music wakes him.
    Bear,
    /// The camera drawn as Braille dots. The only effect that asks the
    /// studio for something outside itself, and the only one that draws
    /// nothing recognisable until the webcam switch in Settings allows it.
    /// Not aliased to `braille`: the dock's spectrum already has a style
    /// by that name meaning something else, and a Braille effect that is
    /// not the camera would want it.
    #[serde(alias = "webcam")]
    Camera,
    Print,
    Rain,
    #[serde(rename = "randomize", alias = "randomsequence")]
    RandomSequence,
    /// A vector tunnel flying past stars and depth rings, drawn the way
    /// a Vectrex drew.
    #[serde(alias = "retro3d")]
    Vectrex,
    RustelBrand,
    Rings,
    Scattered,
    Slice,
    Slide,
    Smoke,
    #[serde(alias = "spray")]
    Snow,
    Spotlights,
    Swarm,
    Sweep,
    SynthGrid,
    Thunderstorm,
    Unstable,
    VhsTape,
    Waves,
    WavesReactive,
    Wipe,
}

/// How a TachyonFX theme reaches the terminal.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TachyonMode {
    /// Transform the finished Ratatui cells, including their characters.
    #[default]
    Text,
    /// Convert the transformed cells into a stretched RGBA backdrop.
    Image,
}

/// A bounded animation applied to score glyphs after the backdrop is composed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CharacterEffect {
    /// Each glyph breathes on its own phase.
    Fade,
    /// Travelling colour fields meet across the source text.
    Aurora,
    /// Every hue across the text, rolling with time.
    Rainbow,
    /// Scan light and chromatic interference pass over the glyphs.
    Hologram,
}

/// Per-theme character animation, independent of the background renderer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharacterVisual {
    pub effect: CharacterEffect,
    #[serde(default = "default_character_strength")]
    pub strength: u8,
    #[serde(default = "default_character_speed")]
    pub speed: u8,
}

impl CharacterVisual {
    /// A character effect at the default strength and speed.
    pub fn new(effect: CharacterEffect) -> Self {
        Self {
            effect,
            strength: default_character_strength(),
            speed: default_character_speed(),
        }
    }
}

const fn default_character_strength() -> u8 {
    60
}

const fn default_character_speed() -> u8 {
    50
}

/// A validated TachyonFX renderer selection, cheap to copy into the frame loop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CellVisual {
    pub effect: CellEffect,
    pub mode: TachyonMode,
    pub density: u8,
    pub speed: u8,
}

/// The colours and visuals a theme hands to the engines.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ThemePaint {
    pub cell: Option<CellVisual>,
    pub characters: Option<CharacterVisual>,
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
    pub secondary: Color,
    pub muted: Color,
}
