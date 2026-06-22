import { readFileSync, writeFileSync, existsSync, readdirSync } from "node:fs";
import { join } from "node:path";

const ISSUES_DIR = join(import.meta.dirname, "..", ".issues");
const TODO_FILE = join(import.meta.dirname, "..", "todo.md");

const todo = readFileSync(TODO_FILE, "utf-8");

const steps = [];
const stepRegex = /\*\*([A-Z]\d+)\.\*\*\s+(.+)/g;
let match;
while ((match = stepRegex.exec(todo)) !== null) {
  steps.push({ id: match[1], title: match[2].trim() });
}

const existing = existsSync(ISSUES_DIR)
  ? readdirSync(ISSUES_DIR)
      .filter((f) => f.endsWith(".md"))
      .map((f) => parseInt(f.split("--")[0], 10))
      .filter((n) => !isNaN(n))
  : [];

const maxExisting = existing.length > 0 ? Math.max(...existing) : 0;
const pad = (n) => String(n).padStart(3, "0");

let seq = maxExisting + 1;
for (const step of steps) {
  const num = pad(seq);
  const filename = `${num}--${slugify(step.title)}.md`;
  const filepath = join(ISSUES_DIR, filename);

  if (existsSync(filepath)) {
    seq++;
    continue;
  }

  const content = `---
title: "${step.title.replace(/"/g, '\\"')}"
status: open
date: ${new Date().toISOString()}
categories:
  - ${categoryForId(step.id)}
description: ""
fmContentType: Issues
---

## Описание задачи

${step.title}
`;

  writeFileSync(filepath, content, "utf-8");
  console.log(`created: ${filename}`);
  seq++;
}

function slugify(text) {
  return text
    .toLowerCase()
    .replace(/[а-яё]/g, (c) => translit(c))
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "")
    .slice(0, 80);
}

function translit(c) {
  const map = {
    а: "a", б: "b", в: "v", г: "g", д: "d", е: "e", ё: "yo",
    ж: "zh", з: "z", и: "i", й: "y", к: "k", л: "l", м: "m",
    н: "n", о: "o", п: "p", р: "r", с: "s", т: "t", у: "u",
    ф: "f", х: "kh", ц: "ts", ч: "ch", ш: "sh", щ: "shch",
    ъ: "", ы: "y", ь: "", э: "e", ю: "yu", я: "ya",
  };
  return map[c] || c;
}

function categoryForId(id) {
  const letter = id[0];
  const map = {
    A: "frontend",
    B: "frontend",
    C: "server",
    D: "server",
    E: "cli",
    F: "cli",
    G: "server",
    H: "server",
    I: "docs",
    J: "server",
  };
  return map[letter] || "general";
}
