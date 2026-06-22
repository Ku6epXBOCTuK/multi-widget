---
title: Setup cocogitto
status: open
description: Setup cocogitto - independent versioning
date: 2026-06-22T15:12:59Z
categories:
  - ci
fmContentType: Issues
---

## 🛠️ Настройка независимого версионирования в монорепе (Cocogitto)

## 1. Конфигурация в корневом cog.toml

```toml
[monorepo] resolver = "Cargo" # Ищет связи Cargo, Npm, Maven для порядка релизов

# Описание пакета бэкенда

[monorepo.packages.my_backend_crate] path = "crates/my_backend_crate"
bump_profiles.main.pre_bump_hooks = [ "cargo set-version --package {{package}} {{version}}" ]

# Описание пакета фронтенда

[monorepo.packages.my_frontend_app] path = "frontend/my_app"
bump_profiles.main.pre_bump_hooks = [ "npm version {{version}} --no-git-tag-version" ]
```
