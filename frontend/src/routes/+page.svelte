<script lang="ts">
	import Stage from "$cmp/Stage.svelte";
	import { stages } from "$lib/fixtures";
</script>

{#each stages as stage (stage.id)}
	<Stage {stage} />
{/each}
