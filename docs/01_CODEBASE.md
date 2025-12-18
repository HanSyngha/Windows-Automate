# AutoMate Codebase Structure

## Overview

AutoMate is an AI-powered Windows automation assistant built with:
- **Frontend**: React + TypeScript + Vite + TailwindCSS
- **Backend**: Rust + Tauri 2.x
- **AI**: OpenAI-compatible API integration

---

## Directory Structure

```
automate/
├── src/                    # Frontend (React/TypeScript)
├── src-tauri/              # Backend (Rust/Tauri)
├── docs/                   # Documentation
├── public/                 # Static assets
├── .github/                # GitHub Actions workflows
├── package.json            # Node.js dependencies
├── vite.config.ts          # Vite configuration
├── tailwind.config.js      # TailwindCSS configuration
└── tsconfig.json           # TypeScript configuration
```

---

## Frontend (`src/`)

### Entry Points
| File | Description |
|------|-------------|
| `main.tsx` | Main app entry point |
| `overlay.tsx` | Overlay window entry point |
| `App.tsx` | Root application component |

### Components (`src/components/`)
| File | Description |
|------|-------------|
| `Chat.tsx` | Main chat interface for AI interaction |
| `Settings.tsx` | Settings panel (API config, language) |
| `AddGuide.tsx` | Guide creation interface |
| `Toast.tsx` | Toast notification component |
| `UpdateChecker.tsx` | Auto-update notification UI |
| `ErrorBoundary.tsx` | React error boundary |

### Overlay Components (`src/components/overlay/`)
| File | Description |
|------|-------------|
| `OverlayApp.tsx` | Overlay root component |
| `CursorGlow.tsx` | Cursor glow effect |
| `ClickRipple.tsx` | Click ripple animation |
| `StatusIndicator.tsx` | Status indicator UI |

### State Management (`src/stores/`)
| File | Description |
|------|-------------|
| `chatStore.ts` | Chat messages state (persisted) |
| `configStore.ts` | App configuration state |
| `guideStore.ts` | Guide management state |
| `overlayStore.ts` | Overlay state |
| `toastStore.ts` | Toast notification state |

### Internationalization (`src/i18n/`)
| File | Description |
|------|-------------|
| `index.ts` | i18n configuration (ko, en, zh, ja) |

---

## Backend (`src-tauri/src/`)

### Core Files
| File | Description |
|------|-------------|
| `main.rs` | Rust entry point |
| `lib.rs` | Tauri app builder, plugin setup, tray handler |

### Commands (`src-tauri/src/commands/`)
Tauri commands exposed to frontend via `invoke()`.

| File | Commands |
|------|----------|
| `mod.rs` | `greet` |
| `screen.rs` | `capture_screen`, `get_ui_tree` |
| `input.rs` | `mouse_move`, `mouse_click`, `keyboard_type`, `keyboard_press` |
| `config.rs` | `get_config`, `save_config`, `test_api_connection` |
| `llm.rs` | `send_message` |
| `guides.rs` | `guide_list`, `guide_preview`, `guide_read`, `guide_index`, `guide_search`, `guide_create` |
| `overlay.rs` | `overlay_show`, `overlay_hide`, `overlay_cursor_move`, `overlay_click`, `overlay_status`, `overlay_set_control` |

### Screen Module (`src-tauri/src/screen/`)
| File | Description |
|------|-------------|
| `mod.rs` | Module exports |
| `capture.rs` | Screen capture using Windows GDI |
| `ui_automation.rs` | Windows UI Automation API integration |

### Input Module (`src-tauri/src/input/`)
| File | Description |
|------|-------------|
| `mod.rs` | Module exports |
| `mouse.rs` | Mouse control (move, click) |
| `keyboard.rs` | Keyboard control (type, press) |

### LLM Module (`src-tauri/src/llm/`)
| File | Description |
|------|-------------|
| `mod.rs` | Module exports |
| `client.rs` | OpenAI-compatible API client |
| `agent.rs` | AI agent logic, action parsing |

### Guides Module (`src-tauri/src/guides/`)
| File | Description |
|------|-------------|
| `mod.rs` | Module exports |
| `storage.rs` | Guide file storage (markdown) |
| `search.rs` | Guide search functionality |

### Config Module (`src-tauri/src/config/`)
| File | Description |
|------|-------------|
| `mod.rs` | Config types, defaults |
| `storage.rs` | Config persistence (JSON) |

---

## Configuration Files

### Tauri Config (`src-tauri/tauri.conf.json`)
```json
{
  "productName": "AutoMate",
  "version": "0.2.0",
  "bundle": {
    "targets": ["nsis", "msi"],
    "createUpdaterArtifacts": true
  },
  "plugins": {
    "updater": {
      "endpoints": ["...latest.json"],
      "pubkey": "..."
    }
  }
}
```

### Cargo Dependencies (`src-tauri/Cargo.toml`)
- `tauri` - Core framework
- `tauri-plugin-shell` - Shell commands
- `tauri-plugin-os` - OS information
- `tauri-plugin-process` - Process control
- `tauri-plugin-global-shortcut` - Global hotkeys
- `tauri-plugin-updater` - Auto-update
- `windows` - Windows API bindings
- `reqwest` - HTTP client
- `serde` / `serde_json` - Serialization
- `base64` / `image` - Image processing

---

## Data Flow

```
┌─────────────────────────────────────────────────────────────┐
│                      Frontend (React)                        │
│  ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────────┐  │
│  │  Chat   │   │Settings │   │  Guide  │   │UpdateChecker│  │
│  └────┬────┘   └────┬────┘   └────┬────┘   └──────┬──────┘  │
│       │             │             │               │          │
│       └─────────────┼─────────────┼───────────────┘          │
│                     │             │                          │
│              ┌──────┴─────────────┴──────┐                   │
│              │     Zustand Stores        │                   │
│              └──────────────┬────────────┘                   │
└─────────────────────────────┼────────────────────────────────┘
                              │ invoke()
┌─────────────────────────────┼────────────────────────────────┐
│                      Backend (Rust/Tauri)                    │
│              ┌──────────────┴────────────┐                   │
│              │     Tauri Commands        │                   │
│              └──────────────┬────────────┘                   │
│       ┌─────────────────────┼─────────────────────┐          │
│       │           │         │         │           │          │
│  ┌────┴────┐ ┌────┴────┐ ┌──┴──┐ ┌────┴────┐ ┌────┴────┐    │
│  │ Screen  │ │  Input  │ │ LLM │ │ Guides  │ │ Config  │    │
│  │ Capture │ │ Control │ │Agent│ │ Storage │ │ Storage │    │
│  └────┬────┘ └────┬────┘ └──┬──┘ └─────────┘ └─────────┘    │
│       │           │         │                                │
└───────┼───────────┼─────────┼────────────────────────────────┘
        │           │         │
   ┌────┴────┐ ┌────┴────┐ ┌──┴──────────┐
   │ Windows │ │ Windows │ │ OpenAI API  │
   │   GDI   │ │  Input  │ │ (Compatible)│
   └─────────┘ └─────────┘ └─────────────┘
```

---

## Key Features

### 1. AI Chat Interface
- Location: `src/components/Chat.tsx` + `src-tauri/src/llm/`
- OpenAI-compatible API
- Action parsing and execution

### 2. Screen Capture & Analysis
- Location: `src-tauri/src/screen/`
- Windows GDI for capture
- UI Automation for element detection

### 3. Input Automation
- Location: `src-tauri/src/input/`
- Mouse movement and clicks
- Keyboard typing and key presses

### 4. Guide System
- Location: `src/components/AddGuide.tsx` + `src-tauri/src/guides/`
- Markdown-based guides
- Search functionality

### 5. Visual Overlay
- Location: `src/components/overlay/`
- Cursor glow effects
- Click ripple animations
- Status indicators

### 6. Auto-Update
- Location: `src/components/UpdateChecker.tsx`
- GitHub Releases integration
- Signed updates with latest.json

---

## Build & Development

### Development
```bash
npm run tauri dev
```

### Production Build
```bash
npm run tauri build
```

### Release (GitHub Actions)
1. Update version in `tauri.conf.json` and `Cargo.toml`
2. Commit and push
3. Create tag: `git tag v0.x.x && git push origin v0.x.x`
4. GitHub Actions builds and creates release

---

## Related Documentation

- [02_REQUIREMENTS.md](./02_REQUIREMENTS.md) - Project requirements
- [03_VERSION_GUIDE.md](./03_VERSION_GUIDE.md) - Version management
- [04_RELEASE_GUIDE.md](./04_RELEASE_GUIDE.md) - Release process
