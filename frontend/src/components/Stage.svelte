<script lang="ts">
	import type { Stage } from "$lib/generated-bindings";
	import Task from "./Task.svelte";

	interface Props {
		stage: Stage;
	}

	let { stage }: Props = $props();
</script>

<div class="stage">
	<div class="title_wrapper">
		<div>{stage.title}</div>
		<div>#{stage.id}</div>
	</div>
	<div class="tasks-list">
		{#each stage.tasks as task (task.id)}
			<Task {task} />
		{/each}
	</div>
</div>
