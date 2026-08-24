# WayOfTeams Installer (wot-setup)

Professional bootstrapper installer for the WayOfTeams desktop app.

## What This Is

A small (~3MB) bootstrapper that guides users through a 6-step installation wizard. When they click "Install", it downloads the full WayOfTeams app (200-400MB) from GitHub Releases and installs it.

## Architecture

```
User visits teams.zerwiz.org/download
  → Downloads bootstrapper (~3MB)
  → Runs installer wizard (6 steps)
  → Bootstrapper pulls full app from github.com/Way-Of/wayofteams-releases
  → Installs to Program Files / Applications / /opt
```

## Components

| File | Purpose |
|------|---------|
| `index.html` | Installer wizard UI (standalone, no dependencies) |
| `build.sh` | Build script for packaging installers |
| `package.json` | Node.js config for electron-builder packaging |

## Build

### Quick preview (browser)
```bash
open index.html
```

### Package as standalone installer
```bash
npm install
npm run build
```

This produces:
- `dist/WayOfTeams-Setup.exe` (Windows)
- `dist/WayOfTeams.dmg` (macOS)
- `dist/WayOfTeams.AppImage` (Linux)

## Distribution

1. Bootstrapper hosted at `teams.zerwiz.org/download`
2. Full app binaries hosted at `github.com/Way-Of/wayofteams-releases`
3. Auto-update checks GitHub API for new versions

## System Requirements

| OS | Minimum | Recommended |
|----|---------|-------------|
| Windows | Windows 10 (1903+) | Windows 11 |
| macOS | macOS 12 (Monterey) | Latest |
| Linux | Ubuntu 20.04+ / Debian 11+ | Latest |
| RAM | 4GB | 8GB |
| Disk | 1GB | 2GB |

## Installer Flow

1. **Welcome** — Branding, version, system requirements
2. **License Agreement** — Terms of service, accept checkbox
3. **Install Location** — Choose directory, disk space check
4. **Optional Components** — AI support, Anchor, OpenChamber, shortcuts
5. **Download & Install** — Progress bars, component status
6. **Complete** — Success, launch option

## Colors & Branding

Matches WayOfTeams marketing site (`teams.zerwiz.org`):

```css
--color-primary: #FF6B35;
--color-bg: #1A1A1A;
--color-surface: #222222;
--color-text-primary: #FFFFFF;
--color-text-secondary: #AAAAAA;
```

Font: Geist (Google Fonts)

## License

Proprietary — Way-Of © 2026
