<script lang="ts">
	import type { Task } from "$lib/generated-bindings";
	import SubTask from "./SubTask.svelte";

	interface Props {
		task: Task;
	}

	let { task }: Props = $props();
</script>

<div class="task">
	<div class="title_wrapper">
		<div>{task.title}</div>
		<div>#{task.id}</div>
	</div>
	<div class="tasks-list">
		{#each task.subtasks as subtask (subtask.id)}
			<SubTask {subtask} />
		{/each}
	</div>
</div>
