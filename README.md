# AutoMate - AI-Powered Windows Automation Assistant

<div align="center">

![AutoMate Logo](public/icon.svg)

**Automate any Windows task with AI vision and natural language commands**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-2.x-24C8D8?logo=tauri)](https://tauri.app)
[![React](https://img.shields.io/badge/React-19-61DAFB?logo=react)](https://react.dev)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)](https://rust-lang.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.x-3178C6?logo=typescript)](https://typescriptlang.org)
[![Release](https://img.shields.io/github/v/release/HanSyngha/Windows-Automate?include_prereleases)](https://github.com/HanSyngha/Windows-Automate/releases)

[Download](#download) • [Features](#features) • [Usage](#usage) • [Development](#development) • [Contributing](#contributing)

</div>

---

## Overview

**AutoMate** is an AI-powered desktop automation tool for Windows that can see your screen and control your mouse and keyboard to perform tasks on your behalf. Simply describe what you want to do in natural language, and AutoMate will analyze your screen using vision AI and execute the necessary actions.

### Key Capabilities

- **Screen Understanding**: Captures and analyzes your screen using Vision AI (GPT-4o, Claude, etc.)
- **UI Element Detection**: Uses Windows UI Automation API for precise element identification
- **Natural Mouse Movement**: Bezier curve-based smooth cursor animation
- **Keyboard Automation**: Full keyboard control with hotkey support
- **Custom Guides**: Create and store task guides for AI reference
- **Visual Feedback**: Real-time overlay showing AI actions (cursor glow, click effects)
- **Multi-language**: Supports English and Korean

---

## Features

### AI Agent with Vision

AutoMate uses OpenAI-compatible APIs to understand your screen and execute multi-step tasks autonomously.

```
User: "Open Chrome and search for 'best restaurants near me'"

AutoMate:
1. Analyzes screen to find Chrome icon
2. Double-clicks to open Chrome
3. Locates search bar
4. Types the search query
5. Presses Enter
```

### Smart Guide System

Create custom guides for complex workflows. The AI will reference these guides when performing related tasks.

- Natural language input → Structured markdown guide
- Hierarchical organization (websites/, applications/, workflows/)
- Automatic guide search using sub-agent

### Visual Effects Overlay

See exactly what the AI is doing with real-time visual feedback:

- **Cursor Glow**: Pulsing indicator around AI-controlled cursor
- **Click Ripples**: Visual feedback on mouse clicks
- **Status Indicator**: Shows current action (Thinking, Moving, Clicking, Typing)

---

## Download

### For Users

**No installation of Node.js or Rust required!** Just download and run the installer.

1. Go to [Releases](https://github.com/HanSyngha/Windows-Automate/releases)
2. Download the latest `AutoMate_x.x.x_x64-setup.exe`
3. Run the installer and follow the setup wizard
4. Launch AutoMate from the Start Menu

**System Requirements:**
- Windows 10/11 (64-bit)
- Internet connection for AI features
- OpenAI API key (or compatible API)

### Auto-Update

AutoMate automatically checks for updates on startup. When a new version is available, you'll see a notification with the option to download and install immediately.

---

## Installation (For Developers)

### Prerequisites

- **Windows 10/11** (64-bit)
- **Node.js** 20.x or later
- **Rust** 1.71 or later
- **Visual Studio Build Tools** with C++ workload

### Quick Start

1. **Clone the repository**

```bash
git clone https://github.com/HanSyngha/Windows-Automate.git
cd Windows-Automate
```

2. **Install dependencies**

```bash
npm install
```

3. **Run in development mode**

```bash
npm run tauri dev
```

4. **Build for production**

```bash
npm run tauri build
```

The installer will be available in `src-tauri/target/release/bundle/`.

---

## Usage

### Initial Setup

1. Launch AutoMate
2. Click the **Settings** icon (gear) in the top right
3. Configure your API:
   - **Endpoint**: `https://api.openai.com/v1` (or compatible endpoint)
   - **API Key**: Your API key
   - **Model**: `gpt-4o` (recommended for vision)
   - Enable **Vision Support**
4. Click **Test Connection** to verify
5. Save settings

### Basic Commands

Simply type what you want to do:

| Command | Description |
|---------|-------------|
| "Open Notepad" | Launches Notepad application |
| "Click the Start button" | Clicks Windows Start menu |
| "Type 'Hello World'" | Types text at cursor position |
| "Press Ctrl+S" | Executes keyboard shortcut |
| "Scroll down" | Scrolls the active window |

### Creating Guides

1. Click the **Add Guide** button (book icon)
2. Describe the workflow in natural language
3. AI will generate a structured guide
4. Guide is automatically saved and indexed

**Example Input:**
```
How to upload a video to YouTube:
1. Go to YouTube Studio
2. Click Create button in top right
3. Select Upload videos
4. Drag and drop or click to select file
5. Fill in title and description
6. Set visibility and publish
```

---

## Development

### Project Structure

```
automate/
├── src/                    # React Frontend
│   ├── components/         # UI Components
│   ├── stores/             # Zustand State Management
│   ├── i18n/               # Internationalization
│   └── overlay.tsx         # Overlay Window Entry
├── src-tauri/              # Rust Backend
│   ├── src/
│   │   ├── commands/       # Tauri Commands
│   │   ├── config/         # Configuration
│   │   ├── screen/         # Screen Capture & UI Automation
│   │   ├── input/          # Mouse & Keyboard Control
│   │   ├── llm/            # LLM Client & Agent
│   │   └── guides/         # Guide System
│   └── Cargo.toml
├── docs/                   # Documentation
└── guides/                 # User Guides Storage
```

### Tech Stack

| Layer | Technology |
|-------|------------|
| Framework | Tauri 2.x |
| Frontend | React 19, TypeScript 5 |
| State | Zustand 5 |
| Styling | TailwindCSS |
| Animation | Motion (Framer Motion) |
| i18n | i18next |
| Backend | Rust |
| Windows API | windows-rs 0.62 |
| UI Automation | uiautomation-rs 0.24 |

### Available Scripts

```bash
# Development
npm run dev           # Start Vite dev server
npm run tauri dev     # Start Tauri in dev mode

# Build
npm run build         # Build frontend
npm run tauri build   # Build production app

# Other
npm run preview       # Preview production build
```

### Environment Variables

Create `.env` file for development:

```env
VITE_DEFAULT_API_ENDPOINT=https://api.openai.com/v1
```

### Creating a Release

Releases are automated via GitHub Actions. To create a new release:

1. Update version in `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json`
2. Commit the version change
3. Create and push a git tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

4. GitHub Actions will automatically build and create a draft release
5. Review and publish the release on GitHub

---

## API Compatibility

AutoMate works with any OpenAI-compatible API:

| Provider | Endpoint | Model |
|----------|----------|-------|
| OpenAI | `https://api.openai.com/v1` | `gpt-4o`, `gpt-4-turbo` |
| Azure OpenAI | `https://{name}.openai.azure.com` | `gpt-4o` |
| Anthropic (via proxy) | Compatible proxy | `claude-3-opus` |
| Local (Ollama) | `http://localhost:11434/v1` | `llava`, `bakllava` |

**Note**: Vision support requires a multimodal model.

---

## Roadmap

- [ ] Multi-monitor support
- [ ] Task recording and playback
- [ ] Plugin system for custom tools
- [ ] Voice command support
- [ ] macOS and Linux support

---

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- [Tauri](https://tauri.app) - Desktop app framework
- [windows-rs](https://github.com/microsoft/windows-rs) - Windows API bindings
- [uiautomation-rs](https://github.com/leexgone/uiautomation-rs) - UI Automation library

---

<div align="center">

**Made with AI by [HanSyngha](https://github.com/HanSyngha)**

If you find this project useful, please consider giving it a star!

</div>
