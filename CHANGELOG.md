# Changelog

All notable changes to **Theme Toolkit** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0-alpha] - 2026-10-15

### Added
- **Initial Tauri v2 Framework Scaffolding:**
  - Scaffolding of cross-platform Tauri v2 desktop application with Rust backend and SvelteKit frontend.
  - Per-Platform Lib Architecture (`windows.rs`, `macos.rs`, `linux.rs`) for native tool integration.
  - Per-platform Tauri configuration targets (`tauri.windows.conf.json`, `tauri.macos.conf.json`, `tauri.linux.conf.json`).
  - Standardized `.mjs` ESM runner suite under `scripts/` adhering to `main:sub:option` command hierarchy.
  - Local studio and applicator interface with live wallpaper, sound scheme, and cursor workshop navigation.
  - Lossless deskthemepack packaging pipeline blueprint utilizing Microsoft `makecab.exe` Diamond Directive Files (`.ddf`).
  - Strict adherence to Microsoft Win32 Desktop Theming, Apple Dynamic Desktop, and FreeDesktop/XDG specifications.
  - Universal `.github/workflows/cleanup-failed-runs.yml` maintenance workflow.
