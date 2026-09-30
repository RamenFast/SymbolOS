# The chromatic orrery

SymbolOS borrows a symbolic color vocabulary from Annie Besant and C. W. Leadbeater's *Thought-Forms* (1905). Here it is an artistic reference: a way to connect a motif with a tone, not a measurement of a person's feelings or a model's activity.

The hex values below are the project's digital choices. A printed 1905 illustration does not supply a CSS color code.

## The familiar lights

- **Primrose** `#FADA5E`: careful reason and the lantern's light.
- **Gold** `#FFD700`: aspiration and a completed piece of work.
- **Gamboge** `#E49B0F`: clear thought and legibility.
- **Orange** `#FF8C00`: energy, drive and making.
- **Green** `#228B22`: adaptability and Rhy's questioning voice.
- **Deep blue** `#0000CD`: devotion and verification.
- **Azure** `#87CEEB`: the umbrella and an open-handed posture.
- **Violet** `#8B00FF`: the bridge between feeling and form.
- **Pale violet** `#DDA0DD`: ideals and poetic space.
- **Rose** `#FFB7C5`: Agape's warmth and care.
- **Carmine** `#960018`: affection and the heart motif.
- **Scarlet** `#FF2400`: a boundary that needs attention.

Mercer has often carried blue, Rhy green, Agape rose and Ben orange. These associations support a story. They are not a required theme for every project or a way to infer somebody's inner state.

## Put readability first

Use color alongside a clear word or shape. Check contrast against the actual background, including light mode, dark mode and reduced-color viewing. A symbolic association does not make low-contrast text readable.

Existing project themes and Ben's current choice take precedence. Black can be a useful background and grey a useful neutral. The older source's “shadow spectrum” is historical symbolism, not a moral verdict on those colors or a diagnostic rule.

Ring colors also depend on the version of the ring vocabulary. See the [eight-to-twelve comparison](ring_system_v2.md#reading-older-references) before assigning meaning to a numbered ring in an old diagram.

## For implementers

Prefer named theme tokens over scattered literals. The earlier palette example proposed `--tf-*` names, while [the inherited webview CSS](mercer_webview_theme_v1.css) actually uses `--mercer-*` tokens mapped from VS Code. Those are different maps. Inspect the consumer before changing either. This guide does not install a palette.

The [earlier palette and CSS sample](https://github.com/RamenFast/SymbolOS_archive/blob/ead60385ef3170028fc91606de24efa45b392034/docs/thoughtforms_colors.md) stay in the archive. [Pattern Room](style_station.md) · [Feeling and form](poetry_translation_layer.md).
