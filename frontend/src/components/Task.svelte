<script lang="ts">
	import type { Activity } from "$lib/generated-types";
	import SubTask from "./SubTask.svelte";

	interface Props {
		task: Activity;
		activities: Activity[];
	}

	let { task, activities }: Props = $props();

	let isDone = $derived(task.status === "done");
	let subtasks = $derived(activities.filter((a) => a.parent_id === task.id));
	let hasSubtasks = $derived(subtasks.length > 0);
</script>

<div class="task-group">
	<li class="parent-task">
		<label class="todo-label">
			<input type="checkbox" checked={isDone} />
			<span class="checkmark"></span>
			<span class="todo-text">{task.title}</span>
		</label>
	</li>
	{#if hasSubtasks}
		<ul class="sub-tasks-list">
			{#each subtasks as subtask (subtask.id)}
				<SubTask {subtask} />
			{/each}
		</ul>
	{/if}
</div>

<style>
	.task-group {
		margin-bottom: 16px;
	}

	.task-group:last-child {
		margin-bottom: 0;
	}

	.todo-label {
		display: flex;
		align-items: flex-start;
		cursor: pointer;
		width: 100%;
	}

	.todo-label input[type="checkbox"] {
		position: absolute;
		opacity: 0;
		height: 0;
		width: 0;
	}

	.checkmark {
		border: 1px solid var(--text-muted);
		width: 16px;
		height: 16px;
		border-radius: 4px;
		margin-right: 12px;
		margin-top: 2px;
		flex-shrink: 0;
		position: relative;
		transition: all 0.2s;
		background: rgba(255, 255, 255, 0.02);
	}

	.todo-label input:checked ~ .checkmark {
		background-color: var(--done-color);
		border-color: var(--done-color);
	}

	.checkmark::after {
		content: "";
		position: absolute;
		display: none;
		left: 5px;
		top: 1px;
		width: 4px;
		height: 8px;
		border: solid white;
		border-width: 0 2px 2px 0;
		transform: rotate(45deg);
	}

	.todo-label input:checked ~ .checkmark::after {
		display: block;
	}

	.todo-text {
		font-size: 14px;
		font-weight: 600;
		color: var(--text-main);
		line-height: 1.4;
		transition: color 0.2s;
	}

	.todo-label input:checked ~ .todo-text {
		color: var(--text-muted);
		text-decoration: line-through;
	}

	.sub-tasks-list {
		list-style: none;
		padding-left: 28px;
		border-left: 1px dashed rgba(255, 255, 255, 0.1);
		margin-left: 7px;
	}
</style>
