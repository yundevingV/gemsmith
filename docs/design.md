# 💎 Gemsmith Design System (`design.md`)
> **Theme:** Royal Gemsmith (중세 연금술 & 고급 보석 세공 공방)  
> **Concept:** 스플랜더 특유의 정통 클래식 감성에 연금술사의 공방 및 정교한 황동/골드 보석 세공 요소를 결합한 고급스러운 보드게임 UI 디자인 시스템

---

## 1. Visual Mood & Core Aesthetics

*   **배경 (Atmosphere):** 깊이감 있는 딥 네이비(`Deep Navy`) 및 다크 에메랄드(`Dark Emerald`) 바탕에 은은한 융단/세공 작업대 가죽 텍스처 오버레이.
*   **프레임 & 억센트 (Frame & Metal):** 황동(`Brass`), 앤틱 골드(`Antique Gold`), 르네상스 감성의 기하학적 세공 테두리.
*   **자원 & 보석 (Gems & Tokens):** 빛을 받는 면(Facet)이 명확히 대비되는 잘 깎여진 정교한 원석/보석 이펙트.

---

## 2. Design Tokens (디자인 토큰)

### 2.1. Color Palette

#### Base & Atmosphere Colors (배경 및 UI 패널)
| Token Name | Hex Code | Usage |
| :--- | :--- | :--- |
| `--bg-base-dark` | `#0B131F` | 메인 앱/게임판 배경 (딥 네이비) |
| `--bg-panel-emerald` | `#0D221E` | 카드/플레이어 판 패널 배경 (다크 에메랄드) |
| `--bg-surface-velvet` | `#161B26` | 팝업 및 모달 벨벳 오버레이 |
| `--border-brass` | `#B8860B` | 기본 프레임 및 테두리 (황동) |
| `--border-gold-accent` | `#F5D061` | 선택/하이라이트 테두리 (빛나는 골드) |

#### Gem Colors (자원 및 혜택 보석)
| Gem Type | Primary Token | Gradient Pair (Light / Dark) | Glow Color |
| :--- | :--- | :--- | :--- |
| **Ruby (Red)** | `#E63946` | `#FF6B6B` / `#990012` | `rgba(230, 57, 70, 0.5)` |
| **Sapphire (Blue)**| `#1D3557` | `#457B9D` / `#0A192F` | `rgba(69, 123, 157, 0.5)` |
| **Emerald (Green)**| `#2A9D8F` | `#52B788` / `#134035` | `rgba(42, 157, 143, 0.5)` |
| **Diamond (White)**| `#E0E1DD` | `#FFFFFF` / `#9A8C98` | `rgba(255, 255, 255, 0.6)`|
| **Onyx (Black)** | `#2B2D42` | `#4A4E69` / `#12131C` | `rgba(74, 78, 105, 0.4)` |
| **Gold / Wildcard**| `#E9C46A` | `#FFE6A7` / `#B58A29` | `rgba(233, 196, 106, 0.7)`|

---

### 2.2. Typography

*   **Display / Header Font:** `Cinzel Decorative`, `Cinzel`, `MedievalSharp` (판타지/클래식 고풍 타이틀)
*   **UI & Numerics Font:** `Cinzel` (카드 점수, 귀족 점수), `Inter` or `Pretendard` (숫자 가독성 보장)

```css
/* Typography Scale */
--font-title-lg: normal 700 32px/1.2 'Cinzel Decorative', serif;
--font-title-md: normal 700 24px/1.3 'Cinzel', serif;
--font-score-card: normal 800 28px/1.0 'Cinzel', serif;
--font-body-bold: normal 600 16px/1.4 'Pretendard', sans-serif;
--font-cost-num: normal 700 14px/1.0 'Pretendard', sans-serif;
```

---

### 2.3. Elevation & Shadows

*   `--shadow-token`: `0px 4px 10px rgba(0, 0, 0, 0.6), inset 0px 1px 2px rgba(255, 255, 255, 0.3)`
*   `--shadow-card-hover`: `0 12px 24px rgba(0, 0, 0, 0.7), 0 0 16px var(--border-gold-accent)`
*   `--shadow-affordable-glow`: `0 0 12px #F5D061, inset 0 0 8px #F5D061`

---

## 3. Core Components Specification

### 3.1. Gem Token Component (`<GemChip />`)
*   **형태:** 정팔각형(Octagon) 또는 입체적 깎은 면이 강조된 둥근 칩.
*   **상태별 시각화:**
    *   **Normal:** 선명한 보석 그라데이션 + 골드 테두리.
    *   **Selected:** Y축 8px 위로 이동, 외부 Gold Glow 효과, 개수 태그 하이라이트.
    *   **Disabled (수량 0):** Grayscale 80% + 투명도 50%.

### 3.2. Development Card Component (`<DevelopmentCard />`)
*   **구조:**
    1.  **Header Zone:** 
        *   왼쪽: 명망 점수 (Prestige Points - Gold Font, 크고 명확하게)
        *   오른쪽: 영구 보석 혜택 아이콘 (빛나는 원석 뱃지)
    2.  **Artwork Zone:** 보석 광산, 운반선, 세공 공방, 왕실 거래처 일러스트 레이어.
    3.  **Cost Footer Zone:** 카드 구매에 필요한 보석 비용 (보석 아이콘 + 숫자 칩 배치).
*   **상태별 시각화:**
    *   **Affordable (구매 가능):** 카드 전체에 옅은 골드 빛 펄스(Pulse) 애니메이션.
    *   **Unaffordable (구매 불가):** 코스트 영역 중 부족한 보석 아이콘 회색조 처리.
    *   **Reserved (예약됨):** 카드 상단에 '황동 씰(Brass Seal)' 깃발 마크 표시.

### 3.3. Noble Tile Component (`<NobleTile />`)
*   **형태:** 완벽한 정사각형 타일 + 모서리 황동 장식 오버레이.
*   **구성:**
    *   왼쪽/상단: 귀족 점수 (`3`점 공통).
    *   하단: 획득에 필요한 카드 조합 조건 (예: Emerald 카드 4장, Ruby 카드 4장).

### 3.4. Player Status Board (`<PlayerBoard />`)
*   **컨셉:** 세공사의 보석함 패널.
*   **구성 요소:**
    *   **Prestige Total:** 플레이어 닉네임 옆 웅장한 로얄 왕관 아이콘과 총 점수.
    *   **Gem Bank (토큰):** 소유 중인 실물 칩 개수 (최대 10개 제한 표시기 포함).
    *   **Development Bonuses (카드 혜택):** 보유 중인 카드로 할인받는 영구 보석 수량.

---

## 4. User Interaction & UX Rules

1.  **자원 수집 액션 (Take Tokens):**
    *   서로 다른 보석 3개 선택 시, 선택한 칩이 튀어나오는 듯한 Pop 애니메이션과 함께 플레이어 보드 방향으로 스파크 효과.
    *   동일 보석 2개 선택은 해당 칩 묶음이 남아있는 수량이 4개 이상일 때만 하이라이트 활성화.
2.  **카드 구매 액션 (Buy Card):**
    *   구매 가능 상태의 카드를 클릭/드래그하면 필요 보석이 소비되며 카드가 플레이어의 혜택 영역으로 슬라이드 이동.
3.  **카드 예약 액션 (Reserve Card):**
    *   카드를 손으로 가져오는 연출과 함께 황동 골드 칩(황동 조커 자원) 1개 자동 획득.

---

## 5. CSS / Tailwind Configuration Example

```javascript
// tailwind.config.js 예시
module.exports = {
  theme: {
    extend: {
      colors: {
        gemsmith: {
          navy: '#0B131F',
          emerald: '#0D221E',
          brass: '#B8860B',
          gold: '#F5D061',
        },
        gem: {
          ruby: '#E63946',
          sapphire: '#1D3557',
          emerald: '#2A9D8F',
          diamond: '#E0E1DD',
          onyx: '#2B2D42',
          wildcard: '#E9C46A',
        }
      },
      fontFamily: {
        cinzel: ['Cinzel', 'serif'],
        display: ['Cinzel Decorative', 'serif'],
      }
    }
  }
}
```