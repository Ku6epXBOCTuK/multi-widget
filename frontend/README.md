# SvelteKit Static Template

Шаблон для разработки статических сайтов на SvelteKit с адаптером static.

## Возможности

- **Svelte 5** с Runes
- **TypeScript** со строгой типизацией
- **Адаптер static** — для деплоя на любой хостинг
- **Prettier** — форматирование
- **ESLint** — линтинг (без console.log, без magic numbers)
- **Stylelint** — линтинг стилей
- **Vitest** — юнит-тесты (jsdom + browser)
- **Playwright** — E2E тесты
- **CSS Variables Linter** — проверка CSS-переменных

## Быстрый старт

```sh
npx degit Ku6epXBOCTuK/sveltekit-template my-site
cd my-site
npm install
```

## Команды

| Команда             | Описание                                                        |
| ------------------- | --------------------------------------------------------------- |
| `npm run dev`       | Dev-сервер                                                      |
| `npm run build`     | Сборка в `build/`                                               |
| `npm run preview`   | Предпросмотр                                                    |
| `npm run check`     | TypeScript проверка                                             |
| `npm run check:all` | Full check: format + stylelint + eslint + svelte-check + vitest |
| `npm run lint`      | Prettier + ESLint                                               |
| `npm run test`      | Unit + E2E тесты                                                |
| `npm run test:unit` | Vitest                                                          |
| `npm run test:e2e`  | Playwright                                                      |

## CSS Variables Linter

Уникальный скрипт `scripts/css-vars.js` проверяет CSS-переменные:

- Ошибка, если переменная используется но не определена
- Предупреждение, если переменная определена но не используется

```sh
npm run lint:css-vars
```

## Требования

Node.js >= 22.0.0
