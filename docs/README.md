# Theme Toolkit Documentation

> 📖 **Project Documentation Site**: Architectural design, user guides, platform packaging specifications, and reference manuals for **Theme Toolkit**.

---

## ⏸️ Documentation Status: On Hold

> [!NOTE]
> The full documentation site for Theme Toolkit is currently **on hold** while the core application framework and platform backends are actively being designed and developed.
>
> Formal guides, API specifications, and tutorials will be published here once the project reaches sufficient maturity and the core features are ready for user documentation.

---

## 🎯 Purpose of Theme Toolkit Documentation

When active, this documentation space will serve as the comprehensive, theme-neutral reference manual for Theme Toolkit, covering:

1. **User Guide & Workflows:**
   - Creating, editing, and previewing desktop themes locally with the Tauri v2 GUI.
   - Managing Sound Schemes, Wallpapers & Slideshows, Cursor Sets, and Color Schemes.
   - Applying and toggling desktop themes on local Windows, macOS, and Linux systems.

2. **Themepack Export & Packaging Specifications:**
   - **Windows:** Compiling lossless `.deskthemepack` archives via Microsoft Diamond Directive Files (`.ddf`) and `makecab.exe` MSZIP compression in strict adherence to Win32 theming specifications.
   - **macOS:** Generating timed and solar Dynamic Desktop HEIC containers and assembling `.soundpack` alert bundles.
   - **Linux:** Generating FreeDesktop Sound Theme v0.8 compliant audio packages, XDG `xcursorgen` multi-resolution cursor sets, and desktop environment timed background XML manifests.

3. **Audio & Asset Engineering Standards:**
   - Loudness normalization standards (-14 LUFS, -1.0 dBFS true peak) for system sound cues.
   - Cursor hotspot alignments, frame animations, and multi-DPI resolutions.
