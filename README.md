# Theme Toolkit

> A fast, privacy-respecting cross-platform desktop theme authoring studio and applicator for **Windows**, **macOS**, and **Linux**, built with Tauri v2, Rust, and SvelteKit.

---

## ✨ Overview

**Theme Toolkit** is a complete, dedicated desktop application designed to replace broken native theme export utilities (such as Microsoft's broken settings export integration) and provide a unified, standard-compliant theming studio.

### 🔒 100% Local Studio — No Store, No Catalogue

Unlike previous theming tools, **Theme Toolkit contains no remote store, online catalogue, or cloud telemetry**. It is engineered strictly for creating, modifying, previewing, and applying themes and desktop customizations directly on the user's local machine.

### 📦 Export to Standard Themepacks

Users can export their complete visual, audio, and cursor configurations into standard-compliant themepack archives for easy archival, backup, and one-click installation at a later time:
- **Windows:** Packages valid `.deskthemepack` archives utilizing Microsoft `makecab.exe` Diamond Directive Files (`.ddf`) with MSZIP compression, eliminating corrupted headers and asset stripping.
- **macOS:** Packages Dynamic Desktop timed/solar HEIC wallpapers, system `.soundpack` bundles, and retina cursor sets.
- **Linux:** Packages FreeDesktop Sound Theme v0.8 compliant audio archives, XDG `xcursorgen` cursor binaries, and timed wallpaper XML manifests.

---

## 🚀 Feature Capabilities

- **🎵 Sound Schemes:** Full system audio event mapping (Logon, Logoff, Alerts, Notifications, Trash, Hardware Connect) loudness-normalized to -14 LUFS.
- **🖼️ Wallpapers & Slideshows:** Single static wallpapers or multi-monitor dynamic slideshows with configurable transition intervals, shuffle modes, and standard alignment metrics (Fill, Fit, Stretch, Tile, Center, Span).
- **🖱️ Cursor Schemes:** Full configuration for 15 standard Win32 pointer roles (`.cur` / `.ani`), macOS retina pointers, and XDG cursor themes.
- **🎨 Color Editing & Visual Styles:** Live DWM accent colorization control, dark/light system and app mode switches, and control panel color palettes.
- **👁️ Live Interactive Previews:** Preview desktop wallpaper viewports, audition sound cues, playback animated cursors, and inspect theme manifests before export or application.

---

## 🛠️ Architecture

Theme Toolkit follows the HELIX-Origin Tauri v2 engineering standards:

```
projects/Theme-Toolkit/
├── src/                         <- SvelteKit / TypeScript ESM frontend studio
├── src-tauri/
│   ├── src/
│   │   ├── main.rs              <- Tauri v2 entry point
│   │   ├── lib.rs               <- Application setup & command registration
│   │   └── platforms/           <- Per-Platform Lib Architecture
│   │       ├── mod.rs           <- Cross-platform ThemeManifest contract
│   │       ├── windows.rs       <- makecab.exe DDF packaging & Win32 registry
│   │       ├── macos.rs         <- Dynamic Desktop HEIC & soundpack bundling
│   │       └── linux.rs         <- FreeDesktop sound theme & xcursorgen
│   ├── Cargo.toml               <- Rust dependencies
│   ├── tauri.conf.json          <- Base Tauri configuration
│   ├── tauri.windows.conf.json  <- Windows bundle targets (NSIS, MSI)
│   ├── tauri.macos.conf.json    <- macOS bundle targets (DMG, App)
│   └── tauri.linux.conf.json    <- Linux bundle targets (DEB, AppImage)
├── scripts/                     <- Native .mjs ESM verification & test runner
└── package.json                 <- main:sub:option command hierarchy
```

---

## 📋 Standard Commands (`main:sub:option`)

In accordance with `.agents/rules/script-standards.md`, all lifecycle scripts adhere to the canonical command hierarchy:

| Command | Action |
|---|---|
| `npm run tauri:dev` | Launch local Tauri v2 development environment |
| `npm run tauri:build:release` | Compile production desktop application release |
| `npm run tauri:build:debug` | Compile debug desktop application build |
| `npm run test:rust` | Run Rust crate unit tests |
| `npm run test:frontend` | Run SvelteKit type checks (`svelte-check`) |
| `npm run test:all` | Run complete test verification suite |
| `npm run verify` | Full static validation gate (Rust check, tests, frontend build) |

---

## 📄 License

BSD-3-Clause © [HELIX Origin](https://github.com/HELIX-Origin)
