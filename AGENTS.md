# wot-setup — AGENTS.md

WayOfTeams Desktop Installer (bootstrapper).

## Purpose

Small (~3MB) bootstrapper installer that guides users through a 6-step installation wizard. Downloads the full WayOfTeams app (200-400MB) from GitHub Releases and installs it on Windows, macOS, or Linux.

## Stack

| Component | Version |
|-----------|---------|
| Rust | 1.98 |
| Tauri | 2.x |
| reqwest | 0.12 |
| tokio | 1.x |
| zip | 2.x |

## Repository Structure

```
wot-setup/
├── src-tauri/              # Tauri Rust app
│   ├── Cargo.toml          # Rust dependencies
│   ├── tauri.conf.json     # Tauri config (window, bundle, security)
│   ├── build.rs            # Tauri build script
│   ├── src/
│   │   └── main.rs         # Rust entry point + Tauri commands
│   ├── icons/              # App icons (32x32, 128x128, .ico, .icns)
│   └── resources/          # Bundled resources
├── index.html              # Standalone installer wizard UI (preview)
├── dist/                   # Built frontend assets
├── docs/
│   └── legal/              # License documents
│       ├── EULA.md
│       ├── Terms-of-Service.md
│       └── Privacy-Policy.md
├── LICENSE                 # MIT License
├── README.md               # Project documentation
├── AGENTS.md               # This file
└── package.json            # Node.js config (electron-builder fallback)
```

## Distribution Strategy

| Asset | Location |
|-------|----------|
| Bootstrapper installer | `teams.zerwiz.org/download` (served from server) |
| Full app binaries | `github.com/Way-Of/wayofteams-releases` (public repo) |
| Auto-update manifest | `github.com/Way-Of/wayofteams-releases/releases/latest` |

**Flow:**
1. User visits `teams.zerwiz.org/download`
2. Downloads bootstrapper (~3MB)
3. Bootstrapper pulls full app from `github.com/Way-Of/wayofteams-releases`
4. Installs to platform-specific location

## Installer Flow (6 Steps)

| Step | Screen | User Action |
|------|--------|-------------|
| 1 | Welcome | Click Next |
| 2 | License Agreement | Accept + Next |
| 3 | Install Location | Browse or Default + Next |
| 4 | Optional Components | Check/uncheck + Next |
| 5 | Download & Install | Wait |
| 6 | Complete | Finish (launch option) |

## System Requirements

| OS | Minimum | Recommended |
|----|---------|-------------|
| Windows | Windows 10 (1903+) | Windows 11 |
| macOS | macOS 12 (Monterey) | Latest |
| Linux | Ubuntu 20.04+ / Debian 11+ | Latest |
| RAM | 4GB | 8GB |
| Disk | 1GB | 2GB |

## Key Commands

```bash
# Preview installer in browser
open index.html

# Build Tauri app
cargo tauri build

# Dev mode
cargo tauri dev
```

## Build Artifacts

| Platform | Output |
|----------|--------|
| Windows | `WayOfTeams-Setup.exe` (NSIS) |
| macOS | `WayOfTeams.dmg` |
| Linux | `WayOfTeams.AppImage` |

## WayOfTeams Colors

```css
--color-primary: #FF6B35;
--color-bg: #1A1A1A;
--color-surface: #222222;
--color-surface-elevated: #2A2A2A;
--color-border: #333333;
--color-border-strong: #444444;
--color-text-primary: #FFFFFF;
--color-text-secondary: #AAAAAA;
--color-text-muted: #666666;
--color-success: #22C55E;
--color-warning: #F59E0B;
--color-info: #3B82F6;
```

Font: Geist (Google Fonts)

## Related Repos

| Repo | Visibility | Purpose |
|------|-----------|---------|
| `Way-Of/wayofteams` | Private | Main app code |
| `Way-Of/wayofteams-releases` | Public | Desktop app binaries + auto-update |
| `Way-Of/wot-setup` | Public | This installer repo |
