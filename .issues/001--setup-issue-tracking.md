---
title: Настроить локальный трекер задач в монорепозитории
status: closed
date: 2026-06-22T01:31:38Z
categories:
  - repo
description: ""
fmContentType: Issues
---

## Описание задачи

Перенести ведение задач из веб-интерфейса GitHub прямо в локальный репозиторий.
Задачи должны храниться как отдельные `.md` файлы в папке `.issues/`,
редактироваться через визуальную панель в VS Code, валидироваться через
локальный `cargo xtask` и проверяться при каждом пуше в CI.

### Чек-лист для выполнения

#### 🛠 1. Инфраструктура и VS Code (Фронтенд-часть)

- [x] Установить расширение **Front Matter CMS** в VS Code для удобной работы в
      «одно окно».
- [x] Создать конфиг `.frontmatter/config/frontmatter.json` и описать тип
      контента `Issue` (поля: заголовок, статус, теги).
- [x] Установить `markdownlint-cli2` в корневой `package.json` для проверки
      синтаксиса текста.

#### 🦀 2. Локальный тулинг (Бэкенд-часть)

- [x] Создать служебный крейт `xtask` в корне Cargo-воркспейса.
- [x] Добавить алиас `xtask = "run --package xtask --"` в файл
      `.cargo/config.toml`.
- [x] Написать в `xtask` команду `lint-issues`:
  - Парсинг и строгая валидация YAML-полей через `serde_yaml` (проверка enum
    статусов и тегов).
  - Вызов `npm run lint:issues:md` через `std::process::Command` для проверки
    тела текста.

#### 🚀 3. Интеграция в CI

- [x] Добавить в `.github/workflows/ci.yml` шаг запуска локального линтера:

  ```yaml
  - name: Lint Issues via xtask
    run: cargo xtask lint-issues
  ```

- [x] Проверить, что при ошибке в оформлении задачи или опечатке в статусе
      сборка в GitHub Actions падает.

### Полезные материалы и скриншоты

### Структура монорепозитория

Целевое расположение всех компонентов системы:

```text
my-monorepo/
├── .cargo/
│   └── config.toml            # Алиас для cargo xtask
├── .frontmatter/
│   └── config/
│       └── frontmatter.json   # Шаблон для VS Code
├── .issues/                   # Папка для ваших задач
│   ├── 001-setup-issue-tracking.md
│   └── attachments/           # Скриншоты и медиа
├── server/                    # Сервер на Rust
├── frontend/                  # Приложение на SvelteKit
├── xtask/                     # Автоматизация линтинга
│   └── src/main.rs
├── Cargo.toml                 # Корневой воркспейс
└── package.json               # Корневой конфиг с markdownlint
```

---

_Заметка: Этот файл является эталонным образцом. Как только
`cargo xtask lint-issues` будет готов, он должен успешно проходить валидацию на
этом файле._
