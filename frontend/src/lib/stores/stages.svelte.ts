import type { Stage } from "$lib/generated-bindings";

let stages = $state<Stage[]>([]);

export function getStages() {
	return {
		get all() {
			return stages;
		},
		init(data: Stage[]) {
			stages = data;
		},
		addStage(stage: Stage) {
			stages = [...stages, stage];
		},
		updateStage(updated: Stage) {
			stages = stages.map((s) => (s.id === updated.id ? updated : s));
		},
		removeStage(id: number) {
			stages = stages.filter((s) => s.id !== id);
		},
	};
}

export const stageStore = getStages();

stageStore.init([
	{
		id: 1,
		title: "Этап 1: Дизайн и протокол",
		status: "done",
		tasks: [
			{
				id: 101,
				parent: 1,
				title: "Продумать визуальную структуру виджета",
				description: "",
				status: "done",
				subtasks: [
					{
						id: 1001,
						parent: 101,
						title: "Определить габариты контейнера под оверлей OBS.",
						status: "done",
						description: "",
					},
					{
						id: 1002,
						parent: 101,
						title:
							"Выбрать цветовую схему для работы на прозрачном/полупрозрачном фоне.",
						status: "done",
						description: "",
					},
				],
			},
			{
				id: 102,
				parent: 1,
				title: "Спроектировать схемы данных",
				description: "",
				status: "done",
				subtasks: [
					{
						id: 1003,
						parent: 102,
						title: "Описать JSON-структуру сущности ToDo (ID, текст, статус).",
						status: "done",
						description: "",
					},
					{
						id: 1004,
						parent: 102,
						title:
							"Спроектировать форматы WebSocket-событий (добавление, обновление, удаление).",
						status: "done",
						description: "",
					},
				],
			},
		],
	},
	{
		id: 2,
		title: "Этап 2: Подготовка фронтенда",
		status: "done",
		tasks: [
			{
				id: 201,
				parent: 2,
				title: "Настроить окружение фронтенд-проекта",
				description: "",
				status: "done",
				subtasks: [
					{
						id: 2001,
						parent: 201,
						title: "Развернуть чистый проект через менеджер пакетов.",
						status: "done",
						description: "",
					},
					{
						id: 2002,
						parent: 201,
						title:
							"Интегрировать выбранный CSS-инструмент (фреймворк или препроцессор).",
						status: "done",
						description: "",
					},
				],
			},
			{
				id: 202,
				parent: 2,
				title: "Перевести сборщик в режим SPA (Single Page App)",
				description: "",
				status: "done",
				subtasks: [
					{
						id: 2003,
						parent: 202,
						title:
							"Подключить адаптер для генерации строго статических файлов.",
						status: "done",
						description: "",
					},
					{
						id: 2004,
						parent: 202,
						title:
							"Отключить SSR (серверный рендеринг) в глобальной конфигурации.",
						status: "done",
						description: "",
					},
					{
						id: 2005,
						parent: 202,
						title: "Настроить резервный роутинг на index.html (fallback ???).",
						status: "done",
						description: "",
					},
					{
						id: 2006,
						parent: 202,
						title: "Настроить сборку в один html файл.",
						status: "done",
						description: "",
					},
				],
			},
		],
	},
	{
		id: 3,
		title: "Этап 3: Статический прототип",
		status: "done",
		tasks: [
			{
				id: 301,
				parent: 3,
				title: "Создать компоненты интерфейса",
				description: "",
				status: "done",
				subtasks: [
					{
						id: 3001,
						parent: 301,
						title:
							"Написать изолированный компонент для строки подзадачи (текст, галочка).",
						status: "done",
						description: "",
					},
					{
						id: 3002,
						parent: 301,
						title:
							"Написать изолированный компонент для строки задачи (текст, галочка, подзадачи).",
						status: "done",
						description: "",
					},
					{
						id: 3003,
						parent: 301,
						title:
							"Написать изолированный компонент для строки этапа (текст, задачи).",
						status: "done",
						description: "",
					},
					{
						id: 3004,
						parent: 301,
						title: "Написать главный компонент-контейнер для вывода списка.",
						status: "done",
						description: "",
					},
					{
						id: 3005,
						parent: 301,
						title:
							"(Отменен - сделаю после другой темы) Добавить базовую анимацию для плавного появления/исчезновения строк.",
						status: "done",
						description: "",
					},
				],
			},
			{
				id: 302,
				parent: 3,
				title: "Настроить управление локальным состоянием",
				description: "",
				status: "done",
				subtasks: [
					{
						id: 3006,
						parent: 302,
						title:
							"Создать глобальное реактивное хранилище (state rune) внутри фронтенда.",
						status: "done",
						description: "",
					},
					{
						id: 3007,
						parent: 302,
						title:
							"Наполнить состояние статичным массивом задач для немедленной проверки верстки.",
						status: "done",
						description: "",
					},
				],
			},
			{
				id: 303,
				parent: 3,
				title: "Проверить отображение в стрим-софте",
				description: "",
				status: "done",
				subtasks: [
					{
						id: 3008,
						parent: 303,
						title: "Скомпилировать проект в финальную статическую папку.",
						status: "done",
						description: "",
					},
					{
						id: 3009,
						parent: 303,
						title:
							"Закинуть сгенерированный index.html в OBS как браузерный источник.",
						status: "done",
						description: "",
					},
				],
			},
		],
	},
	{
		id: 4,
		title: "Этап 4: Разработка бэкенда",
		status: "pending",
		tasks: [
			{
				id: 401,
				parent: 4,
				title: "Развернуть архитектуру бэкенда",
				description: "",
				status: "pending",
				subtasks: [
					{
						id: 4001,
						parent: 401,
						title:
							"Инициализировать Cargo воркспейс для изоляции сервера и CLI приложения.",
						status: "pending",
						description: "",
					},
					{
						id: 4002,
						parent: 401,
						title:
							"Создать проект сервера, подключить сетевой фреймворк и асинхронный рантайм.",
						status: "pending",
						description: "",
					},
				],
			},
			{
				id: 402,
				parent: 4,
				title: "Реализовать постоянное хранение данных",
				description: "",
				status: "pending",
				subtasks: [
					{
						id: 4003,
						parent: 402,
						title:
							"Написать модуль для чтения/записи состояния в локальный файл.",
						status: "pending",
						description: "",
					},
					{
						id: 4004,
						parent: 402,
						title:
							"Обеспечить потокобезопасный доступ к данным в памяти сервера (Mutex/RwLock).",
						status: "pending",
						description: "",
					},
				],
			},
			{
				id: 403,
				parent: 4,
				title: "Подготовить REST API слой",
				description: "",
				status: "pending",
				subtasks: [
					{
						id: 4005,
						parent: 403,
						title:
							"Написать обработчик (Handler) для отдачи полного списка задач (GET).",
						status: "pending",
						description: "",
					},
					{
						id: 4006,
						parent: 403,
						title:
							"Написать обработчики для добавления и изменения статуса задач (POST/PATCH).",
						status: "pending",
						description: "",
					},
					{
						id: 4007,
						parent: 403,
						title:
							"Включить и настроить CORS-политику для входящих локальных запросов.",
						status: "pending",
						description: "",
					},
				],
			},
		],
	},
	{
		id: 5,
		title: "Этап 5: Интеграция WebSocket",
		status: "pending",
		tasks: [
			{
				id: 501,
				parent: 5,
				title: "Организовать WebSocket-сервер",
				description: "",
				status: "pending",
				subtasks: [
					{
						id: 5001,
						parent: 501,
						title:
							"Добавить эндпоинт для апгрейда HTTP-соединения до WebSocket.",
						status: "pending",
						description: "",
					},
					{
						id: 5002,
						parent: 501,
						title:
							"Создать менеджер для сохранения каналов связи со всеми активными окнами виджета.",
						status: "pending",
						description: "",
					},
				],
			},
			{
				id: 502,
				parent: 5,
				title: "Реализовать бродкаст и связать с фронтендом",
				description: "",
				status: "pending",
				subtasks: [
					{
						id: 5003,
						parent: 502,
						title:
							"Интегрировать отправку сообщений в сокеты при каждом успешном вызове REST API.",
						status: "pending",
						description: "",
					},
					{
						id: 5004,
						parent: 502,
						title:
							"Во фронтенде написать функцию начальной загрузки данных через fetch при старте.",
						status: "pending",
						description: "",
					},
					{
						id: 5005,
						parent: 502,
						title:
							"Реализовать постоянное WS-подключение на фронтенде с логикой переподключения (reconnect).",
						status: "pending",
						description: "",
					},
				],
			},
		],
	},
	{
		id: 6,
		title: "Этап 6: CLI Утилита и Проверка",
		status: "pending",
		tasks: [
			{
				id: 601,
				parent: 6,
				title: "Создать CLI приложение",
				description: "",
				status: "pending",
				subtasks: [
					{
						id: 6001,
						parent: 601,
						title:
							"Добавить второй бинарник в воркспейс и подключить библиотеку парсинга ввода.",
						status: "pending",
						description: "",
					},
					{
						id: 6002,
						parent: 601,
						title: "Описать дерево команд (widget todo add, widget todo done).",
						status: "pending",
						description: "",
					},
				],
			},
			{
				id: 602,
				parent: 6,
				title: "Настроить отправку команд и сквозной тест",
				description: "",
				status: "pending",
				subtasks: [
					{
						id: 6003,
						parent: 602,
						title:
							"Внедрить HTTP-клиент в CLI для отправки сформированных JSON-структур на сервер.",
						status: "pending",
						description: "",
					},
					{
						id: 6004,
						parent: 602,
						title:
							"Провести финальный тест: команда из консоли -> изменение в БД -> сокет -> обновление в OBS.",
						status: "pending",
						description: "",
					},
				],
			},
		],
	},
]);
