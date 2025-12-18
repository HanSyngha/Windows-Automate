# AutoMate 의존성 버전 가이드 (2025년 12월 기준)

이 문서는 프로젝트에서 사용하는 각 패키지의 **최신 버전 변경사항**과 **올바른 사용법**을 정리합니다.

---

## 목차

0. [프로젝트 초기화 가이드](#0-프로젝트-초기화-가이드)
1. [Tauri 2.x](#1-tauri-2x)
2. [React 19.2](#2-react-192)
3. [Zustand 5.0](#3-zustand-50)
4. [Motion 12.23](#4-motion-1223-구-framer-motion)
5. [windows-rs 0.62](#5-windows-rs-062)
6. [uiautomation-rs 0.24](#6-uiautomation-rs-024)

---

## 0. 프로젝트 초기화 가이드

> **참고**: [Tauri Create Project](https://v2.tauri.app/start/create-project/) | [Vite Getting Started](https://vite.dev/guide/)

### 사전 요구사항

```bash
# Node.js 20.19+ 또는 22.12+ 필요
node --version

# Rust 설치 (Windows)
# https://rustup.rs/ 에서 rustup-init.exe 다운로드 후 실행
rustup --version

# Rust 1.71+ 필요 (windows-link raw-dylib 지원)
rustc --version
```

### 방법 1: create-tauri-app 사용 (권장)

```bash
# npm 7+ (추가 -- 필요)
npm create tauri-app@latest automate -- --template react-ts

# 또는 대화형 모드
npm create tauri-app@latest
```

**대화형 프롬프트:**
1. Project name: `automate`
2. Identifier: `com.automate.app`
3. Frontend language: `TypeScript / JavaScript`
4. Package manager: `npm`
5. UI template: `React`
6. UI flavor: `TypeScript`

```bash
cd automate
npm install
npm run tauri dev
```

### 방법 2: 기존 Vite 프로젝트에 Tauri 추가

```bash
# 1. Vite + React 프로젝트 생성
npm create vite@latest automate -- --template react-ts
cd automate
npm install

# 2. React 19로 업그레이드 (Vite는 기본 React 18 생성)
npm install react@19 react-dom@19
npm install -D @types/react@19 @types/react-dom@19

# 3. Tauri CLI 설치
npm install -D @tauri-apps/cli@latest

# 4. Tauri 초기화
npx tauri init
```

**tauri init 프롬프트:**
- App name: `AutoMate`
- Window title: `AutoMate`
- Web assets relative path: `../dist`
- Dev server URL: `http://localhost:5173`
- Frontend dev command: `npm run dev`
- Frontend build command: `npm run build`

### Tauri 2 프로젝트 구조

```
automate/
├── src/                      # React 프론트엔드
│   ├── App.tsx
│   ├── main.tsx
│   └── ...
├── src-tauri/                # Rust 백엔드
│   ├── src/
│   │   ├── main.rs           # 진입점
│   │   └── lib.rs            # 커맨드 정의
│   ├── capabilities/         # 권한 설정 (Tauri 2 신규)
│   │   └── default.json
│   ├── icons/
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
├── vite.config.ts
└── tsconfig.json
```

### Cargo.toml (src-tauri/)

```toml
[package]
name = "automate"
version = "0.1.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2.5", features = [] }

[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
base64 = "0.22"
image = "0.25"

[target.'cfg(windows)'.dependencies]
windows = { version = "0.62", features = [
    "Win32_Foundation",
    "Win32_UI_WindowsAndMessaging",
    "Win32_UI_Input_KeyboardAndMouse",
    "Win32_Graphics_Gdi",
    "Win32_UI_Accessibility",
]}
uiautomation = { version = "0.24", features = ["process", "dialog", "event", "clipboard"] }

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
```

---

## 1. Tauri 2.x

> **npm 버전**: @tauri-apps/cli 2.9.x, @tauri-apps/api 2.9.x (2025년 12월)
> **Rust crate 버전**: tauri 2.x, tauri-build 2.5.x (npm과 crate 버전이 다름!)
>
> **참고**: [Tauri Releases](https://github.com/tauri-apps/tauri/releases) | [Tauri 2.9.0 Release Notes](https://v2.tauri.app/release/tauri/v2.9.0/)
>
> **주의**: npm 패키지와 Rust crate의 버전 번호가 다릅니다.
> - npm: `@tauri-apps/cli@2.9.x`, `@tauri-apps/api@2.9.x`
> - Rust: `tauri = "2"`, `tauri-build = "2.5"`

### 주요 변경사항 (2.0 → 2.x)

#### 1.1 권한 시스템 (Allowlist → Capabilities)

**이전 (Tauri 1.x)**:
```json
{
  "tauri": {
    "allowlist": {
      "fs": { "all": true },
      "shell": { "open": true }
    }
  }
}
```

**현재 (Tauri 2.x)**:
```json
// src-tauri/capabilities/default.json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default capabilities",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "shell:allow-open",
    "fs:default"
  ]
}
```

#### 1.2 플러그인 시스템

Tauri 2.x에서는 많은 기능이 플러그인으로 분리됨:

```toml
# Cargo.toml
[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-shell = "2"
tauri-plugin-fs = "2"
tauri-plugin-dialog = "2"
tauri-plugin-os = "2"
```

```typescript
// 프론트엔드에서 사용
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import { readTextFile } from '@tauri-apps/plugin-fs';
```

#### 1.3 IPC Raw Payload (성능 개선)

대용량 데이터 전송 시 JSON 대신 Raw Bytes 사용 가능:

```rust
// Rust 백엔드
#[tauri::command]
fn get_screenshot() -> Vec<u8> {
    // 이미지 바이트 직접 반환 (JSON 직렬화 없음)
    capture_screen()
}
```

---

## 2. React 19.2

> **현재 버전**: 19.2.x (2025년 12월)
>
> **참고**: [React 19 Blog](https://react.dev/blog)

### 주요 신규 기능

#### 2.1 `<Activity />` 컴포넌트

UI를 언마운트하지 않고 숨기면서 상태 유지:

```tsx
import { Activity } from 'react';

function App() {
  const [showSidebar, setShowSidebar] = useState(true);

  return (
    <div>
      {/* React 19.2: 상태 유지하면서 숨김 */}
      <Activity mode={showSidebar ? 'visible' : 'hidden'}>
        <Sidebar />
      </Activity>
    </div>
  );
}
```

#### 2.2 `useEffectEvent` 훅

Effect 내에서 이벤트 로직 분리:

```tsx
import { useEffect, useEffectEvent } from 'react';

function ChatRoom({ roomId, theme }) {
  // 이벤트 핸들러 - theme 변경 시 effect 재실행 안 함
  const onConnected = useEffectEvent((connectedRoomId) => {
    showNotification(`Connected to ${connectedRoomId}`, theme);
  });

  useEffect(() => {
    const connection = createConnection(roomId);
    connection.on('connected', () => {
      onConnected(roomId);
    });
    connection.connect();
    return () => connection.disconnect();
  }, [roomId]); // theme은 의존성에서 제외됨!
}
```

---

## 3. Zustand 5.0

> **현재 버전**: 5.0.x (2025년 12월)
>
> **참고**: [마이그레이션 가이드](https://zustand.docs.pmnd.rs/migrations/migrating-to-v5)

### Breaking Changes

#### 3.1 Default Export 제거

```tsx
// v4 (이전)
import create from 'zustand';

// v5 (현재)
import { create } from 'zustand';
```

#### 3.2 `useShallow` 훅 (중요!)

무한 루프 방지를 위해 필수:

```tsx
import { useShallow } from 'zustand/shallow';

function Component() {
  // 위험: 매 렌더링마다 새 배열 생성 → 무한 루프
  // const items = useStore((state) => state.items.filter(i => i.active));

  // 안전: useShallow로 안정적인 참조
  const items = useStore(
    useShallow((state) => state.items.filter(i => i.active))
  );
}
```

### 권장 패턴 (v5)

```tsx
// stores/agentStore.ts
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { useShallow } from 'zustand/shallow';

interface AgentState {
  isRunning: boolean;
  messages: Message[];
  setRunning: (running: boolean) => void;
  addMessage: (msg: Message) => void;
}

export const useAgentStore = create<AgentState>()(
  persist(
    (set) => ({
      isRunning: false,
      messages: [],
      setRunning: (running) => set({ isRunning: running }),
      addMessage: (msg) => set((s) => ({ messages: [...s.messages, msg] })),
    }),
    { name: 'agent-store' }
  )
);

// 컴포넌트에서 사용
function ChatPanel() {
  // 배열/객체 선택 시 useShallow 필수
  const messages = useAgentStore(useShallow((s) => s.messages));
  const isRunning = useAgentStore((s) => s.isRunning); // 프리미티브는 OK
}
```

---

## 4. Motion 12.23 (구 framer-motion)

> **현재 버전**: 12.x (2025년 12월)
>
> **참고**: [마이그레이션 가이드](https://motion.dev/docs/react-upgrade-guide)

### 패키지 리브랜딩

```bash
# 이전
npm uninstall framer-motion

# 현재
npm install motion
```

### Import 변경

```tsx
// 이전 (framer-motion)
import { motion, AnimatePresence } from 'framer-motion';

// 현재 (motion)
import { motion, AnimatePresence } from 'motion/react';
```

### 사용 예시

```tsx
import { motion, AnimatePresence } from 'motion/react';

function ClickEffect({ x, y }: { x: number; y: number }) {
  return (
    <motion.div
      initial={{ scale: 0, opacity: 1 }}
      animate={{ scale: 2, opacity: 0 }}
      transition={{ duration: 0.5 }}
      style={{
        position: 'fixed',
        left: x,
        top: y,
        width: 20,
        height: 20,
        borderRadius: '50%',
        backgroundColor: 'rgba(100, 149, 237, 0.5)',
      }}
    />
  );
}
```

---

## 5. windows-rs 0.62

> **현재 버전**: 0.62.x (2025년)
>
> **참고**: [GitHub Releases](https://github.com/microsoft/windows-rs/releases)

### 핵심 변경: windows-targets → windows-link

- **raw-dylib 기본 사용**: 링커 복잡성 제거
- **빌드 시간 단축**: import lib 다운로드 불필요
- **MSRV**: Rust 1.71+ 필요

### 의존성 설정

```toml
# Cargo.toml
[target.'cfg(windows)'.dependencies]
windows = { version = "0.62", features = [
    "Win32_Foundation",
    "Win32_UI_WindowsAndMessaging",
    "Win32_UI_Input_KeyboardAndMouse",
    "Win32_Graphics_Gdi",
    "Win32_UI_Accessibility",
]}
```

### 사용 예시

```rust
use windows::{
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::UI::Input::KeyboardAndMouse::*,
};

fn get_cursor_pos() -> Result<(i32, i32), windows::core::Error> {
    let mut point = POINT::default();
    unsafe {
        GetCursorPos(&mut point)?;
    }
    Ok((point.x, point.y))
}
```

---

## 6. uiautomation-rs 0.24

> **현재 버전**: 0.24.x (2025년)
>
> **참고**: [GitHub](https://github.com/leexgone/uiautomation-rs)

### 호환성 주의

v0.19.0 이전 버전과 호환하려면 features 추가 필요:

```toml
[dependencies]
uiautomation = { version = "0.24", features = ["process", "dialog", "event", "clipboard"] }
```

### 기본 사용법

```rust
use uiautomation::{UIAutomation, UIElement, Result};

fn get_active_window_info() -> Result<String> {
    let automation = UIAutomation::new()?;
    let focused = automation.get_focused_element()?;

    Ok(format!(
        "Name: {}, ClassName: {}",
        focused.get_name()?,
        focused.get_classname()?
    ))
}
```

---

## 요약: 핵심 변경사항 체크리스트

| 패키지 | 버전 | 핵심 변경 |
|--------|------|----------|
| **Tauri** | npm 2.9 / crate 2.x | Allowlist → Capabilities, 플러그인 분리 |
| **React** | 19.2 | `<Activity>`, `useEffectEvent` 추가 |
| **Zustand** | 5.0 | default export 제거, `useShallow` 필수 |
| **Motion** | 12.x | `framer-motion` → `motion` 리브랜딩 |
| **windows-rs** | 0.62 | `windows-targets` → `windows-link` (raw-dylib) |
| **uiautomation** | 0.24 | features 명시 필요 |

---

## 참고 링크

- [Tauri 2.0 Stable Release](https://v2.tauri.app/blog/tauri-20/)
- [Zustand v5 Migration](https://zustand.docs.pmnd.rs/migrations/migrating-to-v5)
- [Motion Upgrade Guide](https://motion.dev/docs/react-upgrade-guide)
- [windows-rs GitHub](https://github.com/microsoft/windows-rs)
- [uiautomation-rs GitHub](https://github.com/leexgone/uiautomation-rs)
