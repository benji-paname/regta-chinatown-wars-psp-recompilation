# Branding and typeface provenance

Project: **ReGTA CTW**. Full name: **ReGTA CTW PC Recomp**. Credit: **by Benji Paname**.

The website and GitHub presentation use English as their primary language. The website also offers French and Spanish.

## Primary full-title version — 2 November 2026

The main website and README now use `docs/assets/logo-chinatown-wars.svg`, with the full **CHINATOWN / WARS** below **ReGTA**, and **PSP RECOMPILATION**, in large Pricedown lettering, as the subtitle. ReGTA reuses the exact approved Pricedown outlines, including the lowercase e. The full title uses the owner-supplied `BRUCM___.TTF` (Harold Lohner / Bruce Mikita) from the private Downloads/font folder. Its SHA256 matches the historical input fingerprint below. C and the final N are enlarged to follow the supplied cover's composition; WARS is centered on a narrower second line. Red #d90000, yellow #ffda22 and a black keyline are retained.

The old font is the engraved variant: the artwork generator omits its decorative inline highlights while preserving the actual outer contours and the A/O/R counters to create solid cover-style red lettering. The font file itself is never modified or distributed. This is independently typeset project artwork, not a scan or extraction of the official wordmark, and not a claim of pixel identity with the printed cover.

New exports: main/transparent SVG, black-background SVG, transparent PNG and black PNG (1800 × 1250). The older CTW horizontal/vertical, transparent/black versions remain unchanged and available. Reproduce with `tools/research/generate_full_title_logo.py --bruce-mikita <private BRUCM___.TTF path> --pricedown <private Pricedown Bl.otf path>` and `tools/research/render_brand_svg.ps1` (Windows WPF, no downloaded renderer). Owner authorization already confirmed on 2026-11-02 applies to the supplied font artwork; no font software is published.

## Requested final typography

- **GTA:** Pricedown, in the heavy/Black style associated with the series. [Raymond Larabie's history](https://typodermicfonts.com/between-the-lines/stories/pricedown/) confirms its use by Rockstar. [Publisher page](https://typodermicfonts.com/pricedown/) describes desktop licenses for logos and static graphics.
- **CHINATOWN WARS / CTW:** the font identified for the title is **Bruce Mikita**, not Chinese Rocks. [Historical identification](https://m.thegtaplace.com/downloads/f2974-chinatown-wars-font-bruce-mikita); [Harold Lohner's current publisher page](https://www.haroldsfonts.com/font/bruce-mikita) offers Regular and Solid variants.

## Implemented artwork â€” 25 October 2026

Public name: **ReGTA CTW PC Recomp**. The PSP remains the source platform, not the destination in the project title.

- Pricedown Black supplies the whole **ReGTA** wordmark in warm white. The lowercase **e** is scaled to 78% on the common baseline to make its case visible; R remains as large as GTA.
- Bruce Mikita Two supplies solid **CTW**, red `#d90000` with yellow `#ffda22` outline and black keyline. These are chosen screen colours matching the supplied cover's red/yellow direction, not a claim of calibrated print-colour identity.
- Flat colours replace the previous metallic gradients, inset ornament and diamond separator. Subtitle: **PC RECOMP**.

| Composition | Transparent | Black background | SVG / PNG dimensions |
| --- | --- | --- | --- |
| Horizontal | `logo-ctw-transparent.svg` / `.png` | `logo-ctw-black.svg` / `.png` | 1200 Ã— 280 / 2400 Ã— 560 |
| Vertical | `logo-transparent.svg` / `.png` | `logo-black.svg` / `.png` | 900 Ã— 540 / 1800 Ã— 1080 |

Files are in `docs/assets/` and mirrored into local `outputs/CTW-Rust/branding/`. `logo.svg/png` and `logo-ctw.svg/png` remain transparent aliases for existing website links. Local `logo-variants-preview.png` shows all four variants.

SVGs were rendered with Sharp, without browser automation. Both compositions and black/transparent variants are inspected through local raster previews. Typeface software is not embedded or redistributed.

## Terms and publication status

The supplied Pricedown archive includes **Typodermic Desktop EULA v260817**, effective 17 August 2026. Section 2.2 permits fixed logos and static artwork, including commercial materials, without redistributing the font or providing a reusable font capability. The desktop terms are not a GPL license for the font software.

The supplied Bruce Mikita archive contains fonts and a wfonts.com download note, but no explicit license. Its font metadata identifies Harold Lohner and copyright 2000, with no license text or license URL. **Historical finding (superseded by owner confirmation on 2026-11-02): permission had not been established in the old audit.** Benjamin explicitly confirmed on 2026-11-02 that he holds a license or authorization for the branding. This records the owner’s confirmation; no license document was independently reviewed. Font software remains private. The current publisher's offer does not by itself establish the terms of this old copy.

The files remain private under ignored `target/private-fonts/`. No font software or license PDF is committed. No purchase or publication has been performed. This record documents the evidence, not a legal clearance.

## Reproduce the artwork

Install `fonttools` in a local Python environment, then run the generator:

```powershell
python tools/research/generate_brand_logos.py
```

The generator writes SVG variants; render at 2Ã— with Sharp for PNGs. Glyph shapes remain proportional; the lowercase e has an intentional size adjustment. No font dependency is needed to view the outlined wordmark.

## Input fingerprints

- `Pricedown Bl.otf` â€” SHA-256 `19f8cd90ce76992c565debe80d167f58e6e1e79a6e0b86f24bd9dce12052b256`
- `BRUCM___.TTF` â€” SHA-256 `6c6700ee98153ca00d58284b294f196aed291141ba1dad4915efb559cbf602f2`

- `Bruce Mikita Two.ttf` â€” SHA-256 `f6caa4595aa6ad15d316bcc5a757b0cadd439ee36ae26ab0b08d625da7dcbca1`
