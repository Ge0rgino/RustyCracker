# 🔓 RustyCracker 

Wielowątkowa aplikacja webowa do łamania haseł w archiwach ZIP oraz krakowania skrótów (hash) za pomocą **brute-force**. Projekt realizowany w ramach zajęć z **inżynierii oprogramowania**.

## Spis treści
- [🔓 RustyCracker](#-rustycracker)
  - [Spis treści](#spis-treści)
  - [🏢 Architektura Projektu](#-architektura-projektu)
  - [📁 Struktura Repozytorium](#-struktura-repozytorium)
  - [🗒️ Organizacja Pracy](#️-organizacja-pracy)


## 🏢 Architektura Projektu

Aplikacja została zaprojektowana w architekturze monorepo. Dzieli się na dwa główne moduły:
  - **Backend (`/backend`)**: wysokowydajny silnik obliczeniowy napisany w języku **Rust**.
    - Wykorzystuje framework **Axum** do obsługi żądań HTTP.
    - Oparty na asynchronicznym środowisku **Tokio**.
    - Równoległe przetwarzanie i łamanie haseł realizowane jest za pomocą biblioteki **Rayon**.
  - **Frontend (`/frontend`)**: interfejs użytkownika zbudowany w **React** i **TypeScript**.
    - Za stylizację i responsywność odpowiada **Tailwind CSS**.
    - Raportowanie i wizualizacja danych realizowane przy użyciu **Recharts**.

## 📁 Struktura Repozytorium

```
RustyCracker/
├── backend/                # Kod źródłowy API i silnika w Rust
├── frontend/               # Kod źródłowy aplikacji React/TS
├── docs/                   # Dokumentacja projektowa, procesowa i architektoniczna
│   └── workflow.md         # Standardy pracy, GitFlow i zarządzanie Kanban
├── .gitignore
└── README.md
```

## 🗒️ Organizacja Pracy
- Projekt zarządzany jest przy użyciu tablicy **Github Projects (Kanban)**. Nazwa Repozytorium jest dokładnie taka sama, jak nazwa projektu w Github Projects.
- Stosujemy model z wydzieloną gałęzią integracyjną (rozwój `dev`, stabilny kod na `main`).
- Przepływ pracy, konwencje commitów oraz zasady Code Review są szczegółowo opisane w pilku `docs/workflow.md`.