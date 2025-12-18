# AutoMate - Windows AI Automation Assistant

## 1. 프로젝트 개요

### 1.1 프로젝트명
**AutoMate** - AI 기반 Windows 자동화 어시스턴트

### 1.2 목표
- 사용자의 화면을 인식하고 LLM을 통해 상황을 분석
- 마우스/키보드를 제어하여 사용자 대신 작업 수행
- AI가 작업 중임을 시각적으로 표시 (커서 효과, 오버레이 등)
- **핵심 가치**: 속도, 부드러움, 명확한 상황 판단

### 1.3 기술 스택
- **Frontend**: React 19.2 + TypeScript + Vite 6
- **Backend**: Tauri 2.x (Rust)
- **상태관리**: Zustand 5.0
- **애니메이션**: Motion 12.23 (구 framer-motion)
- **스타일링**: TailwindCSS
- **국제화**: i18next (ko/en)
- **Windows API**: windows-rs 0.62, uiautomation 0.24

---

## 2. 기능 요구사항

### 2.1 설정 시스템

#### OpenAI-Compatible Endpoint
- 사용자가 직접 API endpoint 등록
- 필수 설정:
  - API Base URL (예: `https://api.openai.com/v1`)
  - API Key
  - Model Name (예: `gpt-4o`, `claude-3-opus`)
- 선택 설정:
  - Max Tokens
  - Temperature
  - System Prompt 커스터마이징

#### 저장 위치
- `%APPDATA%/AutoMate/config.json`

### 2.2 화면 인식 (하이브리드)

#### 방식 1: UI Automation API
- Windows UI Automation을 통한 UI 요소 탐지
- 장점: 정확한 요소 식별, 빠른 속도
- 사용: `uiautomation-rs` crate

#### 방식 2: Screenshot + Vision API
- 화면 캡처 후 LLM Vision API로 분석
- 장점: UI Automation이 지원하지 않는 요소 인식 가능
- 사용: Windows GDI 캡처 → Base64 → Vision API

#### 하이브리드 전략
1. 먼저 UI Automation으로 요소 탐지 시도
2. 실패 시 Screenshot + Vision으로 폴백
3. 둘 다 사용하여 상호 보완

### 2.3 입력 제어

#### 마우스 제어
- **부드러운 이동**: Bezier 곡선 기반 자연스러운 커서 이동
- **클릭 효과**: 클릭 위치에 시각적 피드백
- 기능:
  - `move_to(x, y, duration)`: 부드러운 이동
  - `click(x, y)`: 좌클릭
  - `right_click(x, y)`: 우클릭
  - `double_click(x, y)`: 더블클릭
  - `drag(from, to)`: 드래그

#### 키보드 제어
- **타이핑**: 자연스러운 속도로 텍스트 입력
- **단축키**: Ctrl+C, Ctrl+V 등 조합키 지원
- 기능:
  - `type_text(text, delay)`: 텍스트 입력
  - `press_key(key)`: 단일 키 입력
  - `hotkey(keys[])`: 조합키 입력

### 2.4 LLM 에이전트

#### 시스템 프롬프트
```
당신은 Windows 자동화 어시스턴트입니다.
사용자의 화면을 분석하고 요청된 작업을 수행합니다.

사용 가능한 도구:
- screen_capture: 현재 화면 캡처
- ui_elements: UI 요소 목록 조회
- mouse_move: 마우스 이동
- mouse_click: 마우스 클릭
- keyboard_type: 텍스트 입력
- keyboard_hotkey: 단축키 실행
- guide_search: 가이드 검색

작업 수행 시:
1. 먼저 화면 상태를 파악합니다
2. 필요한 UI 요소를 찾습니다
3. 단계별로 작업을 수행합니다
4. 각 단계 후 결과를 확인합니다
```

#### 도구 정의 (Function Calling)
```json
{
  "tools": [
    {
      "name": "screen_capture",
      "description": "현재 화면을 캡처합니다",
      "parameters": {}
    },
    {
      "name": "ui_elements",
      "description": "화면의 UI 요소 목록을 조회합니다",
      "parameters": {
        "filter": "string (optional) - 요소 필터"
      }
    },
    {
      "name": "mouse_move",
      "description": "마우스를 지정 위치로 이동합니다",
      "parameters": {
        "x": "number",
        "y": "number",
        "smooth": "boolean (default: true)"
      }
    },
    {
      "name": "mouse_click",
      "description": "마우스 클릭을 수행합니다",
      "parameters": {
        "x": "number",
        "y": "number",
        "button": "left|right|middle",
        "clicks": "number (default: 1)"
      }
    },
    {
      "name": "keyboard_type",
      "description": "텍스트를 입력합니다",
      "parameters": {
        "text": "string"
      }
    },
    {
      "name": "keyboard_hotkey",
      "description": "단축키를 실행합니다",
      "parameters": {
        "keys": "string[] - 예: ['ctrl', 'c']"
      }
    },
    {
      "name": "guide_search",
      "description": "저장된 가이드에서 관련 정보를 검색합니다",
      "parameters": {
        "query": "string"
      }
    }
  ]
}
```

### 2.5 시각 효과

#### AI 활동 표시
- **커서 글로우**: AI 제어 중 커서 주변 발광 효과
- **클릭 리플**: 클릭 시 물결 효과
- **이동 트레일**: 마우스 이동 경로 표시 (선택적)

#### 상태 오버레이
- 화면 하단/상단에 현재 AI 상태 표시
- "분석 중...", "클릭 수행 중...", "입력 중..." 등

#### 구현 방식
- **Windows Layered Window**: 투명 오버레이 창
- 또는 **Tauri 투명 윈도우**: `transparent: true`, `decorations: false`

### 2.6 가이드 시스템

#### 개요
사용자가 특정 웹페이지나 작업에 대한 가이드를 추가하여 AI가 더 정확하게 작업을 수행할 수 있도록 함.

#### 사용자 인터페이스
1. **Add Guide 버튼**: UI에서 가이드 추가
2. **명령어**: `/guide add` 채팅 명령어

#### 가이드 생성 플로우
```
[사용자 입력 (자연어)]
    ↓
[LLM이 구조화된 마크다운으로 변환]
    ↓
[폴더/파일명 자동 제안]
    ↓
[guides/ 폴더에 저장]
```

#### 가이드 저장 구조
```
guides/
├── websites/
│   ├── google-search.md
│   ├── youtube-upload.md
│   └── github-pr.md
├── applications/
│   ├── vscode-shortcuts.md
│   └── excel-macros.md
└── workflows/
    ├── daily-standup.md
    └── code-review.md
```

#### 가이드 마크다운 형식
```markdown
---
title: YouTube 동영상 업로드
domain: youtube.com
tags: [upload, video, youtube]
created: 2024-12-18
---

# YouTube 동영상 업로드 가이드

## 전제 조건
- YouTube Studio에 로그인된 상태

## 단계별 가이드

### 1. 업로드 버튼 찾기
- 우측 상단 "만들기" 버튼 클릭
- "동영상 업로드" 선택

### 2. 파일 선택
- 드래그 앤 드롭 또는 "파일 선택" 클릭
- ...

## 주의사항
- 업로드 중 브라우저 닫지 말 것
```

#### LLM 프롬프트에 가이드 주입

매 LLM 호출 시 다음 정보 포함:

```
[시스템 프롬프트]

## 사용 가능한 가이드 목록
현재 저장된 가이드 인덱스 (depth-1):
- websites/google-search.md: Google 검색 팁
- websites/youtube-upload.md: YouTube 동영상 업로드
- applications/vscode-shortcuts.md: VSCode 단축키
...

필요 시 guide_search 도구로 가이드 내용을 검색하세요.
```

#### guide_search Agentic Tool

Main LLM이 호출하면 Sub-LLM이 자율적으로 가이드를 탐색하는 도구:

```json
{
  "name": "guide_search",
  "description": "저장된 가이드에서 관련 정보를 검색합니다. Sub-agent가 자율적으로 탐색하여 결과를 반환합니다.",
  "parameters": {
    "query": "string - 검색할 내용 (예: 'YouTube 업로드 방법', 'VSCode 단축키')"
  }
}
```

**Agentic Tool 아키텍처:**

```
[Main LLM] --guide_search(query)--> [Sub-LLM Agent]
                                          |
                                          v
                                   [자율적 탐색 수행]
                                          |
                                          v
                                   [결과 또는 "없음" 반환]
```

**Sub-LLM에게 제공되는 도구:**

```json
{
  "tools": [
    {
      "name": "guide_ls",
      "description": "가이드 폴더/파일 목록 조회",
      "parameters": {
        "path": "string (optional) - 조회할 경로 (기본: guides/)"
      }
    },
    {
      "name": "guide_preview",
      "description": "가이드 파일의 처음 10줄 미리보기",
      "parameters": {
        "file_path": "string - 가이드 파일 경로"
      }
    },
    {
      "name": "guide_read",
      "description": "가이드 파일 전체 읽기",
      "parameters": {
        "file_path": "string - 가이드 파일 경로"
      }
    }
  ]
}
```

**Sub-LLM 시스템 프롬프트:**

```
당신은 가이드 검색 에이전트입니다.
사용자의 쿼리에 맞는 가이드를 찾아 반환하세요.

탐색 전략:
1. guide_ls로 폴더명을 확인하여 관련 카테고리가 있는지 판단
2. 관련 폴더가 있으면, 해당 폴더 내 파일명을 확인
3. 관련 파일이 있으면, guide_preview로 내용이 맞는지 확인
4. 맞으면 guide_read로 전체 내용을 읽어서 반환
5. 관련 가이드가 없으면 "없음"을 반환

응답 형식:
- 가이드를 찾은 경우: 가이드 전체 내용
- 가이드가 없는 경우: "없음"
```

**탐색 플로우 예시:**

```
Query: "YouTube 동영상 업로드 방법"

1. guide_ls() → ["websites/", "applications/", "workflows/"]
   → "websites" 폴더가 관련있어 보임

2. guide_ls("websites/") → ["google-search.md", "youtube-upload.md", "github-pr.md"]
   → "youtube-upload.md"가 관련있어 보임

3. guide_preview("websites/youtube-upload.md")
   → "# YouTube 동영상 업로드 가이드..."
   → 맞는 내용으로 확인됨

4. guide_read("websites/youtube-upload.md")
   → 전체 가이드 내용 반환
```

---

## 3. UI/UX 요구사항

### 3.1 메인 화면 레이아웃
```
┌─────────────────────────────────────┐
│  AutoMate            [설정] [최소화] │
├─────────────────────────────────────┤
│                                     │
│  ┌─────────────────────────────┐   │
│  │                             │   │
│  │      채팅/명령 영역          │   │
│  │                             │   │
│  │                             │   │
│  └─────────────────────────────┘   │
│                                     │
│  ┌─────────────────────────────┐   │
│  │  [입력창]           [전송]   │   │
│  └─────────────────────────────┘   │
│                                     │
│  [Add Guide] [상태: 대기 중]        │
└─────────────────────────────────────┘
```

### 3.2 설정 화면
- API 설정 (Endpoint, Key, Model)
- 시각 효과 on/off
- 마우스 이동 속도
- 언어 선택 (한국어/English)

### 3.3 다국어 지원
- 기본: 한국어
- 지원: English
- i18next 사용

---

## 4. 기술 상세

### 4.1 프로젝트 구조
```
automate/
├── docs/
│   ├── REQUIREMENTS.md
│   └── VERSION_GUIDE.md
├── src/                      # React Frontend
│   ├── components/
│   ├── hooks/
│   ├── stores/
│   ├── i18n/
│   ├── App.tsx
│   └── main.tsx
├── src-tauri/               # Rust Backend
│   ├── src/
│   │   ├── commands/        # Tauri 명령어
│   │   ├── config/          # 설정 관리
│   │   ├── screen/          # 화면 캡처/UI Automation
│   │   ├── input/           # 마우스/키보드 제어
│   │   ├── llm/             # LLM 클라이언트
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── capabilities/
│   ├── Cargo.toml
│   └── tauri.conf.json
├── guides/                  # 사용자 가이드 저장
├── package.json
└── vite.config.ts
```

### 4.2 Rust Crate 의존성
```toml
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
```

### 4.3 npm 의존성
```json
{
  "dependencies": {
    "@tauri-apps/api": "^2.9",
    "react": "^19.2",
    "react-dom": "^19.2",
    "zustand": "^5.0",
    "motion": "^12.23",
    "i18next": "^24",
    "react-i18next": "^15",
    "lucide-react": "^0.460"
  }
}
```

---

## 5. 개발 단계

### Phase 1: 프로젝트 설정 ✅
- [x] Tauri 2.x 프로젝트 초기화
- [x] React + TypeScript 설정
- [x] 기본 UI 레이아웃
- [x] i18n 기본 설정 (ko/en)

### Phase 2: 설정 시스템 ✅ (Backend) / ⏳ (Frontend)
- [x] Config 저장/로드 (`config/storage.rs`)
- [x] AppConfig, ApiConfig 구조체 정의
- [x] test_api_connection 커맨드
- [ ] **설정 UI 컴포넌트** (프론트엔드)
- [ ] 유효성 검사 UI 피드백

### Phase 3: 화면 인식 ✅
- [x] Screenshot 캡처 (`screen/capture.rs`) - GDI 기반 Base64 PNG
- [x] UI Automation (`screen/ui_automation.rs`) - 요소 트리 탐색
- [x] 하이브리드 전략 (agent.rs에서 조합)

### Phase 4: 입력 제어 ✅
- [x] 마우스 제어 (`input/mouse.rs`)
  - [x] Bezier 곡선 부드러운 이동
  - [x] 좌/우/중 클릭
  - [x] 더블 클릭
- [x] 키보드 제어 (`input/keyboard.rs`)
  - [x] 텍스트 타이핑 (유니코드)
  - [x] 조합키 입력 (Ctrl+C 등)

### Phase 5: LLM 에이전트 ✅
- [x] OpenAI-compatible 클라이언트 (`llm/client.rs`)
- [x] Function Calling 구현
- [x] 도구 정의 (mouse, keyboard, screen, guide_search)
- [x] 도구 실행 로직 (`chatStore.ts` - executeAction)
- [x] guide_search agentic tool 연동
- [x] 채팅 UI 연동 (`Chat.tsx`)

### Phase 6: 가이드 시스템 ✅
- [x] guides 모듈 생성 (`guides/mod.rs`, `storage.rs`, `search.rs`)
- [x] guide_ls, guide_preview, guide_read 함수
- [x] guide_search Sub-LLM 에이전트 (자율적 탐색)
- [x] 가이드 추가 기능 (자연어 → 마크다운 변환)
- [x] 가이드 추가 UI (`AddGuide` 컴포넌트)
- [x] 시스템 프롬프트에 depth-1 인덱스 주입 (`build_system_prompt`)

### Phase 7: 시각 효과 ✅
- [x] 투명 오버레이 윈도우 (`overlay.html`, `tauri.conf.json`)
- [x] 커서 글로우 효과 (`CursorGlow.tsx`)
- [x] 클릭 리플 효과 (`ClickRipple.tsx`)
- [x] 상태 표시 오버레이 (`StatusIndicator.tsx`)
- [x] 오버레이 Rust 커맨드 (`commands/overlay.rs`)

### Phase 8: 마무리 ✅
- [x] 다국어 지원 기본 설정 (ko/en)
- [x] 에러 처리 개선 (`ErrorBoundary`, `ToastContainer`)
- [x] 성능 최적화 (`useShallow` 적용)
- [x] 패키징 및 배포 준비
