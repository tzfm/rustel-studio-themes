//! Theme documents for the Rustel studio.
//!
//! Each name loads one JSON document. The studio parses the document.

/// Catalogue themes, in picker order.
pub fn catalogue() -> &'static [(&'static str, &'static str)] {
    CATALOGUE
}

/// Showroom themes, in picker order.
pub fn showroom() -> &'static [(&'static str, &'static str)] {
    SHOWROOM
}

const CATALOGUE: &[(&str, &str)] = &[
    // rustel-dark with a slow drift breathing behind the code: the theme
    // that shows a theme can carry a sketch of its own.
    ("rustel-drift", include_str!("../themes/rustel-drift.json")),
    // Camera-backed themes. They request the camera only while their own
    // picture is visible; the persisted webcam permission still owns
    // whether a device may open. The first three go through Hydra and its
    // `s0` texture; the fourth is drawn by the studio itself, in Braille,
    // and so needs no GPU and no sketch.
    ("webcam-clean", include_str!("../themes/webcam-clean.json")),
    (
        "webcam-distort",
        include_str!("../themes/webcam-distort.json"),
    ),
    (
        "webcam-hacker",
        include_str!("../themes/webcam-hacker.json"),
    ),
    (
        "webcam-braille",
        include_str!("../themes/webcam-braille.json"),
    ),
    // An independently authored feedback current from the snippet shelf.
    ("prism", include_str!("../themes/prism.json")),
    ("rustel-light", include_str!("../themes/rustel-light.json")),
    ("strudel", include_str!("../themes/strudel.json")),
    ("powershell", include_str!("../themes/powershell.json")),
    ("rustel-live", include_str!("../themes/rustel-live.json")),
    ("synthwave", include_str!("../themes/synthwave.json")),
    ("vectrex", include_str!("../themes/vectrex.json")),
    ("mode7", include_str!("../themes/mode7.json")),
    // Black ground, phosphor green, and a slow rain behind the code.
    ("matrix", include_str!("../themes/matrix.json")),
    ("space", include_str!("../themes/space.json")),
    ("campfire", include_str!("../themes/campfire.json")),
    ("basketball", include_str!("../themes/basketball.json")),
    ("forest", include_str!("../themes/forest.json")),
    ("bear", include_str!("../themes/bear.json")),
    ("mono", include_str!("../themes/mono.json")),
    // No palette of its own: `reset` and the terminal's 16 colours, so the
    // studio wears whatever palette the terminal already carries.
    ("terminal", include_str!("../themes/terminal.json")),
    // The same ground with one role moved: the accent and the selection
    // take the palette's slot 1, which reads warm in palettes whose
    // bright slot is gold.
    (
        "terminal-warm",
        include_str!("../themes/terminal-warm.json"),
    ),
    ("solarized", include_str!("../themes/solarized.json")),
    ("ember", include_str!("../themes/ember.json")),
    ("hell", include_str!("../themes/hell.json")),
    ("lavender", include_str!("../themes/lavender.json")),
    // Ports of the palettes people already read code in.
    ("one-dark", include_str!("../themes/one-dark.json")),
    ("dracula", include_str!("../themes/dracula.json")),
    ("tokyo-night", include_str!("../themes/tokyo-night.json")),
    (
        "catppuccin-mocha",
        include_str!("../themes/catppuccin-mocha.json"),
    ),
    (
        "catppuccin-latte",
        include_str!("../themes/catppuccin-latte.json"),
    ),
    ("nord", include_str!("../themes/nord.json")),
    ("ayu-mirage", include_str!("../themes/ayu-mirage.json")),
    ("github-light", include_str!("../themes/github-light.json")),
    (
        "solarized-light",
        include_str!("../themes/solarized-light.json"),
    ),
    ("emacs", include_str!("../themes/emacs.json")),
    // And some with a mood of their own.
    ("unicorn", include_str!("../themes/unicorn.json")),
    // The dark palette with every hue rolling through the code.
    ("rainbow", include_str!("../themes/rainbow.json")),
    // Everything at once, on purpose: a Hydra sketch under an aurora over
    // the glyphs, a lit current line, and a palette
    // where nothing agrees with anything. The one theme that is not trying
    // to stay out of the way.
    ("lsd", include_str!("../themes/lsd.json")),
];

// Fixed effect presets live in JSON too, with independently editable defaults.
const SHOWROOM: &[(&str, &str)] = &[
    ("beams", include_str!("../themes/beams.json")),
    ("binarypath", include_str!("../themes/binarypath.json")),
    ("blackhole", include_str!("../themes/blackhole.json")),
    ("bouncyballs", include_str!("../themes/bouncyballs.json")),
    ("bubbles", include_str!("../themes/bubbles.json")),
    ("burn", include_str!("../themes/burn.json")),
    ("colorshift", include_str!("../themes/colorshift.json")),
    ("crumble", include_str!("../themes/crumble.json")),
    ("decrypt", include_str!("../themes/decrypt.json")),
    ("dvd-bounce", include_str!("../themes/dvd-bounce.json")),
    ("errorcorrect", include_str!("../themes/errorcorrect.json")),
    ("expand", include_str!("../themes/expand.json")),
    ("fireworks", include_str!("../themes/fireworks.json")),
    ("highlight", include_str!("../themes/highlight.json")),
    ("leak", include_str!("../themes/leak.json")),
    (
        "orbittingvolley",
        include_str!("../themes/orbittingvolley.json"),
    ),
    ("overflow", include_str!("../themes/overflow.json")),
    ("print", include_str!("../themes/print.json")),
    ("rain", include_str!("../themes/rain.json")),
    ("scramble", include_str!("../themes/scramble.json")),
    ("rings", include_str!("../themes/rings.json")),
    ("scattered", include_str!("../themes/scattered.json")),
    ("slice", include_str!("../themes/slice.json")),
    ("smoke", include_str!("../themes/smoke.json")),
    ("spotlights", include_str!("../themes/spotlights.json")),
    ("snow", include_str!("../themes/snow.json")),
    ("swarm", include_str!("../themes/swarm.json")),
    ("sweep", include_str!("../themes/sweep.json")),
    ("synthgrid", include_str!("../themes/synthgrid.json")),
    ("thunderstorm", include_str!("../themes/thunderstorm.json")),
    ("unstable", include_str!("../themes/unstable.json")),
    ("vhstape", include_str!("../themes/vhstape.json")),
    ("waves", include_str!("../themes/waves.json")),
    (
        "waves-reactive",
        include_str!("../themes/waves-reactive.json"),
    ),
    ("wipe", include_str!("../themes/wipe.json")),
];

#[cfg(test)]
mod tests {
    use super::{catalogue, showroom};

    #[test]
    fn names_are_unique_and_leave_the_default_theme_out() {
        let mut seen = Vec::new();
        for (name, document) in catalogue()
            .iter()
            .copied()
            .chain(showroom().iter().copied())
        {
            assert!(document.starts_with('{'), "{name}");
            assert!(!seen.contains(&name), "{name} is listed twice");
            assert_ne!(name, "rustel-dark", "{name} stays in the studio");
            seen.push(name);
        }
        assert!(seen.len() > 1, "the pack is empty");
    }
}
