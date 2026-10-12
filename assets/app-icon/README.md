# Atelier Lens app icon

**The hidden sun behind a camera lens**: an ivory disc rising behind an ochre lens ring, its
aperture stopped down to a hexagon, with a small ivory glint, on a volcanic black field. Three
colours, from the Nation's palette: volcanic black `#141210`, ivory `#efe6d2`, ochre `#d9a441`.

## Geometry

A 512 × 512 viewBox, clipped to a rounded square with `rx=112`, as all the Atelier icons. Windows
and Linux use the full tile. The macOS icons (`.icns`, `lightcraft-macos-512.png`) sit on Apple's
grid: an 824/1024 body with a transparent margin.

## Provenance

Drawn for Atelier Lens by hand as SVG (2026). It replaces LightCraft's lynx icon, which stays in
LightCraft's own repository. Licence: `LICENSE.txt` (MIT OR Apache-2.0). The file names keep their
upstream `lightcraft` names so packaging stays untouched.

## Files

| File | Use |
|---|---|
| `lightcraft.svg` | Canonical artwork |
| `lightcraft-small.svg` | The same artwork (upstream kept a lighter trace here); also `hicolor/scalable` |
| `lightcraft-1024.png` | 1024 px render of the full tile |
| `lightcraft-macos-512.png` | Runtime Dock icon on macOS |
| `lightcraft.icns` | macOS bundle icon |
| `lightcraft.ico` | Windows exe icon, 16–256 px |
| `hicolor/<size>/apps/ai.storyteller.lightcraft.png` | Linux icon theme, 16–512 px |

`packaging/icons.sh` regenerates the renders (needs resvg; iconutil on macOS for the `.icns`). The
committed renders were made from the same SVG with cairosvg and Pillow.
