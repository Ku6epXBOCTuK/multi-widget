<script lang="ts">
	import type { Activity } from "$lib/generated-types";
	import Task from "./Task.svelte";

	interface Props {
		stage: Activity;
		activities: Activity[];
	}

	let { stage, activities }: Props = $props();

	let isDone = $derived(stage.status === "done");
	let tasks = $derived(activities.filter((a) => a.parent_id === stage.id));
</script>

<details open={!isDone}>
	<summary>{stage.title}</summary>
	<ul class="todo-tree">
		{#each tasks as task (task.id)}
			<Task {task} {activities} />
		{/each}
	</ul>
</details>

<style>
	details {
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid var(--border-color);
		border-radius: 8px;
		margin-bottom: 12px;
		overflow: hidden;
	}

	summary {
		padding: 12px;
		font-size: 13px;
		font-weight: bold;
		color: var(--accent-color);
		cursor: pointer;
		list-style: none;
		display: flex;
		align-items: center;
		text-transform: uppercase;
		letter-spacing: 1px;
		background: rgba(0, 242, 254, 0.02);
	}

	summary::-webkit-details-marker {
		display: none;
	}

	summary::before {
		content: "▶";
		display: inline-block;
		margin-right: 10px;
		font-size: 10px;
		transition: transform 0.2s;
	}

	details[open] summary::before {
		transform: rotate(90deg);
	}

	.todo-tree {
		padding: 10px 14px;
		list-style: none;
	}
</style>
