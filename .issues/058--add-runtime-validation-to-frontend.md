---
title: Add runtime validation to frontend
status: open
description: Add runtime validation to frontend (typebox\\zod\\valibot)
date: 2026-06-23T03:56:27Z
categories:
  - frontend
fmContentType: Issues
---

## Добавить валидацию на фронтенд

Провести исследование насчет валидацию на клиенте, а также генерацию схем
валидации из типов

specta-typescript генерирует типы по алфавиту, поэтому прямая конвертация в
схемы может ломаться из-за очередности типов

typebox 1.x вроде может и генерировать схемы, и валидировать, надо подробнее
почитать
[typebox docs](https://sinclairzx81.github.io/typebox/#/docs/script/1_syntax)
