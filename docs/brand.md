# The brand layer

Atelier Lens is LightCraft with a different name on the door. We keep the change small on purpose:
upstream lands many changes a day, and every file we touch is a file that can conflict when we
pull them in. So:

**Renamed (what a person sees):** the window titles, menus (including the macOS app menu), dialogs,
About (with the Nation button and a line on where Lens comes from), the top-bar Nation button,
Help-menu links (the Nation, Atelier, this repository and its docs), messages in the UI crate and
the apps, the preset type in file dialogs, the default export folder (`~/Pictures/Atelier Lens
Exports`), the MCP server title, installer and package names, vendor and links, the browser page,
the app icon, and where settings live (`~/Library/Application Support/Atelier Lens`,
`%APPDATA%\Atelier Lens`).

**Not renamed (machine names):** crate names (`lightcraft-*`), the binaries (`lightcraft`,
`lightcraft-cli`), command ids (`app.discord` opens the Nation, `app.artcraft` the Atelier suite)
and automation ids, the `ai.storyteller.lightcraft` app id and icon file names in packaging, the
library folder (`~/Pictures/LightCraft Library`, so existing libraries open), the library and
`.lcpreset` formats, the software tags written into XMP sidecars and exports, the library lock's
program name, environment variables (`LIGHTCRAFT_*`), the Linux settings folder
(`~/.config/lightcraft`), the Windows registry key, and the docs under `docs/`.

## Where the name lives

| Place | File |
|---|---|
| The constants and the translation rewrite | `crates/ui-egui/src/identity.rs` (`PRODUCT`, `UPSTREAM`, `rewrite`, `retired`, `apply`) |
| Our own strings, with translations | `crates/ui-egui/src/brand.tsv` (added to each catalog as it loads; the upstream `locales/*.json` catalogs stay untouched) |
| Format-string catalogs | `crates/ui-egui/build.rs` (`atelier_rewrite`, the same rule as `identity::rewrite`) |
| English UI strings | literal `"Atelier Lens"` in `crates/ui-egui/src` and `apps/` |
| Links | `crates/ui-egui/src/links.rs` (command ids keep their upstream names, the URLs are ours) |
| Settings folder | `crates/engine/src/config.rs` (`config_dir`) |
| MCP | `crates/mcp/src/{server,backend}.rs` |
| Windows file properties | `apps/lightcraft/build.rs` |
| macOS bundle | `packaging/macos/Info.plist.in` (display name, copyright, document type; identifier unchanged) |
| Windows installer | `packaging/windows/lightcraft.wxs` (names, manufacturer, links, finish page; registry key and upgrade code unchanged) |
| Linux desktop entry, AppStream, nfpm, Nix | `packaging/linux/ai.storyteller.lightcraft.*`, `nfpm.yaml`, `nix/package.nix` (homepage) |
| Browser build | `apps/lightcraft-web/index.html` |
| README and NOTICE | README: our header above upstream's feature sections; NOTICE: ours on top of theirs, kept on merge by `.gitattributes` |
| App icon | `assets/app-icon/` (ours; row in `assets/ATTRIBUTION.md`) |

Still upstream's name, for later passes: messages from the engine crates (library, denoise and
face-model errors), the browser backup's README text, and the `docs/` manuals. For the packaging
pass (real installers): the macOS `.app` bundle and DMG volume name (`packaging/macos/package.sh`),
the DMG window background, and the package scripts' file names.
