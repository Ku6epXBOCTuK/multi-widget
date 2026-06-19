<script lang="ts">
	import Stage from "$cmp/Stage.svelte";
	import { stageStore } from "$lib/stores/stages.svelte";
</script>

<div class="widget-container">
	<header>
		<h1>Todo Roadmap</h1>
		<div class="subtitle">SvelteKit + Rust Axum + CLI</div>
	</header>

	<div class="content-scroll">
		{#each stageStore.all as stage (stage.id)}
			<Stage {stage} />
		{/each}
	</div>
</div>

<style>
	.widget-container {
		width: 100%;
		height: 100%;
		background: var(--bg-color);
		border: 2px solid var(--accent-color);
		border-radius: 12px;
		box-shadow: 0 0 20px var(--accent-glow);
		padding: 24px;
		display: flex;
		flex-direction: column;
		backdrop-filter: blur(8px);
	}

	header {
		margin-bottom: 15px;
		border-bottom: 1px solid var(--border-color);
		padding-bottom: 12px;
		flex-shrink: 0;
	}

	h1 {
		font-size: 20px;
		text-transform: uppercase;
		letter-spacing: 2px;
		color: var(--accent-color);
		text-shadow: 0 0 8px var(--accent-glow);
		margin-bottom: 4px;
	}

	.subtitle {
		font-size: 12px;
		color: var(--text-muted);
	}

	.content-scroll {
		flex-grow: 1;
		overflow-y: auto;
		padding-right: 5px;
	}

	.content-scroll::-webkit-scrollbar {
		width: 4px;
	}

	.content-scroll::-webkit-scrollbar-track {
		background: transparent;
	}

	.content-scroll::-webkit-scrollbar-thumb {
		background: var(--border-color);
		border-radius: 4px;
	}
</style>
