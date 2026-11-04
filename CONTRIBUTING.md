# 기여 가이드 (CONTRIBUTING)

Gemsmith에 기여해 주셔서 감사합니다.

## 개발 환경

- Rust stable (`rustup`)
- macOS / Linux / Windows (eframe GUI)

```bash
git clone https://github.com/yundevingV/gemsmith.git
cd gemsmith
cargo test
cargo run --release          # GUI
cargo run --release --bin gemsmith-cli
```

## PR

- **브랜치:** `main`에서만 분기 → **`main`으로 PR** (develop 없음)
- 관례: `feat/…`, `fix/…`, `docs/…`, `chore/…`
- `cargo test` 통과 (PR·push 시 **CI**)
- PR 본문 **## Changelog**에 사용자용 한 줄 (비우면 PR 제목 사용)
- **main** 머지 시 [CHANGELOG.md](CHANGELOG.md) `[Unreleased]`에 Actions가 자동 추가

## 커밋하지 말 것

- `target/` (빌드 산출물)
- 개인 경로·시크릿·API 키

## 코드 위치

| 영역 | 경로 |
|------|------|
| 게임 규칙 | `src/game.rs`, `src/card.rs`, `src/gem.rs` |
| AI | `src/ai.rs` |
| GUI | `src/gui/` |
| i18n | `src/locale.rs` |
| CLI | `src/cli.rs`, `src/bin/cli.rs` |

디자인 토큰: `docs/design.md`, `src/gui/tokens.rs`
