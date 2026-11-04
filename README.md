# Gemsmith

**스플랜더형** 1 vs CPU (Rust). 상업판 복제 아님 — 프로시저럴 카드·Patron.

## GUI (기본)

```bash
cargo run --release
```

메뉴: **언어**(ko/en/ja/zh) · CPU 난이도 · Play.  
UI: Royal Gemsmith 디자인 (`docs/design.md`), 코드 드로잉( PNG 불필요).

## CLI

```bash
cargo run --release --bin gemsmith-cli
```

## 규칙 요약

- 보석 5색 + gold, 카드 3티어, Patron 3명
- **15점** 도달 시 마지막 라운드 후 최고 점수 승리

## 개발

```bash
cargo test
```

PR·CI·Changelog: [CONTRIBUTING.md](CONTRIBUTING.md)
