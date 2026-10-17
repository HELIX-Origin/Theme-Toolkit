<div align="center">

# Theme Toolkit

<p align="center">
  <a href="https://github.com/HELIX-Origin/theme-toolkit/releases"><img src="https://img.shields.io/github/v/release/HELIX-Origin/theme-toolkit?include_prereleases&label=Release&color=blue" alt="Release" height="20" /></a>
  <a href="https://docs.helix-origin.club/theme-toolkit/"><img src="https://img.shields.io/badge/Docs-docs.helix--origin.club-brightgreen" alt="Documentation" height="20" /></a>
  <a href="https://wiki.helix-origin.club/theme-toolkit/"><img src="https://img.shields.io/badge/Wiki-wiki.helix--origin.club-brightgreen" alt="Technical Wiki" height="20" /></a>
  <a href="https://opensource.org/licenses/BSD-3-Clause"><img src="https://img.shields.io/badge/License-BSD--3--Clause-blue.svg" alt="License: BSD-3-Clause" height="20" /></a>
  <a href="https://tauri.app"><img src="https://img.shields.io/badge/Tauri-v2-blue?logo=tauri" alt="Tauri v2" height="20" /></a>
</p>

**A fast, privacy-respecting cross-platform desktop theme authoring studio and applicator for Windows, macOS, and Linux — built with Tauri v2.**

Create, customize, preview, and package complete visual, audio, cursor, and wallpaper themes into native, standard-compliant themepacks (`.deskthemepack`, Dynamic HEIC, FreeDesktop sound schemes) with zero remote store dependencies or telemetry.

</div>

---

## 📖 Primary Gateways

All comprehensive user guides, themepack specifications, and technical architecture manuals are centrally hosted:

- 📖 **Documentation**: [docs.helix-origin.club/theme-toolkit](https://docs.helix-origin.club/theme-toolkit/) — User guide, sound schemes, cursors, and themepack exporting.
- 🧠 **Technical Wiki**: [wiki.helix-origin.club/theme-toolkit](https://wiki.helix-origin.club/theme-toolkit/) — Architecture, per-platform lib isolation, and native packaging specs.
- 🌐 **Organization Portal**: [www.helix-origin.club](https://www.helix-origin.club/) — HELIX Origin organization hub.

---

## ⚡ Quick Start

### Prerequisites
- [Node.js](https://nodejs.org/) (v20+)
- [Rust](https://www.rust-lang.org/) (1.80+)

### Running Locally
```bash
npm install
npm run tauri:dev
```

### Verification & Quality Gate
```bash
npm run verify           # Full static validation gate (Rust check, tests, frontend build)
```

---

## ⚖️ License

[BSD-3-Clause](LICENSE) — Copyright (c) 2026, HELIX Origin.
