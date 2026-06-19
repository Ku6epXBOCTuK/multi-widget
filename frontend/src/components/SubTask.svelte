<script lang="ts">
	import type { SubTask } from "$lib/generated-bindings";

	interface Props {
		subtask: SubTask;
	}

	let { subtask }: Props = $props();

	let isDone = $derived(subtask.status === "done");
</script>

<li class="sub-task-item">
	<label class="todo-label">
		<input type="checkbox" checked={isDone} />
		<span class="checkmark"></span>
		<span class="todo-text">{subtask.title}</span>
	</label>
</li>

<style>
	.sub-task-item {
		display: flex;
		margin-bottom: 6px;
	}

	.sub-task-item:last-child {
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
		width: 12px;
		height: 12px;
		border-radius: 3px;
		margin-right: 10px;
		margin-top: 3px;
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
		left: 3px;
		top: 0px;
		width: 3px;
		height: 6px;
		border: solid white;
		border-width: 0 1.5px 1.5px 0;
		transform: rotate(45deg);
	}

	.todo-label input:checked ~ .checkmark::after {
		display: block;
	}

	.todo-text {
		font-size: 12.5px;
		color: #c9d1d9;
		line-height: 1.4;
		transition: color 0.2s;
	}

	.todo-label input:checked ~ .todo-text {
		color: var(--text-muted);
		text-decoration: line-through;
	}
</style>
