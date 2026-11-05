//! Read a shipped theme document into the colours the engines paint.

use serde::Deserialize;

use crate::CellEffect;

#[cfg(feature = "pack")]
use crate::{CellVisual, TachyonMode, ThemePaint};
#[cfg(feature = "pack")]
use ratatui::style::Color;

#[cfg(feature = "pack")]
pub(crate) struct PackedTheme {
    pub paint: ThemePaint,
    pub surface: Color,
}

#[cfg(feature = "pack")]
pub(crate) fn packed(name: &str) -> PackedTheme {
    let (_, document) = crate::catalogue()
        .iter()
        .copied()
        .chain(crate::showroom().iter().copied())
        .find(|(key, _)| *key == name)
        .unwrap_or_else(|| panic!("theme {name} is not in the pack"));
    let document: Document = serde_json::from_str(document)
        .unwrap_or_else(|error| panic!("theme {name} is malformed: {error}"));
    let cell = document.visual.and_then(|visual| {
        if visual.renderer != "tachyonfx" {
            return None;
        }
        Some(CellVisual {
            effect: visual
                .effect
                .unwrap_or_else(|| panic!("theme {name} names no cell effect")),
            mode: visual.mode,
            density: visual.density,
            speed: visual.speed,
        })
    });
    PackedTheme {
        surface: hex_color(&document.surface),
        paint: ThemePaint {
            cell,
            characters: None,
            background: hex_color(&document.background),
            foreground: hex_color(&document.foreground),
            accent: hex_color(&document.accent),
            secondary: hex_color(&document.syntax.number),
            muted: hex_color(&document.muted),
        },
    }
}

pub(crate) fn cell_effect(document: &str) -> Option<CellEffect> {
    let visual = serde_json::from_str::<EffectDocument>(document)
        .expect("a shipped theme document parses")
        .visual?;
    if visual.renderer != "tachyonfx" {
        return None;
    }
    visual.effect
}

#[cfg(feature = "pack")]
fn hex_color(text: &str) -> Color {
    let hex = text.trim_start_matches('#');
    assert_eq!(hex.len(), 6, "colour {text} is not #RRGGBB");
    let raw = u32::from_str_radix(hex, 16).unwrap_or_else(|_| panic!("colour {text} is not hex"));
    Color::Rgb((raw >> 16) as u8, (raw >> 8) as u8, raw as u8)
}

#[cfg(feature = "pack")]
fn default_density() -> u8 {
    70
}

#[cfg(feature = "pack")]
fn default_speed() -> u8 {
    55
}

#[cfg(feature = "pack")]
#[derive(Deserialize)]
struct Document {
    background: String,
    surface: String,
    foreground: String,
    muted: String,
    accent: String,
    syntax: SyntaxColors,
    visual: Option<Visual>,
}

#[cfg(feature = "pack")]
#[derive(Deserialize)]
struct SyntaxColors {
    number: String,
}

#[cfg(feature = "pack")]
#[derive(Deserialize)]
struct Visual {
    renderer: String,
    #[serde(default)]
    effect: Option<CellEffect>,
    #[serde(default)]
    mode: TachyonMode,
    #[serde(default = "default_density")]
    density: u8,
    #[serde(default = "default_speed")]
    speed: u8,
}

#[derive(Deserialize)]
struct EffectDocument {
    visual: Option<EffectVisual>,
}

#[derive(Deserialize)]
struct EffectVisual {
    renderer: String,
    #[serde(default)]
    effect: Option<CellEffect>,
}
