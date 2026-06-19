<script lang="ts">
	import type { Task, TaskStatus } from "$lib/generated-bindings";
	import SubTask from "./SubTask.svelte";

	interface Props {
		task: Task;
	}

	let { task }: Props = $props();

	const statusLabels: Record<TaskStatus, string> = {
		pending: "Ожидание",
		"in-progress": "В процессе",
		done: "Завершено",
	};

	const statusTextClass: Record<TaskStatus, string> = {
		pending: "status-label-pending",
		"in-progress": "shimmer-text",
		done: "status-label-done",
	};

	let statusLabel = $derived(statusLabels[task.status] ?? task.status);
	let statusClass = $derived(statusTextClass[task.status] ?? "");
	let isDone = $derived(task.status === "done");
	let isInProgress = $derived(task.status === "in-progress");
</script>

<div
	class="task-item"
	class:is-in-progress={isInProgress}
	class:is-done={isDone}
>
	<div class="task-content">
		<div class="task-header">
			<span class="status-dot {task.status}"></span>
			<h3 class="task-title" class:is-done={isDone}>{task.title}</h3>
		</div>
		{#if task.description}
			<p class="task-description">{task.description}</p>
		{/if}
		{#if task.subtasks.length > 0}
			<div class="subtask-tree">
				{#each task.subtasks as subtask (subtask.id)}
					<SubTask {subtask} />
				{/each}
			</div>
		{/if}
		<div class="task-status status-label {statusClass}">{statusLabel}</div>
	</div>
</div>

<style>
	.task-item {
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		border-left: 2px solid transparent;
		transition: all 0.3s ease;
		background: rgba(15, 20, 15, 0.4);
		animation: fadeIn 0.4s ease-out;
	}

	@media (min-width: 768px) {
		.task-item {
			flex-direction: row;
			align-items: flex-start;
			justify-content: space-between;
		}
	}

	.task-item.is-done {
		opacity: 0.6;
	}

	.task-item.is-in-progress {
		border-left: 2px solid var(--amber);
		background: linear-gradient(
			90deg,
			rgba(251, 191, 36, 0.08) 0%,
			rgba(15, 20, 15, 0.4) 100%
		);
		animation: border-glow 2.5s ease-in-out infinite;
	}

	.task-content {
		flex: 1;
		min-width: 0;
	}

	.task-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.5rem;
	}

	.task-title {
		font-size: 16px;
		font-weight: 500;
		color: var(--text-main);
	}

	.task-title.is-done {
		text-decoration: line-through;
		color: var(--text-dim);
	}

	.task-description {
		font-size: 0.875rem;
		color: var(--text-dim);
		margin-left: 34px;
		line-height: 1.625;
		margin-bottom: 0.25rem;
	}

	.task-status {
		margin-left: 34px;
		margin-top: 1rem;
	}

	.status-dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		display: inline-block;
		flex-shrink: 0;
		position: relative;

		&.pending {
			background-color: var(--text-dim);
		}

		&.done {
			background-color: var(--accent-green);
			box-shadow: 0 0 10px rgba(52, 211, 153, 0.6);
		}

		&.in-progress {
			background-color: var(--amber);
			box-shadow: 0 0 10px var(--amber-glow);
			z-index: 2;
		}

		&.in-progress::before {
			content: "";
			position: absolute;
			top: 50%;
			left: 50%;
			transform: translate(-50%, -50%);
			width: 100%;
			height: 100%;
			border-radius: 50%;
			border: 2px solid var(--amber);
			animation: radar-ping 1.5s cubic-bezier(0, 0, 0.2, 1) infinite;
		}

		&.in-progress::after {
			content: "";
			position: absolute;
			top: 50%;
			left: 50%;
			transform: translate(-50%, -50%);
			width: 100%;
			height: 100%;
			border-radius: 50%;
			border: 1px solid var(--amber);
			animation: radar-ping 1.5s cubic-bezier(0, 0, 0.2, 1) infinite 0.4s;
		}
	}

	.status-label {
		font-family: "JetBrains Mono", monospace;
		font-size: 10px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-weight: 700;
	}

	.status-label-pending {
		color: var(--text-dim);
	}

	.status-label-done {
		color: var(--accent-green);
	}

	.shimmer-text {
		background: linear-gradient(
			90deg,
			#d97706 0%,
			#fcd34d 40%,
			#ffffff 50%,
			#fcd34d 60%,
			#d97706 100%
		);
		background-size: 200% auto;
		background-clip: text;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		animation: shimmer 2.5s linear infinite;
		display: inline-flex;
		align-items: baseline;
	}

	.subtask-tree {
		position: relative;
		padding-left: 16px;
		margin-left: 14px;
		border-left: 1px dashed #2a3530;
		margin-top: 1rem;
	}

	::global(.subtask-tree > * + *) {
		margin-top: 0.75rem;
	}

	.subtask-tree::before {
		content: "";
		position: absolute;
		left: -1px;
		top: 8px;
		width: 12px;
		height: 1px;
		background: #2a3530;
	}

	@keyframes radar-ping {
		75%,
		100% {
			transform: translate(-50%, -50%) scale(3.5);
			opacity: 0;
		}
		0% {
			transform: translate(-50%, -50%) scale(1);
			opacity: 0.8;
		}
	}
</style>
