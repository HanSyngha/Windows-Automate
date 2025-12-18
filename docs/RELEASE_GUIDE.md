# Release & Auto-Update Guide

이 문서는 AutoMate의 릴리스 프로세스와 자동 업데이트 시스템에 대해 설명합니다.

## Table of Contents

- [빌드 환경 설정](#빌드-환경-설정)
- [릴리스 프로세스](#릴리스-프로세스)
- [자동 업데이트 시스템](#자동-업데이트-시스템)
- [GitHub Actions 워크플로우](#github-actions-워크플로우)
- [트러블슈팅](#트러블슈팅)

---

## 빌드 환경 설정

### 필수 요구사항

| 요구사항 | 버전 | 용도 |
|---------|------|------|
| Windows 10/11 | 64-bit | 빌드 환경 (WSL 불가) |
| Node.js | 20.x+ | Frontend 빌드 |
| Rust | 1.71+ | Backend 빌드 |
| Visual Studio Build Tools | 2019+ | C++ 컴파일러 |

### 로컬 빌드 테스트

```powershell
# Windows PowerShell에서 실행

# 1. 의존성 설치
npm install

# 2. 개발 모드 테스트
npm run tauri dev

# 3. 프로덕션 빌드
npm run tauri build
```

빌드 결과물 위치:
```
src-tauri/target/release/bundle/
├── nsis/
│   └── AutoMate_x.x.x_x64-setup.exe    # NSIS 인스톨러
└── msi/
    └── AutoMate_x.x.x_x64_en-US.msi    # MSI 인스톨러
```

---

## 릴리스 프로세스

### Step 1: 버전 업데이트

두 파일의 버전을 동일하게 업데이트:

**src-tauri/Cargo.toml:**
```toml
[package]
name = "automate"
version = "0.2.0"  # 버전 업데이트
```

**src-tauri/tauri.conf.json:**
```json
{
  "version": "0.2.0"  // 버전 업데이트
}
```

### Step 2: 변경사항 커밋

```bash
git add -A
git commit -m "chore: bump version to 0.2.0"
```

### Step 3: Git Tag 생성 및 Push

```bash
# Tag 생성 (v prefix 필수)
git tag v0.2.0

# Main 브랜치와 Tag 함께 Push
git push origin main --tags
```

### Step 4: GitHub Actions 자동 빌드

Tag push 시 자동으로:
1. `.github/workflows/release.yml` 실행
2. Windows runner에서 빌드
3. NSIS/MSI 인스톨러 생성
4. `latest.json` 생성
5. GitHub Releases에 Draft로 업로드

### Step 5: Release 발행

1. GitHub Repository → Releases 이동
2. Draft release 확인
3. Release notes 수정 (필요시)
4. **Publish release** 클릭

---

## 자동 업데이트 시스템

### 아키텍처

```
┌─────────────────────────────────────────────────────────────────┐
│                        GitHub Releases                          │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │  latest.json    │  │  setup.exe      │  │  setup.nsis.zip │ │
│  │  (버전 정보)     │  │  (인스톨러)      │  │  (서명된 번들)   │ │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘ │
└───────────┼─────────────────────┼─────────────────────┼─────────┘
            │                     │                     │
            ▼                     │                     │
┌───────────────────────┐         │                     │
│   UpdateChecker.tsx   │         │                     │
│  ┌─────────────────┐  │         │                     │
│  │ check() 호출     │◄─┼─────────┘                     │
│  │ 버전 비교        │  │                               │
│  │ 알림 표시        │  │                               │
│  └────────┬────────┘  │                               │
│           │           │                               │
│  ┌────────▼────────┐  │                               │
│  │ downloadAndInstall│◄┼───────────────────────────────┘
│  │ 진행률 표시      │  │
│  │ 앱 재시작        │  │
│  └─────────────────┘  │
└───────────────────────┘
```

### 업데이트 플로우

1. **앱 시작 시 자동 확인**
   ```typescript
   // src/components/UpdateChecker.tsx
   useEffect(() => {
     checkForUpdates();
   }, []);
   ```

2. **버전 비교**
   - Endpoint: `https://github.com/HanSyngha/Windows-Automate/releases/latest/download/latest.json`
   - 현재 버전 vs latest.json의 버전 비교

3. **사용자 알림**
   - 새 버전 발견 시 우상단에 알림 표시
   - "Install Now" / "Later" 선택 가능

4. **다운로드 및 설치**
   - 진행률 바 표시
   - 백그라운드 다운로드
   - 설치 완료 후 자동 재시작

### latest.json 형식

```json
{
  "version": "0.2.0",
  "notes": "See https://github.com/HanSyngha/Windows-Automate/releases/tag/v0.2.0",
  "pub_date": "2024-01-15T10:30:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "...",
      "url": "https://github.com/.../AutoMate_0.2.0_x64-setup.nsis.zip"
    }
  }
}
```

---

## GitHub Actions 워크플로우

### release.yml

**트리거 조건:**
```yaml
on:
  push:
    tags:
      - 'v*'  # v로 시작하는 모든 태그
```

**주요 단계:**
1. `actions/checkout` - 코드 체크아웃
2. `actions/setup-node` - Node.js 설정
3. `dtolnay/rust-action` - Rust 설정
4. `npm ci` - 의존성 설치
5. `tauri-apps/tauri-action` - 빌드 및 릴리스

**환경 변수 (GitHub Secrets):**

| Secret | 설명 | 필수 |
|--------|------|------|
| `GITHUB_TOKEN` | 자동 제공됨 | O |
| `TAURI_SIGNING_PRIVATE_KEY` | 업데이트 서명 키 | X (선택) |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | 서명 키 비밀번호 | X (선택) |

### ci.yml

PR 및 main 브랜치 push 시 실행:
- Frontend lint & type check
- Rust clippy & fmt check
- 전체 빌드 테스트

---

## 서명 키 설정 (선택)

보안을 위해 업데이트 서명 키 설정 권장:

### 1. 키 생성

```bash
# Tauri CLI로 키 쌍 생성
npm run tauri signer generate -- -w ~/.tauri/automate.key
```

출력:
- Private key: `~/.tauri/automate.key`
- Public key: 콘솔에 출력됨

### 2. GitHub Secrets 설정

Repository → Settings → Secrets and variables → Actions:
- `TAURI_SIGNING_PRIVATE_KEY`: Private key 내용
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`: 키 생성 시 입력한 비밀번호

### 3. tauri.conf.json 업데이트

```json
{
  "plugins": {
    "updater": {
      "pubkey": "여기에_public_key_붙여넣기"
    }
  }
}
```

---

## 트러블슈팅

### 빌드 실패: openssl-sys

**증상:**
```
error: failed to run custom build command for `openssl-sys`
```

**원인:** WSL/Linux에서 Windows 빌드 시도

**해결:** Windows 환경에서 빌드하거나 GitHub Actions 사용

---

### 업데이트 확인 실패

**증상:** 앱 시작 시 업데이트 확인 안됨

**확인사항:**
1. 인터넷 연결 확인
2. `latest.json` URL 접근 가능 여부
3. 브라우저 개발자 도구에서 네트워크 에러 확인

---

### NSIS 인스톨러 누락

**증상:** 빌드 후 NSIS 폴더 없음

**원인:** NSIS 미설치

**해결:**
```powershell
# Windows에서 Chocolatey로 설치
choco install nsis
```

---

### SmartScreen 경고

**증상:** 인스톨러 실행 시 "Windows protected your PC" 경고

**원인:** 코드 서명 없음

**임시 해결:** "More info" → "Run anyway" 클릭

**영구 해결:** EV 코드 서명 인증서 구매 및 적용

---

## 버전 관리 규칙

[Semantic Versioning](https://semver.org/) 준수:

| 버전 | 변경 유형 | 예시 |
|------|----------|------|
| MAJOR (x.0.0) | 하위 호환성 없는 변경 | API 변경, 설정 포맷 변경 |
| MINOR (0.x.0) | 하위 호환 기능 추가 | 새 기능, 새 언어 지원 |
| PATCH (0.0.x) | 버그 수정 | 버그 픽스, 성능 개선 |

**예시:**
- `0.1.0` → `0.2.0`: 새 기능 추가
- `0.2.0` → `0.2.1`: 버그 수정
- `0.2.1` → `1.0.0`: 정식 릴리스
