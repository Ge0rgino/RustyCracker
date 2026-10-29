# 🔄 Workflow i Standardy Pracy – RustyCracker

Ten dokument definiuje cykl życia zadania w naszym zespole: od momentu zaplanowania pracy na tablicy Kanban, przez pisanie kodu i logowanie czasu, aż po włączenie zmian do głównej gałęzi (Merge/Pull Request).

---

## 📍 1. Planowanie i Inicjalizacja Zadania (GitHub Projects + Clockify)

1. **Utworzenie Issue:** Utwórz nowe zgłoszenie (Issue) w repozytorium. Nadaj mu zwięzły tytuł i krótki opis.
2. **Estymacja:** W GitHub Projects przypisz do zadania estymatę (zakładany czas w godzinach, np. `1.5h`).
3. **Przypisanie:** Przypisz zadanie do siebie (Assignees).
4. **Rozpoczęcie pracy:** 
   - Przeciągnij kartę zadania z kolumny **To Do** do **In Progress**.
   - Uruchom stoper w **Clockify** (lub dodaj wpis ręcznie po zakończeniu), wpisując dokładną nazwę zadania z GitHuba. Jeżeli zadanie nazywa się np. `#9 Opracowanie standardów i dokumentacji przepływu pracy (Workflow)`, to taka nazwa ma widnieć w **Clockify**.

---

## 🪾 2. Strategia Rozgałęzień (Branching)

Używamy uproszczonego modelu GitFlow.

- Główną gałęzią deweloperską, od której odbijamy i do której mergujemy naszą codzienną pracę, jest **`dev`**.
- Gałąź **`main`** służy wyłącznie do utrzymywania stabilnych, działających wersji.

Przed rozpoczęciem pracy zawsze upewnij się, że masz aktualny stan gałęzi `dev`:
```bash
git switch dev
git pull origin dev
```

**Nazewnictwo gałęzi (Feature Branches):**
Tworząc nową gałąź, stosuj konwencję: `typ/numer_issue-krotki-opis`
```bash
# Przykłady:
git switch -c feat/12-zip-bruteforce
git switch -c fix/15-ui-overflow
git switch -c docs/8-repo-setup
```

---

## 📝 3. Konwencja Semantic Commits

Historia Git musi być czytelna i jasno wskazywać, jakiego modułu monorepo dotyczy zmiana. Używamy standardu **Conventional Commits**.

**Struktura:**
`typ(zakres): krótki opis zmiany (#numer_issue)`

### Dostępne typy:
- `feat`: Nowa funkcjonalność (np. nowy endpoint, nowy komponent React).
- `fix`: Naprawa błędu.
- `docs`: Zmiany w dokumentacji (README, workflow, komentarze).
- `refactor`: Zmiana struktury kodu, która nie dodaje funkcjonalności ani nie naprawia błędu (np. wydzielenie funkcji, optymalizacja).
- `chore`: Prace utrzymaniowe, aktualizacje zależności (Cargo.toml, package.json), konfiguracja.

### Dostępne zakresy (Scopes) w monorepo:
- `(backend)`: Zmiany w silniku Rust (Katalog `/backend`).
- `(frontend)`: Zmiany w interfejsie (Katalog `/frontend`).
- `(repo)`: Zmiany globalne, np. `.gitignore`, dokumentacja główna.

**Przykłady poprawnych commitów:**
> - `feat(backend): implement parallel zip extraction using rayon (#12)`
> - `fix(frontend): resolve progress bar overflow on mobile screens (#15)`
> - `chore(repo): update GitHub issue templates (#4)`

---

## 🤝 4. Pull Request (PR), Code Review i obsługa Clockify

Gdy skończysz pisać kod, czas zintegrować zmiany z gałęzią deweloperską.

1. **Weryfikacja, synchronizacja i wysłanie kodu:**
   - Upewnij się, że Twój branch roboczy uwzględnia aktualne zmiany z `dev`. Pobierz aktualny stan serwera i zsynchronizuj gałęzie:
     ```bash
     git fetch origin
     git switch dev
     git pull --ff-only origin dev
     git switch nazwa-twojej-galezi
     git merge dev
     ```
   - Następnie rozwiąż konflikty, jeśli wystąpią, i uruchom testy ponownie. Dzięki temu synchronizujesz branch roboczy ze świeżym stanem `origin/dev`.
   - Przed utworzeniem PR sprawdź, czy wprowadzone zmiany działają poprawnie (uruchom testy lub wykonaj weryfikację manualną).
   - Wypchnij swoją gałąź na serwer:
     ```bash
     git push -u origin nazwa-twojej-galezi
     ```
2. **Utworzenie PR i status In Review:**
   - Utwórz Pull Request z Twojej gałęzi roboczej **do gałęzi `dev`** (**NIGDY** bezpośrednio do `main`!).
   - W opisie PR wpisz powiązanie z zadaniem, np. `Closes #12`.
   - Przenieś kartę zadania na tablicy Kanban z **In Progress** do **In Review**.
3. **Zasady Clockify podczas Review:**
   - **Pauza:** W momencie wystawienia PR zatrzymaj stoper w Clockify. Czas oczekiwania na sprawdzenie kodu nie jest czasem pracy.
   - **Poprawki:** Jeśli recenzent zgłosi uwagi, **wznów stoper w Clockify** na czas nanoszenia poprawek. Po wysłaniu poprawek ponownie go zatrzymaj.
4. **Code Review:** Poproś przynajmniej jednego członka zespołu o sprawdzenie kodu.
5. **Merge:** Po akceptacji (Approve), wykonaj Merge opcją **Squash and merge**, aby w historii gałęzi `dev` pojawił się jeden czysty commit.

---

## 🏁 5. Zamykanie Zadania i Ewaluacja

Po udanym włączeniu (Merge) kodu do gałęzi `dev`:
1. Usuń swoją lokalną i zdalną gałąź roboczą, aby utrzymać porządek w repozytorium.
2. **Stan Issue i Projects:** Upewnij się, że samo **Issue zostało zamknięte (Closed)**, a jego karta w GitHub Projects znajduje się w kolumnie **Done**. Ponieważ scalanie następuje do gałęzi `dev`, sprawdź, czy Issue zamknęło się automatycznie; jeśli nie, zamknij je ręcznie.
3. Upewnij się, że stoper w **Clockify** jest zatrzymany, a łączny zmierzony czas odzwierciedla faktyczny czas implementacji i poprawek.
4. Na etapie pisania sprawozdania z inżynierii oprogramowania, zestaw czas planowany (Estymata) z czasem rzeczywistym (Clockify) w arkuszu ewaluacyjnym i zanotuj wniosek z ewentualnej wariancji.