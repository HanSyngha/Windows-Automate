# AutoMate 코드베이스 구조

## 개요

AutoMate는 AI 기반 Windows 자동화 어시스턴트입니다.

**기술 스택:**
- **프론트엔드**: React + TypeScript + Vite + TailwindCSS
- **백엔드**: Rust + Tauri 2.x
- **AI**: OpenAI 호환 API

---

## 전체 디렉토리 구조

```
automate/
├── .github/                # GitHub Actions 워크플로우
├── docs/                   # 문서
├── public/                 # 정적 파일 (아이콘 등)
├── src/                    # 프론트엔드 소스 (React/TypeScript)
├── src-tauri/              # 백엔드 소스 (Rust/Tauri)
├── index.html              # 메인 HTML
├── overlay.html            # 오버레이 HTML
├── package.json            # Node.js 의존성
├── tailwind.config.js      # TailwindCSS 설정
├── tsconfig.json           # TypeScript 설정
└── vite.config.ts          # Vite 빌드 설정
```

---

## 루트 파일

| 파일 | 설명 |
|------|------|
| `index.html` | 메인 앱의 HTML 엔트리포인트 |
| `overlay.html` | 오버레이 창의 HTML 엔트리포인트 |
| `package.json` | npm 패키지 의존성 및 스크립트 정의 |
| `package-lock.json` | npm 의존성 잠금 파일 |
| `postcss.config.js` | PostCSS 설정 (TailwindCSS용) |
| `tailwind.config.js` | TailwindCSS 테마 및 설정 |
| `tsconfig.json` | TypeScript 컴파일러 설정 |
| `tsconfig.node.json` | Node.js용 TypeScript 설정 |
| `vite.config.ts` | Vite 빌드 도구 설정 |
| `vite.config.d.ts` | Vite 설정 타입 정의 |
| `README.md` | 프로젝트 소개 및 사용법 |

---

## GitHub 워크플로우 (`.github/workflows/`)

| 파일 | 설명 |
|------|------|
| `ci.yml` | PR 시 빌드 테스트 실행 |
| `release.yml` | 태그 푸시 시 Windows 빌드 및 GitHub Release 생성 |

---

## 문서 (`docs/`)

| 파일 | 설명 |
|------|------|
| `01_CODEBASE.md` | 코드베이스 구조 설명 (현재 문서) |
| `02_REQUIREMENTS.md` | 프로젝트 요구사항 정의 |
| `03_VERSION_GUIDE.md` | 버전 관리 가이드 |
| `04_RELEASE_GUIDE.md` | 릴리즈 프로세스 가이드 |

---

## 프론트엔드 (`src/`)

### 엔트리포인트

| 파일 | 설명 |
|------|------|
| `main.tsx` | 메인 앱 React 엔트리포인트, 루트 렌더링 |
| `overlay.tsx` | 오버레이 창 React 엔트리포인트 |
| `App.tsx` | 메인 앱 루트 컴포넌트, 라우팅 및 레이아웃 |
| `index.css` | 전역 CSS 스타일 (TailwindCSS 포함) |
| `vite-env.d.ts` | Vite 환경 타입 정의 |

### 컴포넌트 (`src/components/`)

| 파일 | 설명 |
|------|------|
| `Chat.tsx` | AI 채팅 인터페이스, 메시지 입력/표시, 가이드 선택 |
| `Settings.tsx` | 설정 패널, API 설정/언어 선택/테마 설정 |
| `AddGuide.tsx` | 가이드 생성 다이얼로그 |
| `Toast.tsx` | 토스트 알림 컴포넌트 |
| `UpdateChecker.tsx` | 자동 업데이트 확인 및 알림 UI |
| `ErrorBoundary.tsx` | React 에러 경계, 에러 발생 시 폴백 UI |

### 오버레이 컴포넌트 (`src/components/overlay/`)

| 파일 | 설명 |
|------|------|
| `OverlayApp.tsx` | 오버레이 루트 컴포넌트 |
| `CursorGlow.tsx` | 커서 주변 글로우 효과 |
| `ClickRipple.tsx` | 클릭 시 물결 효과 애니메이션 |
| `StatusIndicator.tsx` | 현재 작업 상태 표시 (실행중/완료 등) |

### 상태 관리 (`src/stores/`)

| 파일 | 설명 |
|------|------|
| `chatStore.ts` | 채팅 메시지 상태 관리, localStorage 영속화 |
| `configStore.ts` | 앱 설정 상태 (API 키, 엔드포인트, 언어 등) |
| `guideStore.ts` | 가이드 목록 및 선택 상태 |
| `overlayStore.ts` | 오버레이 표시/숨김 상태 |
| `toastStore.ts` | 토스트 알림 큐 관리 |

### 다국어 지원 (`src/i18n/`)

| 파일 | 설명 |
|------|------|
| `index.ts` | i18next 초기화 및 언어 설정 |
| `ko.json` | 한국어 번역 파일 |
| `en.json` | 영어 번역 파일 |

---

## 백엔드 (`src-tauri/`)

### 루트 파일

| 파일 | 설명 |
|------|------|
| `Cargo.toml` | Rust 패키지 의존성 및 메타데이터 |
| `build.rs` | Tauri 빌드 스크립트 |
| `tauri.conf.json` | Tauri 앱 설정 (창 크기, 번들, 업데이터 등) |

### Capabilities (`src-tauri/capabilities/`)

| 파일 | 설명 |
|------|------|
| `default.json` | 기본 권한 설정 (파일 접근, 쉘 실행 등) |

### 소스 코드 (`src-tauri/src/`)

#### 코어 파일

| 파일 | 설명 |
|------|------|
| `main.rs` | Rust 바이너리 엔트리포인트 |
| `lib.rs` | Tauri 앱 빌더, 플러그인 초기화, 트레이 아이콘 핸들러 |

#### Commands 모듈 (`src-tauri/src/commands/`)

프론트엔드에서 `invoke()`로 호출하는 Tauri 커맨드들.

| 파일 | 설명 |
|------|------|
| `mod.rs` | 커맨드 모듈 내보내기, `greet` 테스트 커맨드 |
| `screen.rs` | `capture_screen`: 화면 캡처, `get_ui_tree`: UI 요소 트리 가져오기 |
| `input.rs` | `mouse_move`: 마우스 이동, `mouse_click`: 마우스 클릭, `keyboard_type`: 텍스트 입력, `keyboard_press`: 키 누르기 |
| `config.rs` | `get_config`: 설정 로드, `save_config`: 설정 저장, `test_api_connection`: API 연결 테스트 |
| `llm.rs` | `send_message`: AI에게 메시지 전송 및 응답 받기 |
| `guides.rs` | `guide_list`: 가이드 목록, `guide_preview`: 미리보기, `guide_read`: 읽기, `guide_index`: 인덱싱, `guide_search`: 검색, `guide_create`: 생성 |
| `overlay.rs` | `overlay_show`: 오버레이 표시, `overlay_hide`: 숨김, `overlay_cursor_move`: 커서 이동, `overlay_click`: 클릭, `overlay_status`: 상태, `overlay_set_control`: 제어 설정 |

#### Screen 모듈 (`src-tauri/src/screen/`)

화면 캡처 및 UI 분석 기능.

| 파일 | 설명 |
|------|------|
| `mod.rs` | 모듈 내보내기 |
| `capture.rs` | Windows GDI를 이용한 화면 캡처, PNG 인코딩 |
| `ui_automation.rs` | Windows UI Automation API로 UI 요소 탐지 |

#### Input 모듈 (`src-tauri/src/input/`)

마우스/키보드 입력 자동화.

| 파일 | 설명 |
|------|------|
| `mod.rs` | 모듈 내보내기 |
| `mouse.rs` | 마우스 커서 이동 및 클릭 (좌클릭/우클릭/더블클릭) |
| `keyboard.rs` | 키보드 입력 시뮬레이션 (텍스트 입력, 특수키) |

#### LLM 모듈 (`src-tauri/src/llm/`)

AI API 통신 및 에이전트 로직.

| 파일 | 설명 |
|------|------|
| `mod.rs` | 모듈 내보내기 |
| `client.rs` | OpenAI 호환 API 클라이언트, HTTP 요청 처리 |
| `agent.rs` | AI 에이전트 로직, 액션 파싱 및 실행 |

#### Guides 모듈 (`src-tauri/src/guides/`)

가이드 문서 저장 및 검색.

| 파일 | 설명 |
|------|------|
| `mod.rs` | 모듈 내보내기 |
| `storage.rs` | 마크다운 가이드 파일 CRUD |
| `search.rs` | 가이드 내용 검색 기능 |

#### Config 모듈 (`src-tauri/src/config/`)

앱 설정 관리.

| 파일 | 설명 |
|------|------|
| `mod.rs` | 설정 타입 정의 (ApiConfig, AppConfig 등) |
| `storage.rs` | JSON 파일로 설정 영속화 |

---

## 데이터 흐름

```
┌─────────────────────────────────────────────────────────────┐
│                    프론트엔드 (React)                        │
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
│                    백엔드 (Rust/Tauri)                        │
│              ┌──────────────┴────────────┐                   │
│              │     Tauri Commands        │                   │
│              └──────────────┬────────────┘                   │
│       ┌─────────────────────┼─────────────────────┐          │
│       │           │         │         │           │          │
│  ┌────┴────┐ ┌────┴────┐ ┌──┴──┐ ┌────┴────┐ ┌────┴────┐    │
│  │  화면   │ │  입력   │ │ LLM │ │ 가이드  │ │  설정   │    │
│  │  캡처   │ │  제어   │ │에이전트│ │ 저장소  │ │ 저장소  │    │
│  └────┬────┘ └────┬────┘ └──┬──┘ └─────────┘ └─────────┘    │
│       │           │         │                                │
└───────┼───────────┼─────────┼────────────────────────────────┘
        │           │         │
   ┌────┴────┐ ┌────┴────┐ ┌──┴──────────┐
   │ Windows │ │ Windows │ │ OpenAI API  │
   │   GDI   │ │  Input  │ │   (호환)    │
   └─────────┘ └─────────┘ └─────────────┘
```

---

## 주요 기능

### 1. AI 채팅 인터페이스
- **위치**: `src/components/Chat.tsx` + `src-tauri/src/llm/`
- OpenAI 호환 API 사용
- 자연어 명령을 액션으로 변환 및 실행

### 2. 화면 캡처 및 분석
- **위치**: `src-tauri/src/screen/`
- Windows GDI로 스크린샷 캡처
- UI Automation으로 화면 요소 탐지

### 3. 입력 자동화
- **위치**: `src-tauri/src/input/`
- 마우스 이동 및 클릭
- 키보드 텍스트 입력 및 특수키

### 4. 가이드 시스템
- **위치**: `src/components/AddGuide.tsx` + `src-tauri/src/guides/`
- 마크다운 기반 작업 가이드
- 검색 및 선택 기능

### 5. 시각적 오버레이
- **위치**: `src/components/overlay/`
- 커서 글로우 효과
- 클릭 물결 애니메이션
- 상태 표시기

### 6. 자동 업데이트
- **위치**: `src/components/UpdateChecker.tsx`
- GitHub Releases 연동
- 서명된 업데이트 (latest.json)

---

## 빌드 및 개발

### 개발 모드
```bash
npm run tauri dev
```

### 프로덕션 빌드
```bash
npm run tauri build
```

### 릴리즈 (GitHub Actions)
1. `tauri.conf.json`과 `Cargo.toml`에서 버전 업데이트
2. 커밋 및 푸시
3. 태그 생성: `git tag v0.x.x && git push origin v0.x.x`
4. GitHub Actions가 빌드 후 Release 생성

---

## 관련 문서

- [02_REQUIREMENTS.md](./02_REQUIREMENTS.md) - 프로젝트 요구사항
- [03_VERSION_GUIDE.md](./03_VERSION_GUIDE.md) - 버전 관리 가이드
- [04_RELEASE_GUIDE.md](./04_RELEASE_GUIDE.md) - 릴리즈 가이드
