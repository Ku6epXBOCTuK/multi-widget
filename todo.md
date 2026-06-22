# Роадмап multi-widget: итеративные минимальные шаги

## Текущее состояние

**Готово:** SvelteKit SPA (Stage→Task→SubTask), Axum REST (GET/POST/PATCH),
SQLite, SSE эндпоинт на сервере + бродкаст, CORS. **Не сделано:** Фронтенд не
слушает SSE, нет DELETE, CLI, system:config, create_from хардкодит ID, readme
неактуален.

---

## Фаза A: Фронтенд — neverthrow + конфиг

> Цель: обернуть fetch в neverthrow, вынести URL в конфиг. Не зависит от SSE,
> можно делать первой.

- **A1.** Добавить neverthrow, создать `safeFetch` с типизированными ошибками
- **A2.** Обернуть существующий fetch в `safeFetch`
- **A3.** Вынести URL API в переменную окружения, заменить хардкод
- **A4.** Ручной тест: сервер + фронтенд, данные загружаются, ошибки
  обрабатываются

---

## Фаза B: Фронтенд — SSE клиент

> Цель: подключить EventSource, обновлять стор в реальном времени.

- **B1.** Создать SSE utility модуль с `EventSource` и reconnect
- **B2.** Обработка `CreateActivity` → `activityStore.add()`
- **B3.** Обработка `UpdateActivity` → `activityStore.update()`
- **B4.** Обработка `DeleteActivity` → `activityStore.remove()`
- **B5.** Обработать `Lagged` → полный re-fetch
- **B6.** Ручной тест: обновить активность через curl, виджет обновляется без
  перезагрузки

---

## Фаза C: DELETE-эндпоинт

> Цель: замкнуть CRUD.

- **C1.** Добавить DELETE route + handler на сервере
- **C2.** Broadcast `DeleteActivity` через SSE
- **C3.** Ручной тест: `curl -X DELETE` → виджет убирает строку

---

## Фаза D: Исправить create_from

> Цель: убрать хардкод ID.

- **D1.** Использовать DB-generated ID вместо захардкоженного

---

## Фаза E: CLI — ядро

> Цель: отдельный крейт `widget-cli`, text-режим.

- **E1.** Создать крейт widget-cli с clap и reqwest
- **E2.** Корневая команда `widget` с clap
- **E3.** Подкоманда `widget todo` с вариантами List/Add/Done/Delete
- **E4.** HTTP-клиент модуль для запросов к серверу
- **E5.** `widget todo list` — текстовый вывод (id, title, status)
- **E6.** `widget todo add "текст"` — POST на сервер
- **E7.** `widget todo done #N` — PATCH status
- **E8.** `widget todo delete #N` — DELETE
- **E9.** Флаг `--json` для вывода в JSON

---

## Фаза F: CLI — TUI режим

> Цель: интерактивный просмотр списка задач в терминале.

- **F1.** Добавить ratatui/crossterm в зависимости
- **F2.** TUI модуль: инициализация runtime, структура App
- **F3.** Рендер списка с выделением текущей строки
- **F4.** Обработка клавиатуры: навигация, done, add, delete, выход
- **F5.** Интеграция TUI как подкоманда `widget todo tui`
- **F6.** Автообновление через polling или SSE

---

## Фаза G: OpenAPI / utoipa (опционально)

> Цель: автогенерация спецификации API. Параллельно с E-F.

- **G1.** Добавить utoipa в сервер
- **G2.** Аннотировать типы `#[derive(ToSchema)]`
- **G3.** Аннотировать handlers `#[utoipa::path(...)]`
- **G4.** Эндпоинт `/api-docs/openapi.json`
- **G5.** Сгенерировать клиент из OpenAPI (опционально)

---

## Фаза H: system:config (темы + видимость)

> Цель: управлять темой и видимостью виджетов через CLI.

- **H1.** Добавить `SystemConfig` в shared
- **H2.** GET/PATCH эндпоинты конфига на сервере
- **H3.** SSE-событие `system:config`
- **H4.** Фронтенд: применять тему из конфига
- **H5.** CLI: `widget system theme dark`
- **H6.** CLI: `widget todo hide` / `widget todo show` (универсально для
  виджетов)

---

## Фаза I: Readme + документация

- **I1.** Обновить readme.md (WebSocket → SSE, текущая архитектура)
- **I2.** README для widget-cli (установка, примеры команд)

---

## Фаза J: Рефакторинг сервера — чистая архитектура + ТДД

> Цель: разделить сервер на слои, вынести бизнес-логику, покрыть тестами. Каждый
> шаг — один коммит.

- **J1.** Создать модульную структуру: domain / repository / service / routes /
  events
- **J2.** Вынести сущности в domain (model, ошибки)
- **J3.** Unit-тесты ошибок domain
- **J4.** Определить трейт Repository + mock-тест
- **J5.** Реализовать SqliteRepository, перенести логику из текущего db.rs
- **J6.** Реализовать CacheRepository (decorator) + unit-тест
- **J7.** Вынести бизнес-логику в service + unit-тесты
- **J8.** Переписать handlers на service + интеграционный тест
- **J9.** Переписать SSE broadcast через event bus + unit-тест
- **J10.** Переписать AppState на DI, wiring в main
- **J11.** Удалить старый db.rs, проверить `cargo check`
- **J12.** Интеграционные тесты handlers (CRUD)
- **J13.** Тесты edge cases (невалидные данные, несуществующие id)

---

## Порядок выполнения

1. **A** — neverthrow + конфиг
2. **B** — SSE клиент на фронте
3. **C** — DELETE-эндпоинт
4. **D** — Исправить create_from
5. **E** — CLI ядро
6. **F** — CLI TUI
7. **G** — OpenAPI (параллельно с E-F, опционально)
8. **J** — Рефакторинг сервера (после стабильного функционала)
9. **H** — system:config (после рефакторинга)
10. **I** — Документация (в конце или по ходу дела)

---

## Затронутые модули

| Фаза | Модули                                                       |
| ---- | ------------------------------------------------------------ |
| A    | `frontend` — утилиты, стор, конфиг                           |
| B    | `frontend` — SSE-клиент, стор                                |
| C    | `server` — роуты, db                                         |
| D    | `shared` — модель, `server` — handler                        |
| E    | `widget-cli` — крейт целиком (clap, HTTP-клиент, команды)    |
| F    | `widget-cli` — TUI-модуль                                    |
| G    | `server` + `shared` — аннотации utoipa                       |
| H    | `shared` — SystemConfig, `server` — роуты, `frontend` — тема |
| I    | корень — readme, `widget-cli` — readme                       |
| J    | `server` — domain/repository/service/routes/events/state     |

---

## Верификация

После каждой фазы:

1. `cargo check --workspace` — сервер + shared + CLI компилируются
2. `cargo clippy --workspace` — нет предупреждений
3. `npm run check` (во фронтенде) — TypeScript компилируется
4. Ручной тест: запустить сервер + фронтенд, проверить что изменения работают
5. После CLI: `cargo run --bin widget-cli -- todo list` — проверить вывод
